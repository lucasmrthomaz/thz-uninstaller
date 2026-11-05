use std::env;
use std::fs;
use std::path::PathBuf;

use eframe::egui::{self, FontData, FontDefinitions, FontFamily};

pub fn configure_windows_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    let windir = env::var("WINDIR")
        .or_else(|_| env::var("SystemRoot"))
        .unwrap_or_else(|_| "C:\\Windows".to_string());

    let fonts_dir = PathBuf::from(windir).join("Fonts");
    let segoe_regular = fonts_dir.join("segoeui.ttf");
    let segoe_bold = fonts_dir.join("segoeuib.ttf");

    if let Ok(regular) = fs::read(&segoe_regular) {
        fonts
            .font_data
            .insert("segoeui".to_owned(), FontData::from_owned(regular).into());
        if let Some(vec) = fonts.families.get_mut(&FontFamily::Proportional) {
            vec.insert(0, "segoeui".to_owned());
        }
    }
    if let Ok(bold) = fs::read(&segoe_bold) {
        fonts
            .font_data
            .insert("segoeuib".to_owned(), FontData::from_owned(bold).into());
        fonts
            .families
            .insert(FontFamily::Name("bold".into()), vec!["segoeuib".to_owned()]);
    } else if let Some(prop) = fonts.families.get(&FontFamily::Proportional).cloned() {
        fonts.families.insert(FontFamily::Name("bold".into()), prop);
    }
    ctx.set_fonts(fonts);
}

