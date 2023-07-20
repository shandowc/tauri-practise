use crate::models::{AppArg, FrameInfo, Setting, VideoSummary};
use crate::skip_fail;
use crate::utils::read_frame_info;
use serde_json_path::JsonPath;
use std::fs;
use std::path::Path;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ParseError(#[from] serde_json_path::ParseError),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
pub fn validata_path(s: &str) -> Result<(), Error> {
    return JsonPath::parse(s)
        .map(|_| ())
        .map_err(|e| Error::ParseError(e));
}

#[tauri::command]
pub fn set_config(app: AppArg<'_>, config: Setting) -> Result<(), Error> {
    for rule in config.annotations.iter() {
        validata_path(&rule.value_path)?
    }
    let mut app = app.0.lock().unwrap();
    app.config = Some(config);
    Ok(())
}

#[tauri::command]
pub fn load_root_dir(app: AppArg<'_>, root_dir: &str) -> Result<VideoSummary, Error> {
    let root_path = Path::new(root_dir);

    let mut app = app.0.lock().unwrap();

    app.root_dir = String::from(root_dir);
    app.timestamps.clear();

    for entry in fs::read_dir(root_path)? {
        let entry = skip_fail!(entry);
        let meta = skip_fail!(entry.metadata());
        if meta.is_file() {
            continue;
        }
        if let Ok(name) = entry.file_name().into_string() {
            if let Ok(i) = name.parse::<i64>() {
                let frame_path = entry.path().join("frame.jpg");
                if !frame_path.exists() {
                    continue;
                }
                app.timestamps.push(i);
            }
        }
    }
    app.current_index = 0;
    app.timestamps.sort();
    Ok(VideoSummary {
        frame_cnt: app.timestamps.len() as i32,
    })
}

#[tauri::command]
pub fn next_frame_info(app: AppArg<'_>) -> Option<FrameInfo> {
    let mut app = app.0.lock().unwrap();
    let mut curridx = app.current_index;
    let pts_len = app.timestamps.len() as i32;

    curridx = curridx + 1;
    if curridx > pts_len {
        return None;
    }
    let curtimestamp = *app.timestamps.get(curridx as usize)?;
    app.current_index = curridx;

    let timestamp_dir = Path::new(&app.root_dir).join(format!("{curtimestamp}"));

    return read_frame_info(
        curridx,
        app.config.as_ref().unwrap(),
        curtimestamp,
        &timestamp_dir,
    );
}

#[tauri::command]
pub fn previous_frame_info(app: AppArg<'_>) -> Option<FrameInfo> {
    let mut app = app.0.lock().unwrap();
    let mut curridx = app.current_index;

    curridx = curridx - 1;
    if curridx < 0 {
        return None;
    }
    let curtimestamp = *app.timestamps.get(curridx as usize)?;
    app.current_index = curridx;

    let timestamp_dir = Path::new(&app.root_dir).join(format!("{curtimestamp}"));

    return read_frame_info(
        curridx,
        app.config.as_ref().unwrap(),
        curtimestamp,
        &timestamp_dir,
    );
}

#[tauri::command]
pub fn current_frame_info(app: AppArg<'_>) -> Option<FrameInfo> {
    let app = app.0.lock().unwrap();
    let curtimestamp = *app.timestamps.get(app.current_index as usize)?;
    let timestamp_dir = Path::new(&app.root_dir).join(format!("{curtimestamp}"));
    return read_frame_info(
        app.current_index,
        app.config.as_ref().unwrap(),
        curtimestamp,
        &timestamp_dir,
    );
}

#[tauri::command]
pub fn goto_frame_idx(app: AppArg<'_>, frame_idx: i32) -> Option<FrameInfo> {
    let mut app = app.0.lock().unwrap();
    if app.timestamps.len() > frame_idx as usize {
        app.current_index = frame_idx;
    }
    let curtimestamp = *app.timestamps.get(app.current_index as usize)?;
    let timestamp_dir = Path::new(&app.root_dir).join(format!("{curtimestamp}"));
    return read_frame_info(
        app.current_index,
        app.config.as_ref().unwrap(),
        curtimestamp,
        &timestamp_dir,
    );
}
