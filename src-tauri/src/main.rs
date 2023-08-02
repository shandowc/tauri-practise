// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod command;
mod macros;
mod models;
mod utils;
mod background;

use std::env;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};

use std::thread;
use std::sync;
use models::App;
use tauri::Manager;
use background::Request;


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
        .setup(|app| {
            let (tx, rx): (Sender<background::Request>, Receiver<Request>) =
                sync::mpsc::channel();
            thread::spawn(move || background::do_ffmpeg_background(rx));
            let state = models::AppState(Arc::new(
                Mutex::new(App{
                    root_dir: String::new(),
                    current_index: 0,
                    timestamps: Vec::new(),
                    config: None,
                    ffmpeg_tx: tx,
                })
            ));
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::validata_path,
            command::current_frame_info,
            command::greet,
            command::set_config,
            command::load_root_dir,
            command::goto_frame_idx,
            command::quit_ffmpeg_process,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
