use crate::models::{FrameInfo, Rect, Setting, Target};
use crate::{skip_fail, skip_none};
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
            return path
                .query(json)
                .exactly_one()
                .ok()
                .map(|v| format!("{}: {}", rule.key, v.to_string()));
        })
        .collect()
}

pub fn read_timestamp_dir(timestamp_dir: &Path) -> Vec<(String, String, Value)> {
    let dir_result = fs::read_dir(&timestamp_dir).map_err(|e| {
        log::error!("{:?}", e);
        e
    });
    let mut res = Vec::new();
    if dir_result.is_err() {
        return res;
    }
    for entry in dir_result.unwrap() {
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
        let s = skip_fail!(fs::read_to_string(entry.path()));
        let json: Value = skip_fail!(serde_json::from_str(&s));
        res.push((module, s, json));
    }
    return res;
}

pub fn read_frame_info(
    frame_idx: i32,
    cfg: &Setting,
    timestamp: i64,
    timestamp_dir: &Path,
) -> Option<FrameInfo> {
    let mut targets = Vec::new();

    let infos = read_timestamp_dir(timestamp_dir);

    // initial track maps
    let mut tracked_targets: HashMap<i64, Target> = HashMap::new();
    let mut tracks: HashMap<Rect, i64> = HashMap::new();
    for (module, content, jvalue) in infos.iter() {
        if module != "flock:face_track_module" && module != "flock:struct_track_module" {
            continue;
        }
        let track_id = jvalue["track_id"].as_i64();
        if track_id.is_none() {
            log::warn!("track has no track_id: {}", content);
            continue;
        }

        let jroi = &jvalue["roi"]["$binary"]["$readable"];
        let roi = Rect::deserialize(jroi).ok();
        if roi.is_none() {
            log::warn!("track has no roi: {}", content);
            continue;
        }
        let roi = roi.unwrap();
        let track_id = track_id.unwrap();

        tracks.insert(roi.clone(), track_id);

        let overlay_strings: Vec<String> = get_overlay_strings(cfg, &module, jvalue);
        tracked_targets.insert(
            track_id,
            Target {
                track_id,
                label: jvalue["label"].as_i64().unwrap(),
                roi,
                selected: false,
                annotations: overlay_strings,
            },
        );
    }

    // assign detects track_ids
    for (module, content, jvalue) in infos.iter() {
        if module != "flock:detect_module" {
            continue;
        }
        let jroi = &jvalue["roi"]["$binary"]["$readable"];
        let roi = Rect::deserialize(jroi).ok();
        if roi.is_none() {
            log::warn!("detect has no roi: {}", content);
            continue;
        }
        let roi = roi.unwrap();

        let track_id = tracks.get(&roi);
        if track_id.is_none() {
            log::warn!("detect has no related track_id: {}", content);
            continue;
        }
        let track_id = track_id.unwrap();

        let target = tracked_targets.get_mut(track_id);
        if target.is_none() {
            log::warn!("detect has no matching track: {}", content);
            continue;
        }
        let mut overlay_strings: Vec<String> = get_overlay_strings(cfg, &module, jvalue);
        let target = target.unwrap();
        target.annotations.append(&mut overlay_strings);
    }

    // process other modules, map by track_id
    let mut jsons: HashMap<String, Vec<String>> = HashMap::new();
    for (module, content, jvalue) in infos.iter() {
        if let Some(v) = jsons.get_mut(module) {
            v.push(content.clone());
        } else {
            jsons.insert(module.clone(), vec![content.clone()]);
        }
        if module == "flock:face_track_module"
            || module == "struct_track_module"
            || module == "flock:detect_module"
        {
            continue;
        }
        let track_id = jvalue["track_id"].as_i64();
        if track_id.is_none() {
            continue;
        }
        let track_id = track_id.unwrap();
        let target = tracked_targets.get_mut(&track_id);
        if target.is_none() {
            log::warn!("object has no matching track: {}", content);
            continue;
        }
        let mut overlay_strings: Vec<String> = get_overlay_strings(cfg, &module, jvalue);
        let target = target.unwrap();
        target.annotations.append(&mut overlay_strings);
    }

    for (_, target) in tracked_targets {
        targets.push(target);
    }

    let image_path: String;
    if timestamp_dir.join("frame.jpg").is_file() {
        image_path = String::from(timestamp_dir.join("frame.jpg").to_str()?);
    } else {
        image_path = String::from(timestamp_dir.join("frame_aux.jpg").to_str()?);
    }

    Some(FrameInfo {
        frame_idx,
        timestamp,
        image_path,
        targets,
        jsons,
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use serde_json_path::JsonPath;

    use super::rfind_utf8;

    #[test]
    fn test_rfind_utf8() {
        let pos = rfind_utf8("你好aa", 'a').unwrap();
        assert_eq!(pos, 3)
    }

    #[test]
    fn test_json_path() {
        let value = json!({ "foo": { "bar": ["baz", 42] } });
        let path = JsonPath::parse("$.foo.bar[1]").unwrap();
        let node = path.query(&value).exactly_one().unwrap();
        assert_eq!(node.to_string(), "42");
    }
}
