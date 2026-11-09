use eframe::egui;
use thz_uninstaller::constants::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};
use thz_uninstaller::ui::platform::fonts::configure_windows_fonts;
use thz_uninstaller::ui::platform::screenshot::parse_screenshot_arg;
use thz_uninstaller::ui::UninstallerApp;

fn main() -> eframe::Result<()> {
    let screenshot_path = parse_screenshot_arg();

    let viewport = egui::ViewportBuilder::default()
        .with_title("Uninstaller")
        .with_inner_size([DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT])
        .with_min_inner_size([480.0, 360.0]);

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Uninstaller",
        options,
        Box::new(move |cc| {
            configure_windows_fonts(&cc.egui_ctx);
            Ok(Box::new(UninstallerApp::new(screenshot_path)))
        }),
    )
}
