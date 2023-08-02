use ffmpeg_sidecar::command::FfmpegCommand;
use ffmpeg_sidecar::event::OutputVideoFrame;
use std::sync::mpsc::Receiver;
use image::{ImageBuffer, Rgb};
use std::time::Instant;

pub struct LoadVideo {
    root_dir: String,
    timestamps: Vec<i64>,
    video_path: String,
}

pub enum Request {
    LoadVideo(LoadVideo),
    StopLoad(),
}

pub fn do_ffmpeg_background(rx: Receiver<Request>) {
    loop {
        let mut msg = rx.recv().unwrap();
        'outer: {
            match rx.recv().unwrap() {
                background::LoadVideo(msg) => {
                    let mut frame_iter = FfmpegCommand::new()
                        .input(msg.root_dir)
                        .rawvideo()
                        .spawn()?
                        .iter()?
                        .filter_frames()
                        .peekable();
                    for ts in timestamps {
                        while frame_iter.peek().is_some() {
                            let frame = frame_iter.next().unwrap();
                            let pts = (frame.timestamp * 1000.0) as i64;
                            if pts == *ts {
                                let image: Option<ImageBuffer<Rgb<u8>, Vec<u8>>> =
                                    ImageBuffer::from_raw(frame.width, frame.height, frame.data);
                                if image.is_some() {
                                    let image_dest =
                                        root_path.join(format!("{}", ts)).join("frame_aux.jpg");
                                    let image = image.unwrap();

                                    let start = Instant::now();
                                    image.save(image_dest)?;
                                    let duration = start.elapsed();
                                    println!(
                                        "Time elapsed in expensive_function() is: {:?}",
                                        duration
                                    );
                                }
                                break;
                            }
                            if pts > *ts {
                                break;
                            }
                        }
                        if frame_iter.peek().is_none() {
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
