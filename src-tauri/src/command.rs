use crate::utils::read_frame_info;
use crate::skip_fail;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tauri::State;
use crate::models::{FrameInfo, App, Setting};

#[derive(Default)]
pub struct AppState(pub Arc<Mutex<App>>);

pub type AppArg<'a> = State<'a, AppState>;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
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
pub fn get_config(app: AppArg<'_>) -> Option<Setting> {
    let app = app.0.lock().unwrap();
    app.config.as_ref().map(|r| r.clone())
}

#[tauri::command]
pub fn set_config(app: AppArg<'_>, config: Setting) {
    let mut app = app.0.lock().unwrap();
    app.config = Some(config)
}

#[tauri::command]
pub fn load_root_dir(app: AppArg<'_>, root_dir: &str) -> Result<(), Error> {
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
    app.current_index = -1;
    app.timestamps.sort();
    Ok(())
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

    return read_frame_info(curtimestamp, &timestamp_dir);
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

    return read_frame_info(curtimestamp, &timestamp_dir);
}
