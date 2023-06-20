use serde::Serialize;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tauri::State;
use base64::{Engine as _, engine::general_purpose};

#[derive(Default, Serialize)]
pub struct App {
    pub root_dir: String,
    pub current_index: i32,
    pub timestamps: Vec<i64>,
}

#[derive(Serialize)]
pub struct FrameInfo {
    pub timestamp: i64,
    pub image_data: String,
    pub targets: Vec<Target>,
}

#[derive(Serialize)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Serialize)]
pub struct Target {
    pub track_id: i64,
    pub label: i64,
    pub roi: Rect,
    pub selected: bool,
    pub annotations: Vec<String>,
}

#[derive(Default)]
pub struct AppState(pub Arc<Mutex<App>>);

pub type AppArg<'a> = State<'a, AppState>;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn load_root_dir(app: AppArg<'_>, root_dir: &str) {
    let root_path = Path::new(root_dir);

    let mut app = app.0.lock().unwrap();

    app.root_dir = String::from(root_dir);
    app.timestamps.clear();

    for entry in fs::read_dir(root_path).unwrap() {
        let entry = entry.unwrap();

        if entry.metadata().unwrap().is_file() {
            continue;
        }

        let name = entry.file_name().into_string().unwrap();
        if let Ok(i) = name.parse::<i64>() {
            let frame_path = entry.path().join("frame.jpg");
            if !frame_path.exists() {
                continue;
            }
            app.timestamps.push(i);
        }
    }
    app.current_index = -1;
    app.timestamps.sort();
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

    let frame_path = Path::new(&app.root_dir).join(format!("{curtimestamp}")).join("frame.jpg");

    let content = fs::read(frame_path).unwrap();

    Some(FrameInfo {
        timestamp: curtimestamp,
        image_data: general_purpose::STANDARD_NO_PAD.encode(content),
        targets: Vec::new(),
    })
}
