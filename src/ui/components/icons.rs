use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};

// ============================================================================
// Vector Icon Painter for Program Rows
// ============================================================================

pub fn draw_program_icon(painter: &egui::Painter, center: Pos2, name: &str, is_msi: bool) {
    let lower = name.to_lowercase();
    let r = Rect::from_center_size(center, Vec2::new(16.0, 16.0));

    if lower.contains("google chrome") {
        painter.circle_filled(center, 7.5, Color32::from_rgb(234, 67, 53));
        painter.circle_filled(
            Pos2::new(center.x - 2.0, center.y + 2.5),
            5.5,
            Color32::from_rgb(52, 168, 83),
        );
        painter.circle_filled(
            Pos2::new(center.x + 2.5, center.y + 1.5),
            5.5,
            Color32::from_rgb(251, 188, 5),
        );
        painter.circle_filled(center, 4.0, Color32::WHITE);
        painter.circle_filled(center, 3.0, Color32::from_rgb(66, 133, 244));
    } else if lower.contains("onedrive") {
        painter.circle_filled(
            Pos2::new(center.x - 2.0, center.y + 1.0),
            4.5,
            Color32::from_rgb(0, 120, 212),
        );
        painter.circle_filled(
            Pos2::new(center.x + 2.5, center.y + 1.5),
            4.0,
            Color32::from_rgb(40, 168, 234),
        );
    } else if lower.contains("microsoft 365") || lower.contains("office") {
        let half = 6.5;
        painter.rect_filled(
            Rect::from_min_size(Pos2::new(center.x - half, center.y - half), Vec2::new(6.0, 6.0)),
            0.0,
            Color32::from_rgb(242, 80, 34),
        );
        painter.rect_filled(
            Rect::from_min_size(Pos2::new(center.x + 0.5, center.y - half), Vec2::new(6.0, 6.0)),
            0.0,
            Color32::from_rgb(127, 186, 0),
        );
        painter.rect_filled(
            Rect::from_min_size(Pos2::new(center.x - half, center.y + 0.5), Vec2::new(6.0, 6.0)),
            0.0,
            Color32::from_rgb(0, 164, 239),
        );
        painter.rect_filled(
            Rect::from_min_size(Pos2::new(center.x + 0.5, center.y + 0.5), Vec2::new(6.0, 6.0)),
            0.0,
            Color32::from_rgb(255, 185, 0),
        );
    } else if lower.contains("brave") {
        painter.circle_filled(center, 7.0, Color32::from_rgb(251, 84, 43));
        painter.circle_filled(center, 3.5, Color32::WHITE);
    } else if lower.contains("edge") {
        painter.circle_filled(center, 7.2, Color32::from_rgb(0, 120, 215));
        painter.circle_filled(
            Pos2::new(center.x + 1.5, center.y - 1.0),
            4.5,
            Color32::from_rgb(50, 200, 160),
        );
    } else if lower.contains("gemini") {
        let pts = vec![
            Pos2::new(center.x, center.y - 7.5),
            Pos2::new(center.x + 2.5, center.y - 2.5),
            Pos2::new(center.x + 7.5, center.y),
            Pos2::new(center.x + 2.5, center.y + 2.5),
            Pos2::new(center.x, center.y + 7.5),
            Pos2::new(center.x - 2.5, center.y + 2.5),
            Pos2::new(center.x - 7.5, center.y),
            Pos2::new(center.x - 2.5, center.y - 2.5),
        ];
        painter.add(egui::Shape::convex_polygon(
            pts,
            Color32::from_rgb(76, 139, 245),
            Stroke::NONE,
        ));
    } else if lower.contains("nvidia") {
        painter.rect_filled(r, 2.0, Color32::from_rgb(20, 24, 20));
        painter.circle_stroke(
            center,
            4.5,
            Stroke::new(2.0_f32, Color32::from_rgb(118, 185, 0)),
        );
    } else if lower.contains("discord") {
        painter.circle_filled(center, 7.2, Color32::from_rgb(88, 101, 242));
        painter.circle_filled(Pos2::new(center.x - 2.5, center.y), 1.5, Color32::WHITE);
        painter.circle_filled(Pos2::new(center.x + 2.5, center.y), 1.5, Color32::WHITE);
    } else if lower.contains("android studio") {
        painter.circle_filled(center, 5.5, Color32::from_rgb(61, 220, 132));
    } else if lower.contains("visual studio") {
        painter.rect_filled(r, 2.0, Color32::from_rgb(104, 33, 122));
    } else if lower.contains("logi options") {
        painter.rect_filled(r, 3.0, Color32::from_rgb(130, 90, 230));
    } else if lower.contains("google drive") {
        painter.circle_filled(center, 6.5, Color32::from_rgb(52, 168, 83));
        painter.circle_filled(
            Pos2::new(center.x + 2.0, center.y + 2.0),
            4.0,
            Color32::from_rgb(251, 188, 5),
        );
    } else if lower.contains("readest") {
        painter.rect_filled(r, 2.0, Color32::from_rgb(230, 198, 145));
    } else if lower.contains("smart connect") || lower.contains("opencode") {
        painter.rect_filled(r, 3.0, Color32::from_rgb(24, 28, 34));
        painter.rect_stroke(
            r.shrink(3.5),
            1.0,
            Stroke::new(1.5_f32, Color32::from_rgb(0, 190, 210)),
            egui::StrokeKind::Inside,
        );
    } else if lower.contains("antigravity") {
        if lower.contains("ide") {
            painter.rect_filled(r, 2.0, Color32::from_rgb(18, 20, 24));
        } else {
            painter.circle_filled(center, 7.2, Color32::WHITE);
        }
        let pts = vec![
            Pos2::new(center.x, center.y - 5.0),
            Pos2::new(center.x + 5.0, center.y + 5.0),
            Pos2::new(center.x - 5.0, center.y + 5.0),
        ];
        painter.add(egui::Shape::convex_polygon(
            pts,
            Color32::from_rgb(66, 133, 244),
            Stroke::NONE,
        ));
    } else if is_msi
        || lower.contains("runtime")
        || lower.contains(".net")
        || lower.contains("software development kit")
    {
        // Classic Windows installer box / computer icon
        painter.rect_filled(
            r.shrink(1.0),
            1.5,
            Color32::from_rgb(220, 226, 232),
        );
        painter.rect_stroke(
            r.shrink(1.0),
            1.5,
            Stroke::new(1.0_f32, Color32::from_rgb(90, 105, 125)),
            egui::StrokeKind::Inside,
        );
        painter.rect_filled(
            Rect::from_min_size(Pos2::new(center.x - 5.0, center.y - 5.0), Vec2::new(6.0, 5.0)),
            0.0,
            Color32::from_rgb(40, 140, 210),
        );
    } else {
        // Generic application package icon
        painter.rect_filled(
            r.shrink(1.5),
            2.0,
            Color32::from_rgb(80, 90, 102),
        );
    }
}

