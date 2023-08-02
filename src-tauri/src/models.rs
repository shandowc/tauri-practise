use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::State;
use std::sync;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ParseError(#[from] serde_json_path::ParseError),
    #[error(transparent)]
    FfmpegError(#[from] ffmpeg_sidecar::error::Error),
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
}


#[derive(Default)]
pub struct AppState(pub Arc<Mutex<App>>);

pub type AppArg<'a> = State<'a, AppState>;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Setting {
    pub annotations: Vec<AnnotationConfig>,
}

#[derive(Debug, Default, Serialize)]
pub struct App {
    pub root_dir: String,
    pub current_index: i32,
    pub timestamps: Vec<i64>,
    pub config: Option<Setting>,
}

#[derive(Debug, Serialize)]
pub struct VideoSummary {
    pub frame_cnt: i32,
}

#[derive(Debug, Serialize)]
pub struct FrameInfo {
    pub frame_idx: i32,
    pub timestamp: i64,
    pub image_path: String,
    pub targets: Vec<Target>,
    pub jsons: HashMap<String, Vec<String>>,
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
