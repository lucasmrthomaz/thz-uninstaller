use eframe::egui::{self, Color32, FontId, Pos2, Rect, Sense, Stroke, Vec2};

pub fn render_simulated_title_bar(ui: &mut egui::Ui, ctx: &egui::Context, full_rect: Rect) {
    let title_rect = Rect::from_min_max(
        Pos2::new(full_rect.left(), full_rect.top()),
        Pos2::new(full_rect.right(), full_rect.top() + 34.0),
    );
    ui.painter().rect_filled(
        title_rect,
        0.0,
        Color32::from_rgb(241, 243, 249),
    );

    // App icon in title bar (16x16 at x=18, y=18)
    let icon_c = Pos2::new(18.0, 18.0);
    ui.painter()
        .circle_filled(icon_c, 8.0, Color32::from_rgb(45, 52, 62));
    ui.painter().circle_stroke(
        icon_c,
        4.5,
        Stroke::new(2.0_f32, Color32::from_rgb(65, 190, 240)),
    );

    ui.painter().text(
        Pos2::new(32.0, 18.0),
        egui::Align2::LEFT_CENTER,
        "THZ Uninstaller",
        FontId::proportional(12.0),
        Color32::from_rgb(120, 120, 125),
    );

    // Caption minimize / maximize / close icons
    let cap_color = Color32::from_rgb(120, 120, 125);
    ui.painter().line_segment(
        [Pos2::new(458.0, 18.0), Pos2::new(468.0, 18.0)],
        Stroke::new(1.0_f32, cap_color),
    );
    ui.painter().rect_stroke(
        Rect::from_center_size(Pos2::new(509.0, 18.0), Vec2::new(10.0, 10.0)),
        0.0,
        Stroke::new(1.0_f32, cap_color),
        egui::StrokeKind::Inside,
    );
    let close_rect = Rect::from_center_size(Pos2::new(555.0, 18.0), Vec2::new(28.0, 24.0));
    if ui
        .interact(close_rect, ui.id().with("win_close"), Sense::click())
        .clicked()
    {
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
    ui.painter().line_segment(
        [Pos2::new(550.0, 13.0), Pos2::new(560.0, 23.0)],
        Stroke::new(1.0_f32, cap_color),
    );
    ui.painter().line_segment(
        [Pos2::new(560.0, 13.0), Pos2::new(550.0, 23.0)],
        Stroke::new(1.0_f32, cap_color),
    );
}

pub fn render_window_border(ui: &mut egui::Ui, full_rect: Rect) {
    ui.painter().rect_stroke(
        full_rect,
        0.0,
        Stroke::new(1.0_f32, Color32::from_rgb(17, 18, 21)),
        egui::StrokeKind::Inside,
    );
    ui.painter().rect_stroke(
        full_rect.shrink(1.0),
        0.0,
        Stroke::new(1.0_f32, Color32::from_rgb(57, 58, 59)),
        egui::StrokeKind::Inside,
    );
}

