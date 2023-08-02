use reqwest;
use std::env::current_dir;
use std::fs::File;
use std::io;
use std::io::Error;
use std::os::unix::prelude::PermissionsExt;
use tempfile;
use zip;

fn main() -> Result<(), Error> {
    download_ffmpeg()?;
    tauri_build::build();
    Ok(())
}

fn download_ffmpeg() -> Result<(), Error> {
    let target_triple = std::env::var("TARGET").unwrap();
    let ffmpeg_binary_name = if target_triple.contains("windows") {
        format!("ffmpeg-{}.exe", target_triple)
    } else {
        format!("ffmpeg-{}", target_triple)
    };
    let target = current_dir().unwrap().join("bin").join(ffmpeg_binary_name);
    if !target.is_file() {
        let mut tmpfile = tempfile::tempfile().unwrap();

        let url = if target_triple.contains("windows") {
            "https://github.com/ffbinaries/ffbinaries-prebuilt/releases/download/v4.4.1/ffmpeg-4.4.1-win-64.zip"
        } else if target_triple.contains("apple") {
            "https://github.com/ffbinaries/ffbinaries-prebuilt/releases/download/v4.4.1/ffmpeg-4.4.1-osx-64.zip"
        } else {
            "https://github.com/ffbinaries/ffbinaries-prebuilt/releases/download/v4.4.1/ffmpeg-4.4.1-linux-64.zip"
        };

        let _ = reqwest::blocking::get(url).unwrap().copy_to(&mut tmpfile);

        let mut archive = zip::ZipArchive::new(tmpfile).unwrap();
        let ffmpeg_zip_file = if target_triple.contains("windows") {
            "ffmpeg.exe"
        } else {
            "ffmpeg"
        };
        let mut file = archive.by_name(ffmpeg_zip_file).unwrap();
        let mut dest = File::create(&target).unwrap();
        io::copy(&mut file, &mut dest)?;
    }
    let mut perms = std::fs::metadata(&target)?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&target, perms)?;
    Ok(())
}
