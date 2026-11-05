use eframe::egui::{
    self, Color32, FontId, Pos2, Rect, Sense, Stroke,
};

use crate::constants::LIST_ROW_HEIGHT;
use crate::models::{ProgramEntry, SortColumn};
use crate::ui::app::UninstallerApp;
use crate::ui::components::icons::draw_program_icon;
use crate::ui::components::menu::render_action_menu_items;

pub struct TableLayout {
    pub hdr_top: f32,
    pub hdr_bot: f32,
    pub list_top: f32,
    pub list_bot: f32,
    pub col0_x: f32,
    pub col1_x: f32,
    pub col2_x: f32,
    pub max_visible_rows: usize,
    pub total_rows: usize,
    pub max_scroll: usize,
}

impl TableLayout {
    pub fn compute(app: &UninstallerApp, top_offset: f32, h: f32, total_rows: usize) -> Self {
        let hdr_top = top_offset + 25.0;
        let hdr_bot = hdr_top + 24.0;
        let col0_x = 2.0 + app.col_name_width;
        let col1_x = col0_x + app.col_size_width;
        let col2_x = col1_x + app.col_date_width;
        let list_top = hdr_bot;
        let list_bot = (h - 44.0).max(list_top + LIST_ROW_HEIGHT);
        let max_visible_rows = (((list_bot - list_top) / LIST_ROW_HEIGHT) as usize).max(1);
        let max_scroll = total_rows.saturating_sub(max_visible_rows);

        Self {
            hdr_top,
            hdr_bot,
            list_top,
            list_bot,
            col0_x,
            col1_x,
            col2_x,
            max_visible_rows,
            total_rows,
            max_scroll,
        }
    }
}

pub fn render_column_headers(app: &mut UninstallerApp, ui: &mut egui::Ui, layout: &TableLayout) {
    let hdr_rect = Rect::from_min_max(
        Pos2::new(2.0, layout.hdr_top),
        Pos2::new(layout.col2_x, layout.hdr_bot),
    );
    ui.painter().rect_filled(hdr_rect, 0.0, Color32::WHITE);

    let (hdr_name, hdr_size, hdr_date) = if app.is_portuguese {
        ("Nome do Programa", "Tamanho", "Instalado Em")
    } else {
        ("Program Name", "Size", "Installed On")
    };

    let c0_r = Rect::from_min_max(Pos2::new(2.0, layout.hdr_top), Pos2::new(layout.col0_x, layout.hdr_bot));
    let c1_r = Rect::from_min_max(Pos2::new(layout.col0_x, layout.hdr_top), Pos2::new(layout.col1_x, layout.hdr_bot));
    let c2_r = Rect::from_min_max(Pos2::new(layout.col1_x, layout.hdr_top), Pos2::new(layout.col2_x, layout.hdr_bot));

    for (r, col, lbl, tx) in [
        (c0_r, SortColumn::ProgramName, hdr_name, 21.0),
        (c1_r, SortColumn::Size, hdr_size, layout.col0_x + 13.0),
        (c2_r, SortColumn::InstalledOn, hdr_date, layout.col1_x + 22.0),
    ] {
        if ui
            .interact(r, ui.id().with(("hdr", lbl)), Sense::click())
            .clicked()
        {
            if app.sort_column == col {
                app.sort_ascending = !app.sort_ascending;
            } else {
                app.sort_column = col;
                app.sort_ascending = col == SortColumn::ProgramName;
            }
            app.scroll_row_offset = 0;
        }
        ui.painter().text(
            Pos2::new(tx, layout.hdr_top + 12.0),
            egui::Align2::LEFT_CENTER,
            lbl,
            FontId::proportional(12.0),
            Color32::BLACK,
        );
    }

    ui.painter().line_segment(
        [Pos2::new(layout.col0_x, layout.hdr_top + 2.0), Pos2::new(layout.col0_x, layout.hdr_bot - 2.0)],
        Stroke::new(1.0_f32, Color32::from_rgb(229, 229, 229)),
    );
    ui.painter().line_segment(
        [Pos2::new(layout.col1_x, layout.hdr_top + 2.0), Pos2::new(layout.col1_x, layout.hdr_bot - 2.0)],
        Stroke::new(1.0_f32, Color32::from_rgb(229, 229, 229)),
    );
}

