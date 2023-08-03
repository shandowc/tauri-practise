use ffmpeg_sidecar::command::FfmpegCommand;
use ffmpeg_sidecar::error::Error as FfmpegError;
use ffmpeg_sidecar::event::OutputVideoFrame;
use image::{ImageBuffer, Rgb};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use tauri::{AppHandle, Manager, Window};
use serde::{Deserialize, Serialize};
// use std::time::Instant;

pub struct LoadVideo {
    pub root_dir: String,
    pub timestamps: Vec<i64>,
    pub video_path: String,
    pub force: bool,
}

pub enum Request {
    LoadVideo(LoadVideo),
    StopLoad(),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FFmpegProgress {
    video_path: String,
    frame_cnt: i32,
    done: bool,
}

fn get_frame_iter(video_path: &str) -> Result<impl Iterator<Item = OutputVideoFrame>, FfmpegError> {
    Ok(FfmpegCommand::new()
        .input(video_path)
        .rawvideo()
        .create_no_window()
        .spawn()?
        .iter()?
        .filter_frames())
}

fn process_msg(window: &Window, msg: Request, rx: &Receiver<Request>) -> Option<Request> {
    match msg {
        Request::LoadVideo(info) => {
            let frame_iter = get_frame_iter(&info.video_path);
            if let Err(err) = frame_iter {
                log::error!("{}", err);
                _ = window.emit("background_error", format!("ffmpeg: {}", err));
                return None;
            }
            let mut frame_iter = frame_iter.unwrap().peekable();
            let mut ts_iter = info.timestamps.iter().peekable();
            let mut processed_cnt = 0;
            loop {
                if frame_iter.peek().is_none() || ts_iter.peek().is_none() {
                    _ = window.emit("video_progress", FFmpegProgress {
                        video_path: info.video_path.clone(),
                        frame_cnt: processed_cnt,
                        done: true,
                    });
                    return None;
                }
                if let Ok(new_msg) = rx.try_recv() {
                    return Some(new_msg);
                }
                let ts = ts_iter.peek().unwrap();
                let pts = (frame_iter.peek().unwrap().timestamp * 1000f32) as i64;
                if pts == **ts {
                    processed_cnt += 1;
                    _ = window.emit("video_progress", FFmpegProgress {
                        video_path: info.video_path.clone(),
                        frame_cnt: processed_cnt,
                        done: false,
                    });

                    let frame = frame_iter.next().unwrap();
                    let ts = ts_iter.next().unwrap();

                    let image_dest = PathBuf::from(&info.root_dir)
                        .join(format!("{}", ts))
                        .join("frame_aux.jpg");
                    let image_dest_default = PathBuf::from(&info.root_dir)
                        .join(format!("{}", ts))
                        .join("frame.jpg");

                    if !info.force && (image_dest_default.exists() || image_dest.exists()) {
                        continue;
                    }

                    let image: Option<ImageBuffer<Rgb<u8>, Vec<u8>>> =
                        ImageBuffer::from_raw(frame.width, frame.height, frame.data);
                    if image.is_some() {
                        let image = image.unwrap();

                        // let start = Instant::now();
                        if let Err(err) = image.save(image_dest) {
                            log::warn!("{}", err);
                        }
                        // let duration = start.elapsed();
                        // println!("Time elapsed in expensive_function() is: {:?}", duration);
                    }
                } else if pts < **ts {
                    frame_iter.next();
                } else {
                    processed_cnt += 1;
                    _ = window.emit("video_progress", FFmpegProgress {
                        video_path: info.video_path.clone(),
                        frame_cnt: processed_cnt,
                        done: false,
                    });
                    ts_iter.next();
                }
            }
        }
        _ => {
            return None;
        }
    }
}

pub fn do_ffmpeg_background(handle: AppHandle, rx: Receiver<Request>) {
    let window = handle.get_window("main").unwrap();
    loop {
        let mut msg = rx.recv().unwrap();
        loop {
            let next_msg = process_msg(&window, msg, &rx);
            if next_msg.is_none() {
                break;
            }
            msg = next_msg.unwrap();
        }
    }
}
