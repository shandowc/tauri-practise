use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Setting {
    annotations: Vec<AnnotationConfig>,
}

#[derive(Default, Serialize)]
pub struct App {
    pub root_dir: String,
    pub current_index: i32,
    pub timestamps: Vec<i64>,
    pub config: Option<Setting>,
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
    pub roi: Option<Rect>,
    pub selected: bool,
    pub annotations: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AnnotationConfig {
    inspoint: String,
    key: String,
    value_path: String,
}

