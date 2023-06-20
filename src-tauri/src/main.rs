// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod command;

fn main() {
    tauri::Builder::default()
        .manage(command::AppState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            command::greet,
            command::load_root_dir,
            command::next_frame_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
