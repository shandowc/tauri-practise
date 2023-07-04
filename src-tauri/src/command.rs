use crate::utils::rfind_utf8;
use crate::{skip_fail, skip_none};
use base64::{engine::general_purpose, Engine as _};
use jsonpath_rust::JsonPathQuery;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tauri::State;
use urlencoding;

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

#[derive(Serialize, Deserialize)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Serialize, Deserialize)]
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

    for entry in fs::read_dir(&timestamp_dir).ok()? {
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

        log::info!("{module}");
        println!("{module}\n");

        let s = skip_fail!(fs::read_to_string(entry.path()));
        let json: Value = skip_fail!(serde_json::from_str(&s));

        let o = json.path("$.roi.['$binary'].['$readable']").unwrap();
        match o.as_array() {
            None => continue,
            Some(arr) => {
                if arr.len() == 0 {
                    continue;
                }
                match serde_json::from_value::<Rect>(o.get(0).unwrap().to_owned()) {
                    Ok(rect) => {
                        let width = rect.width;
                        print!("{width}\n");
                    }
                    Err(err) => print!("{err}\n"),
                }
            }
        }

        // let pre = o.to_string();
        // print!("{pre}\n");

        // match serde_json::from_value::<Rect>(o) {
        //     Ok(rect) => {
        //         let width = rect.width;
        //         print!("{width}\n");
        //     },
        //     Err(err) => print!("{err}\n")
        // }
    }

    let frame_path = timestamp_dir.join("frame.jpg");

    let content = fs::read(frame_path).unwrap_or_else(|e| {
        log::warn!("{:?}", e);
        Vec::new()
    });

    Some(FrameInfo {
        timestamp: curtimestamp,
        image_data: general_purpose::STANDARD_NO_PAD.encode(content),
        targets: Vec::new(),
    })
}
