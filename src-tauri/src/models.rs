use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::State;

#[derive(Default)]
pub struct AppState(pub Arc<Mutex<App>>);

pub type AppArg<'a> = State<'a, AppState>;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Setting {
    pub annotations: Vec<AnnotationConfig>,
}

impl Setting {
    pub fn new() -> Setting {
        return Setting {
            annotations: vec![AnnotationConfig {
                inspoint: String::from("flock:detect_module"),
                key: String::from("detect_confidence"),
                value_path: String::from("$.confidence"),
            }],
        };
    }
}

#[derive(Debug, Default, Serialize)]
pub struct App {
    pub root_dir: String,
    pub current_index: i32,
    pub timestamps: Vec<i64>,
    pub config: Option<Setting>,
}

#[derive(Debug, Serialize)]
pub struct FrameInfo {
    pub timestamp: i64,
    pub image_data: String,
    pub targets: Vec<Target>,
    pub jsons: HashMap<String, Vec<String>>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    pub track_id: i64,
    pub label: i64,
    pub roi: Rect,
    pub selected: bool,
    pub annotations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationConfig {
    pub inspoint: String,
    pub key: String,
    pub value_path: String,
}