pub fn render_list_rows(
    app: &mut UninstallerApp,
    ui: &mut egui::Ui,
    layout: &TableLayout,
    filtered_items: &[ProgramEntry],
) {
    app.scroll_row_offset = app.scroll_row_offset.min(layout.max_scroll);

    for (vis_idx, (idx, item)) in filtered_items
        .iter()
        .enumerate()
        .skip(app.scroll_row_offset)
        .take(layout.max_visible_rows)
        .enumerate()
    {
        let y0 = layout.list_top + (vis_idx as f32) * LIST_ROW_HEIGHT;
        let y1 = y0 + LIST_ROW_HEIGHT;
        let row_rect = Rect::from_min_max(Pos2::new(2.0, y0), Pos2::new(layout.col2_x, y1));

        let is_selected = app.selected_id.as_deref() == Some(item.id.as_str());

        let (fill_color, border_color_opt) = if is_selected {
            (
                Color32::from_rgb(209, 232, 255),
                Some(Color32::from_rgb(102, 167, 232)),
            )
        } else if item.is_recent {
            (
                Color32::from_rgb(255, 234, 191),
                Some(Color32::from_rgb(255, 183, 128)),
            )
        } else if idx % 2 == 1 {
            (Color32::from_rgb(226, 229, 234), None)
        } else {
            (Color32::from_rgb(240, 240, 240), None)
        };

        ui.painter().rect_filled(row_rect, 0.0, fill_color);
        if let Some(bc) = border_color_opt {
            ui.painter().line_segment(
                [Pos2::new(4.0, y0 + 0.5), Pos2::new(layout.col2_x - 1.0, y0 + 0.5)],
                Stroke::new(1.0_f32, bc),
            );
            ui.painter().line_segment(
                [Pos2::new(4.0, y1 - 0.5), Pos2::new(layout.col2_x - 1.0, y1 - 0.5)],
                Stroke::new(1.0_f32, bc),
            );
        }

        // Draw 16x16 Program Icon at x=32, y=y0+12
        draw_program_icon(
            ui.painter(),
            Pos2::new(32.0, y0 + 12.0),
            &item.display_name,
            item.is_msi,
        );

        // Column 0: Program Name + optional gray "(32-bit)"
        let name_clip = Rect::from_min_max(
            Pos2::new(44.0, y0),
            Pos2::new(layout.col0_x - 4.0, y1),
        );
        let mut display_title = item.display_name.clone();
        if display_title.len() > 54 {
            display_title.truncate(51);
            display_title.push_str("...");
        }
        let name_galley = ui.painter().layout_no_wrap(
            display_title,
            FontId::proportional(12.0),
            Color32::BLACK,
        );
        let name_w_px = name_galley.size().x;
        let name_pos = Pos2::new(46.0, y0 + 12.0 - name_galley.size().y * 0.5);
        ui.painter().with_clip_rect(name_clip).galley(
            name_pos,
            name_galley,
            Color32::BLACK,
        );

        if item.is_32bit && !item.is_store_app {
            ui.painter().with_clip_rect(name_clip).text(
                Pos2::new(46.0 + name_w_px + 4.0, y0 + 12.0),
                egui::Align2::LEFT_CENTER,
                "(32-bit)",
                FontId::proportional(11.5),
                Color32::from_rgb(128, 128, 128),
            );
        }

        // Column 1: Size
        let size_str = item.formatted_size(app.is_portuguese);
        if !size_str.is_empty() {
            ui.painter().text(
                Pos2::new(layout.col0_x + 18.0, y0 + 12.0),
                egui::Align2::LEFT_CENTER,
                size_str,
                FontId::proportional(12.0),
                Color32::BLACK,
            );
        }

        // Column 2: Installed On
        if !item.install_date.is_empty() {
            ui.painter().text(
                Pos2::new(layout.col1_x + 22.0, y0 + 12.0),
                egui::Align2::LEFT_CENTER,
                &item.install_date,
                FontId::proportional(12.0),
                Color32::BLACK,
            );
        }

        let resp = ui.interact(
            row_rect,
            ui.id().with(("row", &item.id)),
            Sense::click(),
        );
        if resp.clicked() || resp.secondary_clicked() {
            app.selected_id = Some(item.id.clone());
        }
        if resp.double_clicked() {
            app.trigger_uninstall(item.clone());
        }
        let item_clone = item.clone();
        resp.context_menu(|ui| {
            app.selected_id = Some(item_clone.id.clone());
            render_action_menu_items(app, ui, Some(item_clone));
        });
    }
}

pub fn render_scrollbar(
    app: &mut UninstallerApp,
    ui: &mut egui::Ui,
    layout: &TableLayout,
    w: f32,
) {
    let sb_rect = Rect::from_min_max(
        Pos2::new(layout.col2_x, layout.hdr_top),
        Pos2::new(w - 2.0, layout.list_bot),
    );
    ui.painter()
        .rect_filled(sb_rect, 0.0, Color32::from_rgb(240, 240, 240));

    let track_top = layout.hdr_top + 17.0;
    let track_bot = layout.list_bot;
    let track_h = (track_bot - track_top).max(24.0);
    let thumb_h = if layout.total_rows > layout.max_visible_rows {
        ((layout.max_visible_rows as f32 / layout.total_rows as f32) * 681.0)
            .clamp(32.0, track_h)
    } else {
        track_h
    };

    let sb_resp = ui.interact(
        sb_rect,
        ui.id().with("vscrollbar"),
        Sense::click_and_drag(),
    );
    if (sb_resp.dragged() || sb_resp.clicked()) && layout.max_scroll > 0 {
        if let Some(pos) = sb_resp.interact_pointer_pos() {
            if sb_resp.clicked() && pos.y < track_top {
                app.scroll_row_offset = app.scroll_row_offset.saturating_sub(1);
            } else {
                let denom = (track_h - thumb_h).max(1.0);
                let frac = ((pos.y - track_top - thumb_h * 0.5) / denom).clamp(0.0, 1.0);
                app.scroll_row_offset = (frac * layout.max_scroll as f32).round() as usize;
            }
        }
    }

    let scroll_frac = if layout.max_scroll > 0 {
        app.scroll_row_offset as f32 / layout.max_scroll as f32
    } else {
        0.0
    };
    let thumb_top = track_top + scroll_frac * (track_h - thumb_h);
    let thumb_bot = thumb_top + thumb_h;

    let thumb_rect = Rect::from_min_max(
        Pos2::new(layout.col2_x + 4.0, thumb_top),
        Pos2::new(w - 6.0, thumb_bot),
    );
    let thumb_color = if sb_resp.dragged() || sb_resp.hovered() {
        Color32::from_rgb(166, 166, 166)
    } else {
        Color32::from_rgb(205, 205, 205)
    };
    ui.painter().rect_filled(thumb_rect, 0.0, thumb_color);
}

