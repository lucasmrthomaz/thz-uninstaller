use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use eframe::egui;

pub fn save_color_image_to_png(color_image: &egui::ColorImage, path: &Path) {
    let width = color_image.width() as u32;
    let height = color_image.height() as u32;
    let mut rgba = Vec::with_capacity((width * height * 4) as usize);
    for px in &color_image.pixels {
        rgba.extend_from_slice(&px.to_array());
    }
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            let _ = fs::create_dir_all(parent);
        }
    }
    if let Some(buf) = image::RgbaImage::from_raw(width, height, rgba) {
        let _ = buf.save(path);
    }
}

pub fn parse_screenshot_arg() -> Option<PathBuf> {
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--screenshot" {
            if let Some(val) = args.get(i + 1) {
                return Some(PathBuf::from(val));
            }
        } else if let Some(rest) = args[i].strip_prefix("--screenshot=") {
            return Some(PathBuf::from(rest));
        }
        i += 1;
    }
    None
}

