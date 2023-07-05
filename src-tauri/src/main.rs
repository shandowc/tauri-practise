// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod command;
mod utils;
mod macros;
mod models;

use std::io::Write;
use std::env;

fn main() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .parse_env(env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"))
        .init();

    tauri::Builder::default()
        .manage(command::AppState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            command::greet,
            command::get_config,
            command::set_config,
            command::load_root_dir,
            command::next_frame_info,
            command::previous_frame_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
