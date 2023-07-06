use crate::models::{AppArg, FrameInfo, Setting};
use crate::skip_fail;
use crate::utils::read_frame_info;
use std::fs;
use std::path::Path;
use serde_json_path::JsonPath;

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

fn load_config() -> Option<Setting> {
    let home = dirs::home_dir()?;
    let f = home.join("flock_inspect.json");
    let data = fs::read_to_string(f)
        .map_err(|err| {
            log::error!("Failed to read config file: {}", err);
        })
        .ok()?;
    let setting: Setting = serde_json::from_str(&data)
        .map_err(|err| {
            log::error!("Failed to parse config file: {}", err);
        })
        .ok()?;
    Some(setting)
}

fn get_default_config() -> Setting {
    Setting::new()
}

#[tauri::command]
pub fn validata_path(s: &str) -> Result<(), Error> {
    return JsonPath::parse(s).map(|_| ()).map_err(|e| Error::ParseError(e));
}

#[tauri::command]
pub fn get_config(app: AppArg<'_>) -> Option<Setting> {
    let mut app = app.0.lock().unwrap();
    if let Some(cfg) = &app.config {
        return Some(cfg.clone());
    }
    let cfg = load_config().or_else(|| Some(get_default_config()));
    app.config = cfg.clone();
    cfg
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
    app.current_index = 0;
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

    return read_frame_info(app.config.as_ref().unwrap(), curtimestamp, &timestamp_dir);
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

    return read_frame_info(app.config.as_ref().unwrap(), curtimestamp, &timestamp_dir);
}

#[tauri::command]
pub fn current_frame_info(app: AppArg<'_>) -> Option<FrameInfo> {
    let app = app.0.lock().unwrap();
    let curtimestamp = *app.timestamps.get(app.current_index as usize)?;
    let timestamp_dir = Path::new(&app.root_dir).join(format!("{curtimestamp}"));
    return read_frame_info(app.config.as_ref().unwrap(), curtimestamp, &timestamp_dir);
}
