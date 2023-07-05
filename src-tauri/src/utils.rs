use crate::models::{FrameInfo, Rect, Target};
use crate::{skip_fail, skip_none};
use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;
use serde_json::Value;
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

pub fn read_frame_info(timestamp: i64, timestamp_dir: &Path) -> Option<FrameInfo> {
    let mut targets = Vec::new();

    let dir_result = fs::read_dir(&timestamp_dir).map_err(|e| {
        log::error!("{:?}", e);
        e
    });
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
        let json: Value = skip_fail!(serde_json::from_str(&s));

        let a = &json["roi"]["$binary"]["$readable"];
        let rect = Rect::deserialize(a).ok();

        targets.push(Target {
            track_id: 0,
            label: 0,
            roi: rect,
            selected: false,
            annotations: Vec::new(),
        });
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
    })
}

#[cfg(test)]
mod tests {
    use super::{read_frame_info, rfind_utf8};
    use std::path::Path;

    #[test]
    fn test_rfind_utf8() {
        let pos = rfind_utf8("你好aa", 'a').unwrap();
        assert_eq!(pos, 3)
    }

    #[test]
    fn test_read_frame_info() {
        read_frame_info(0, Path::new("")).unwrap();
    }
}
