use crate::background::{Request, LoadVideo};
use crate::models::{AppArg, FrameInfo, Setting, VideoSummary, Error};
use crate::skip_fail;
use crate::utils::read_frame_info;
use serde_json_path::JsonPath;
use std::fs;
use std::path::Path;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
pub fn load_root_dir(
    app: AppArg<'_>,
    root_dir: &str,
    aux_video: Option<&str>,
    aux_video_force: Option<bool>,
) -> Result<VideoSummary, Error> {
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
                app.timestamps.push(i);
            }
        }
    }
    app.current_index = 0;
    app.timestamps.sort();

    if let Some(video) = aux_video {
        if let Err(err) = app.ffmpeg_tx.send(Request::LoadVideo(LoadVideo {
            root_dir: root_dir.to_string(),
            timestamps: app.timestamps.clone(),
            video_path: video.to_string(),
            force: aux_video_force.map_or(false, |v| v),
        })) {
            log::warn!("background task error {:?}", err);
        }
    }
    Ok(VideoSummary {
        frame_cnt: app.timestamps.len() as i32,
    })
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

#[tauri::command]
pub fn quit_ffmpeg_process(app: AppArg<'_>) {
    let app = app.0.lock().unwrap();
    if let Err(err) = app.ffmpeg_tx.send(Request::StopLoad()) {
        log::warn!("background task error {:?}", err);
    }
}
