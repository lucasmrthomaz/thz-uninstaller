use eframe::egui::{self, Key};

use crate::models::{ActiveModal, ProgramEntry, ViewMode};
use crate::ui::app::UninstallerApp;

pub fn handle_keyboard_shortcuts(app: &mut UninstallerApp, ctx: &egui::Context) {
    if !matches!(app.active_modal, ActiveModal::None) {
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            app.active_modal = ActiveModal::None;
        }
        return;
    }

    let mut do_refresh = false;
    let mut do_desktop = false;
    let mut do_store = false;
    let mut do_export = false;
    let mut do_uninstall = false;
    let mut do_force = false;
    let mut do_reg = false;
    let mut do_folder = false;
    let mut do_web = false;
    let mut do_google = false;
    let mut do_escape = false;

    ctx.input(|i| {
        if i.key_pressed(Key::F5) {
            do_refresh = true;
        }
        if i.key_pressed(Key::F9) {
            do_desktop = true;
        }
        if i.key_pressed(Key::F1) {
            do_store = true;
        }
        if i.modifiers.command && i.key_pressed(Key::S) {
            do_export = true;
        }
        if i.key_pressed(Key::Enter) && !i.modifiers.any() {
            do_uninstall = true;
        }
        if i.modifiers.shift && i.key_pressed(Key::Delete) {
            do_force = true;
        }
        if i.modifiers.command && i.key_pressed(Key::R) {
            do_reg = true;
        }
        if i.modifiers.command && i.key_pressed(Key::E) {
            do_folder = true;
        }
        if i.modifiers.command && i.key_pressed(Key::W) {
            do_web = true;
        }
        if i.modifiers.command && i.key_pressed(Key::G) {
            do_google = true;
        }
        if i.key_pressed(Key::Escape) {
            do_escape = true;
        }
    });

    if do_refresh {
        app.refresh();
    }
    if do_desktop {
        app.view_mode = ViewMode::DesktopApps;
        app.scroll_row_offset = 0;
        app.selected_id = app.filtered_and_sorted().first().map(|p| p.id.clone());
    }
    if do_store {
        app.view_mode = ViewMode::StoreApps;
        app.scroll_row_offset = 0;
        app.selected_id = app.filtered_and_sorted().first().map(|p| p.id.clone());
    }
    if do_export {
        app.export_html();
    }
    if do_escape && !app.search_query.is_empty() {
        app.search_query.clear();
        app.scroll_row_offset = 0;
    }
    if let Some(sel) = app.selected_program() {
        if do_uninstall {
            app.trigger_uninstall(sel);
        } else if do_force {
            app.trigger_force_removal(sel);
        } else if do_reg {
            app.open_registry_entry(&sel);
        } else if do_folder {
            app.open_installation_folder(&sel);
        } else if do_web {
            app.open_website_or_store(&sel);
        } else if do_google {
            app.google_for(&sel.display_name);
        }
    }
}

pub fn handle_list_navigation_and_scroll(
    app: &mut UninstallerApp,
    ctx: &egui::Context,
    filtered_items: &[ProgramEntry],
    max_visible_rows: usize,
    max_scroll: usize,
) {
    if !matches!(app.active_modal, ActiveModal::None) {
        return;
    }

    // Mouse wheel / touchpad scrolling
    let wheel_dy = ctx.input(|i| {
        if i.raw_scroll_delta.y.abs() > 0.0 {
            i.raw_scroll_delta.y
        } else {
            i.smooth_scroll_delta.y
        }
    });
    if wheel_dy.abs() > 0.0 {
        app.scroll_accum -= wheel_dy;
        let step_px = 16.0;
        let steps = (app.scroll_accum / step_px).trunc() as isize;
        if steps != 0 {
            app.scroll_accum -= (steps as f32) * step_px;
            let next_off = (app.scroll_row_offset as isize + steps)
                .clamp(0, max_scroll as isize) as usize;
            app.scroll_row_offset = next_off;
        }
    }

    // Keyboard list navigation (Up/Down/PageUp/PageDown/Home/End)
    let mut nav_delta: Option<isize> = None;
    let mut nav_home = false;
    let mut nav_end = false;
    ctx.input(|i| {
        if i.key_pressed(Key::ArrowDown) {
            nav_delta = Some(1);
        } else if i.key_pressed(Key::ArrowUp) {
            nav_delta = Some(-1);
        } else if i.key_pressed(Key::PageDown) {
            nav_delta = Some(max_visible_rows as isize);
        } else if i.key_pressed(Key::PageUp) {
            nav_delta = Some(-(max_visible_rows as isize));
        } else if i.key_pressed(Key::Home) {
            nav_home = true;
        } else if i.key_pressed(Key::End) {
            nav_end = true;
        }
    });

    let total_rows = filtered_items.len();
    if !filtered_items.is_empty() && (nav_delta.is_some() || nav_home || nav_end) {
        let cur_idx = app
            .selected_id
            .as_ref()
            .and_then(|id| filtered_items.iter().position(|p| &p.id == id))
            .unwrap_or(0);
        let new_idx = if nav_home {
            0
        } else if nav_end {
            total_rows - 1
        } else if let Some(d) = nav_delta {
            (cur_idx as isize + d).clamp(0, (total_rows - 1) as isize) as usize
        } else {
            cur_idx
        };
        app.selected_id = Some(filtered_items[new_idx].id.clone());
        if new_idx < app.scroll_row_offset {
            app.scroll_row_offset = new_idx;
        } else if new_idx >= app.scroll_row_offset + max_visible_rows {
            app.scroll_row_offset = new_idx + 1 - max_visible_rows;
        }
    }
}

