use crate::models::{FrameInfo, Rect, Setting, Target};
use crate::{skip_fail, skip_none};
use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;
use serde_json::Value;
use serde_json_path::JsonPath;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use urlencoding;

pub fn rfind_utf8(s: &str, chr: char) -> Option<usize> {
    if let Some(rev_pos) = s.chars().rev().position(|c| c == chr) {
        Some(s.chars().count() - rev_pos - 1)
    } else {
        None
    }
}

pub fn get_overlay_strings(cfg: &Setting, module: &str, json: &Value) -> Vec<String> {
    cfg.annotations
            .iter()
            .filter_map(|rule| {
                if rule.inspoint != module {
                    return None;
                }
                let path = JsonPath::parse(&rule.value_path).unwrap();
                return path.query(json).exactly_one().ok().map(|v| format!("{}: {}", rule.key, v.to_string()));
            })
            .collect()
}

pub fn read_frame_info(cfg: &Setting, timestamp: i64, timestamp_dir: &Path) -> Option<FrameInfo> {
    let mut targets = Vec::new();

    let dir_result = fs::read_dir(&timestamp_dir).map_err(|e| {
        log::error!("{:?}", e);
        e
    });

    let mut tracked_targets: HashMap<i64, Target> = HashMap::new();
    let mut detects: HashMap<Rect, Value> = HashMap::new();
    let mut tracks: HashMap<Rect, i64> = HashMap::new();
    let mut jsons: HashMap<String, Vec<String>> = HashMap::new();

    for entry in dir_result.ok()? {
        let entry = skip_fail!(entry);
        let meta = skip_fail!(entry.metadata());
        if !meta.is_file() {
            continue;
        }
        let name = skip_fail!(entry.file_name().into_string());
        if !name.ends_with(".json") {
            continue;
        }
        let idx = skip_none!(rfind_utf8(&name, '-'));
        let module = skip_fail!(urlencoding::decode(&name[..idx])).into_owned();

        // log::info!("{module}");

        let s = skip_fail!(fs::read_to_string(entry.path()));

        if let Some(v) = jsons.get_mut(&module) {
            v.push(s.clone());
        } else {
            jsons.insert(module.clone(), vec![s.clone()]);
        }

        let json: Value = skip_fail!(serde_json::from_str(&s));

        let a = &json["roi"]["$binary"]["$readable"];
        let roi = Rect::deserialize(a).ok();
        let track_id = json["track_id"].as_i64();

        if !roi.is_none() && track_id.is_none() {
            detects.insert(roi.unwrap(), json);
            continue;
        }

        if track_id.is_none() {
            continue;
        }

        let roi = roi.unwrap();
        let track_id = track_id.unwrap();
        tracks.insert(roi.clone(), track_id);

        let target = tracked_targets.get_mut(&track_id);

        let mut overlay_strings: Vec<String> = get_overlay_strings(cfg, &module, &json);

        if !target.is_none() {
            let target = target.unwrap();
            target.annotations.append(&mut overlay_strings);
            continue;
        }

        tracked_targets.insert(
            track_id,
            Target {
                track_id,
                label: json["label"].as_i64().unwrap(),
                roi,
                selected: false,
                annotations: overlay_strings,
            },
        );
    }

    for (roi, v) in detects.into_iter() {
        let track_id = tracks.get(&roi);

        let mut overlay_strings = get_overlay_strings(cfg, "flock:detect_module", &v);

        if track_id.is_none() || !tracked_targets.contains_key(track_id.as_ref().unwrap()) {
            targets.push(Target {
                track_id: -1,
                label: v["label"].as_i64().unwrap(),
                roi,
                selected: false,
                annotations: overlay_strings,
            });
            continue;
        }
        let track_id = track_id.unwrap();
        let target = tracked_targets.get_mut(track_id).unwrap();
        target.annotations.append(&mut overlay_strings);
    }

    for (_, target) in tracked_targets {
        targets.push(target);
    }

    let frame_path = timestamp_dir.join("frame.jpg");

    let content = fs::read(frame_path).unwrap_or_else(|e| {
        log::warn!("{:?}", e);
        Vec::new()
    });

    Some(FrameInfo {
        timestamp,
        image_data: general_purpose::STANDARD_NO_PAD.encode(content),
        targets,
        jsons,
    })
}

#[cfg(test)]
mod tests {
    use crate::models::Setting;
    use serde_json::json;
    use serde_json_path::JsonPath;

    use super::{read_frame_info, rfind_utf8};
    use std::path::Path;

    #[test]
    fn test_rfind_utf8() {
        let pos = rfind_utf8("你好aa", 'a').unwrap();
        assert_eq!(pos, 3)
    }

    #[test]
    fn test_read_frame_info() {
        let cfg = Setting::new();
        let f = read_frame_info(&cfg, 0, Path::new("/data/vps/10002023060605043241601/0")).unwrap();
        print!("{:?}", f);
    }

    #[test]
    fn test_json_path() {
        let value = json!({ "foo": { "bar": ["baz", 42] } });
        let path = JsonPath::parse("$.foo.bar[1]").unwrap();
        let node = path.query(&value).exactly_one().unwrap();
        assert_eq!(node.to_string(), "42");
    }
}
