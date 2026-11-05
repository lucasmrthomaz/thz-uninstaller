use eframe::egui::{self, Color32, FontFamily, FontId, Pos2, Rect, Stroke};

use crate::models::ProgramEntry;
use crate::ui::app::UninstallerApp;
use crate::utils::formatting::format_total_bytes_str;

pub fn render_status_bar(
    app: &UninstallerApp,
    ui: &mut egui::Ui,
    w: f32,
    h: f32,
    filtered_items: &[ProgramEntry],
    total_size_bytes: u64,
) {
    let status_top = h - 23.0;
    let status_rect = Rect::from_min_max(
        Pos2::new(2.0, status_top),
        Pos2::new(w - 2.0, h - 2.0),
    );
    ui.painter()
        .rect_filled(status_rect, 0.0, Color32::from_rgb(240, 240, 240));
    ui.painter().line_segment(
        [Pos2::new(2.0, status_top + 0.5), Pos2::new(w - 2.0, status_top + 0.5)],
        Stroke::new(1.0_f32, Color32::from_rgb(215, 215, 215)),
    );

    let status_y = h - 12.0;
    let count_str = format!("{}", filtered_items.len());
    let total_str = format_total_bytes_str(total_size_bytes, app.is_portuguese);

    // Bold count + regular middle text + bold total size
    let g_count = ui.painter().layout_no_wrap(
        count_str,
        FontId::new(12.0, FontFamily::Name("bold".into())),
        Color32::BLACK,
    );
    let cx = 8.0;
    let c_w = g_count.size().x;
    ui.painter().galley(
        Pos2::new(cx, status_y - g_count.size().y * 0.5),
        g_count,
        Color32::BLACK,
    );

    let mid_text = if app.is_portuguese {
        " programas com tamanho total de "
    } else {
        " programs of "
    };
    let g_mid = ui.painter().layout_no_wrap(
        mid_text.to_string(),
        FontId::proportional(12.0),
        Color32::BLACK,
    );
    let m_w = g_mid.size().x;
    ui.painter().galley(
        Pos2::new(cx + c_w, status_y - g_mid.size().y * 0.5),
        g_mid,
        Color32::BLACK,
    );

    let g_tot = ui.painter().layout_no_wrap(
        total_str,
        FontId::new(12.0, FontFamily::Name("bold".into())),
        Color32::BLACK,
    );
    ui.painter().galley(
        Pos2::new(cx + c_w + m_w, status_y - g_tot.size().y * 0.5),
        g_tot,
        Color32::BLACK,
    );

    // Vertical divider at x=388
    ui.painter().line_segment(
        [Pos2::new(388.0, status_top + 2.0), Pos2::new(388.0, h - 3.0)],
        Stroke::new(1.0_f32, Color32::from_rgb(215, 215, 215)),
    );

    // Right status link
    let star_c = Pos2::new(403.0, status_y);
    ui.painter()
        .circle_filled(star_c, 4.5, Color32::from_rgb(242, 195, 50));
    let link_text = "";
    let link_color = Color32::from_rgb(0, 102, 204);
    let g_link = ui.painter().layout_no_wrap(
        link_text.to_string(),
        FontId::proportional(12.0),
        link_color,
    );
    let link_w = g_link.size().x;
    ui.painter().galley(
        Pos2::new(413.0, status_y - g_link.size().y * 0.5),
        g_link,
        link_color,
    );
    ui.painter().line_segment(
        [
            Pos2::new(413.0, status_y + 6.5),
            Pos2::new(413.0 + link_w, status_y + 6.5),
        ],
        Stroke::new(1.0_f32, link_color),
    );
}
