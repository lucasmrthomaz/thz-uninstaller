use std::process::Command;

use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};

use crate::models::{ActiveModal, ProgramEntry, ViewMode};
use crate::ui::app::UninstallerApp;

pub fn render_menu_bar(
    app: &mut UninstallerApp,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    w: f32,
    top_offset: f32,
) {
    let menu_top = top_offset;
    let menu_bot = top_offset + 23.0;
    let menu_rect = Rect::from_min_max(
        Pos2::new(2.0, menu_top),
        Pos2::new(w - 2.0, menu_bot),
    );
    ui.painter().rect_filled(menu_rect, 0.0, Color32::WHITE);
    ui.painter().line_segment(
        [Pos2::new(2.0, menu_bot + 0.5), Pos2::new(w - 2.0, menu_bot + 0.5)],
        Stroke::new(1.0_f32, Color32::from_rgb(180, 180, 180)),
    );
    ui.painter().line_segment(
        [Pos2::new(2.0, menu_bot + 1.5), Pos2::new(w - 2.0, menu_bot + 1.5)],
        Stroke::new(1.0_f32, Color32::from_rgb(160, 160, 160)),
    );

    let menu_ui_rect = Rect::from_min_max(
        Pos2::new(12.0, menu_top + 1.0),
        Pos2::new(w - 10.0, menu_bot - 1.0),
    );
    let mut menu_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(menu_ui_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    menu_ui.spacing_mut().item_spacing = Vec2::new(14.0, 0.0);
    menu_ui.visuals_mut().widgets.inactive.bg_fill = Color32::TRANSPARENT;
    menu_ui.visuals_mut().widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;

    let (m_file, m_act, m_view, m_help) = if app.is_portuguese {
        ("Arquivo", "Ação", "Ver", "Ajuda")
    } else {
        ("File", "Action", "View", "Help")
    };

    menu_ui.menu_button(m_file, |ui| {
        if ui.button("Export to HTML...\tCtrl+S").clicked() {
            app.export_html();
            ui.close_menu();
        }
        ui.separator();
        if ui.button("Exit\tEsc").clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    });

    let selected_prog = app.selected_program();
    menu_ui.menu_button(m_act, |ui| {
        render_action_menu_items(app, ui, selected_prog);
    });

    menu_ui.menu_button(m_view, |ui| {
        if ui
            .radio_value(
                &mut app.view_mode,
                ViewMode::DesktopApps,
                "Desktop Apps\tF9",
            )
            .clicked()
        {
            app.scroll_row_offset = 0;
            app.selected_id = app.filtered_and_sorted().first().map(|p| p.id.clone());
            ui.close_menu();
        }
        if ui
            .radio_value(
                &mut app.view_mode,
                ViewMode::StoreApps,
                "Microsoft Store Apps\tF1",
            )
            .clicked()
        {
            app.scroll_row_offset = 0;
            app.selected_id = app.filtered_and_sorted().first().map(|p| p.id.clone());
            ui.close_menu();
        }
        ui.separator();
        ui.menu_button("Interface Language", |ui| {
            ui.radio_value(&mut app.is_portuguese, true, "Português (Brasil)");
            ui.radio_value(&mut app.is_portuguese, false, "English");
        });
        if ui.button("Reset Columns").clicked() {
            app.reset_columns();
            ui.close_menu();
        }
        ui.separator();
        if ui.button("Refresh\tF5").clicked() {
            app.refresh();
            ui.close_menu();
        }
    });

    menu_ui.menu_button(m_help, |ui| {
        if ui.button("About...").clicked() {
            app.active_modal = ActiveModal::About;
            ui.close_menu();
        }
    });
}

pub fn render_action_menu_items(
    app: &mut UninstallerApp,
    ui: &mut egui::Ui,
    program: Option<ProgramEntry>,
) {
    let has_sel = program.is_some();
    let (lbl_uninst, lbl_mod, lbl_force, lbl_rem, lbl_reg, lbl_dir, lbl_web, lbl_store) =
        if app.is_portuguese {
            (
                "Desinstalar...\tEnter",
                "Modificar...",
                "Remoção Forçada\tShift+Del",
                "Remover Entrada",
                "Entrada do Registro\tCtrl+R",
                "Pasta de Instalação\tCtrl+E",
                "Site do Programa\tCtrl+W",
                "Abrir na Microsoft Store\tCtrl+W",
            )
        } else {
            (
                "Uninstall...\tEnter",
                "Modify...",
                "Force Removal\tShift+Del",
                "Remove Entry",
                "Registry Entry\tCtrl+R",
                "Installation Folder\tCtrl+E",
                "Program Website\tCtrl+W",
                "Open in Microsoft Store\tCtrl+W",
            )
        };

    if ui.add_enabled(has_sel, egui::Button::new(lbl_uninst)).clicked() {
        if let Some(p) = program.clone() {
            app.trigger_uninstall(p);
        }
        ui.close_menu();
    }

    if let Some(ref p) = program {
        if !p.modify_path.is_empty() && !p.no_modify {
            if ui.button(lbl_mod).clicked() {
                let _ = Command::new("cmd.exe").args(["/C", &p.modify_path]).spawn();
                ui.close_menu();
            }
        }
    }

    ui.separator();

    if ui.add_enabled(has_sel, egui::Button::new(lbl_force)).clicked() {
        if let Some(p) = program.clone() {
            app.trigger_force_removal(p);
        }
        ui.close_menu();
    }

    if ui.add_enabled(has_sel, egui::Button::new(lbl_rem)).clicked() {
        if let Some(p) = program.clone() {
            app.trigger_remove_entry(p);
        }
        ui.close_menu();
    }

    ui.separator();

    if ui.add_enabled(has_sel, egui::Button::new(lbl_reg)).clicked() {
        if let Some(p) = program.clone() {
            app.open_registry_entry(&p);
        }
        ui.close_menu();
    }

    let has_folder = program
        .as_ref()
        .map(|p| !p.install_location.is_empty())
        .unwrap_or(false);
    if ui
        .add_enabled(has_folder, egui::Button::new(lbl_dir))
        .clicked()
    {
        if let Some(p) = program.clone() {
            app.open_installation_folder(&p);
        }
        ui.close_menu();
    }

    let is_store = program.as_ref().map(|p| p.is_store_app).unwrap_or(false);
    let web_label = if is_store { lbl_store } else { lbl_web };
    if ui
        .add_enabled(has_sel, egui::Button::new(web_label))
        .clicked()
    {
        if let Some(p) = program.clone() {
            app.open_website_or_store(&p);
        }
        ui.close_menu();
    }

    ui.separator();

    ui.add_enabled_ui(has_sel, |ui| {
        ui.menu_button("Google for", |ui| {
            if let Some(p) = program {
                if ui
                    .button(format!("{}\tCtrl+G", p.display_name))
                    .clicked()
                {
                    app.google_for(&p.display_name);
                    ui.close_menu();
                }
                if !p.publisher.is_empty() {
                    if ui.button(&p.publisher).clicked() {
                        app.google_for(&p.publisher);
                        ui.close_menu();
                    }
                }
            }
        });
    });
}

