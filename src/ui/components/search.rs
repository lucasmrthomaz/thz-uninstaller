use eframe::egui::{self, Color32, FontId, Key, Pos2, Rect, Stroke};

use crate::constants::SEARCH_PLACEHOLDER;
use crate::models::ActiveModal;
use crate::ui::app::UninstallerApp;

pub fn render_search_bar(app: &mut UninstallerApp, ui: &mut egui::Ui, ctx: &egui::Context, w: f32, h: f32) {
    let search_top = h - 43.0;
    let search_bot = h - 24.0;
    let search_rect = Rect::from_min_max(
        Pos2::new(2.0, search_top),
        Pos2::new(w - 2.0, search_bot),
    );
    ui.painter().rect_filled(search_rect, 0.0, Color32::WHITE);
    ui.painter().line_segment(
        [Pos2::new(2.0, search_top), Pos2::new(w - 2.0, search_top)],
        Stroke::new(1.0_f32, Color32::from_rgb(236, 236, 236)),
    );
    ui.painter().line_segment(
        [Pos2::new(2.0, search_bot + 0.5), Pos2::new(w - 2.0, search_bot + 0.5)],
        Stroke::new(1.0_f32, Color32::from_rgb(131, 131, 131)),
    );

    // Support instant typing into the search filter when no modal is open
    if matches!(app.active_modal, ActiveModal::None) {
        let mut typed_change = false;
        ctx.input(|i| {
            if !i.modifiers.command && !i.modifiers.ctrl && !i.modifiers.alt {
                for ev in &i.events {
                    if let egui::Event::Text(text) = ev {
                        for ch in text.chars() {
                            if !ch.is_control() {
                                app.search_query.push(ch);
                                typed_change = true;
                            }
                        }
                    }
                }
                if i.key_pressed(Key::Backspace) && !app.search_query.is_empty() {
                    app.search_query.pop();
                    typed_change = true;
                }
            }
        });
        if typed_change {
            app.scroll_row_offset = 0;
            let updated = app.filtered_and_sorted();
            app.selected_id = updated.first().map(|p| p.id.clone());
        }
    }

    let placeholder = if app.is_portuguese {
        "Digite para encontrar um programa"
    } else {
        SEARCH_PLACEHOLDER
    };
    if app.search_query.is_empty() {
        ui.painter().text(
            search_rect.center(),
            egui::Align2::CENTER_CENTER,
            placeholder,
            FontId::proportional(12.0),
            Color32::from_rgb(109, 109, 109),
        );
    } else {
        ui.painter().text(
            search_rect.center(),
            egui::Align2::CENTER_CENTER,
            &app.search_query,
            FontId::proportional(12.0),
            Color32::BLACK,
        );
    }
}

