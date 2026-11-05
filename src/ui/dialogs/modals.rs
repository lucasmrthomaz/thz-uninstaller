use std::process::Command;

use eframe::egui::{self, Color32, RichText};

use crate::models::{ActiveModal, TraceCategory};
use crate::services::leftover::scan_leftover_traces;
use crate::ui::app::UninstallerApp;

pub fn render_modals(app: &mut UninstallerApp, ctx: &egui::Context) {
    let mut next_modal: Option<ActiveModal> = None;
    let mut remove_program_id: Option<String> = None;

    match &mut app.active_modal {
        ActiveModal::None => {}
        ActiveModal::About => {
            let mut open = true;
            egui::Window::new("About")
                .collapsible(false)
                .resizable(false)
                .default_width(360.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .open(&mut open)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(6.0);
                        ui.heading("Uninstaller 1.5.4.180 (Open-Source)");
                        ui.label("Clean-room open-source desktop & Store program uninstaller");
                        ui.add_space(12.0);
                        if ui.button("   Close   ").clicked() {
                            next_modal = Some(ActiveModal::None);
                        }
                    });
                });
            if !open {
                next_modal = Some(ActiveModal::None);
            }
        }
        ActiveModal::StatusMessage { title, message } => {
            let title_str = title.clone();
            let msg_str = message.clone();
            egui::Window::new(title_str)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(msg_str);
                    ui.add_space(10.0);
                    if ui.button("OK").clicked() {
                        next_modal = Some(ActiveModal::None);
                    }
                });
        }
        ActiveModal::ConfirmUninstall { program } => {
            let prog = program.clone();
            egui::Window::new("Uninstall Program")
                .collapsible(false)
                .resizable(false)
                .default_width(440.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(
                        RichText::new(format!(
                            "Uninstall \"{}\"?",
                            prog.display_name
                        ))
                        .strong(),
                    );
                    if !prog.uninstall_string.is_empty() {
                        ui.label(
                            RichText::new(format!("Command: {}", prog.uninstall_string))
                                .monospace()
                                .small(),
                        );
                    }
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Launch Uninstaller & Scan Leftovers").clicked() {
                            if !prog.uninstall_string.is_empty() {
                                let _ = Command::new("cmd.exe")
                                    .args(["/C", &prog.uninstall_string])
                                    .spawn();
                            }
                            let traces = scan_leftover_traces(&prog);
                            next_modal = Some(ActiveModal::LeftoverScanner {
                                program: prog.clone(),
                                is_force_removal: false,
                                traces,
                                cleaned_count: None,
                            });
                        }
                        if ui.button("Cancel").clicked() {
                            next_modal = Some(ActiveModal::None);
                        }
                    });
                });
        }
        ActiveModal::ConfirmRemoveEntry { program } => {
            let prog = program.clone();
            egui::Window::new("Remove Entry")
                .collapsible(false)
                .resizable(false)
                .default_width(400.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(format!(
                        "Remove registry entry for \"{}\"?",
                        prog.display_name
                    ));
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Remove Entry").clicked() {
                            remove_program_id = Some(prog.id.clone());
                            next_modal = Some(ActiveModal::None);
                        }
                        if ui.button("Cancel").clicked() {
                            next_modal = Some(ActiveModal::None);
                        }
                    });
                });
        }
        ActiveModal::LeftoverScanner {
            program,
            is_force_removal,
            traces,
            cleaned_count,
        } => {
            let title = if *is_force_removal {
                format!("Force Removal - {}", program.display_name)
            } else {
                format!("Leftover Traces - {}", program.display_name)
            };
            let prog_id = program.id.clone();

            egui::Window::new(title)
                .collapsible(false)
                .resizable(true)
                .default_size([500.0, 340.0])
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    let fs_count = traces
                        .iter()
                        .filter(|t| t.category == TraceCategory::FileSystem)
                        .count();
                    let reg_count = traces
                        .iter()
                        .filter(|t| t.category == TraceCategory::Registry)
                        .count();

                    egui::ScrollArea::vertical()
                        .max_height(220.0)
                        .show(ui, |ui| {
                            egui::CollapsingHeader::new(
                                RichText::new(format!("File System ({} items)", fs_count))
                                    .strong(),
                            )
                            .default_open(true)
                            .show(ui, |ui| {
                                for item in traces
                                    .iter_mut()
                                    .filter(|t| t.category == TraceCategory::FileSystem)
                                {
                                    ui.checkbox(
                                        &mut item.checked,
                                        format!("{} ({})", item.path, item.description),
                                    );
                                }
                            });

                            egui::CollapsingHeader::new(
                                RichText::new(format!("Registry ({} items)", reg_count))
                                    .strong(),
                            )
                            .default_open(true)
                            .show(ui, |ui| {
                                for item in traces
                                    .iter_mut()
                                    .filter(|t| t.category == TraceCategory::Registry)
                                {
                                    ui.checkbox(
                                        &mut item.checked,
                                        format!("{} ({})", item.path, item.description),
                                    );
                                }
                            });
                        });

                    if let Some(count) = cleaned_count {
                        ui.colored_label(
                            Color32::from_rgb(20, 130, 50),
                            format!("Removed {} leftover traces.", count),
                        );
                    }

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Finish").clicked() {
                            let checked = traces.iter().filter(|t| t.checked).count();
                            *cleaned_count = Some(checked);
                            remove_program_id = Some(prog_id.clone());
                        }
                        if ui.button("Close").clicked() {
                            next_modal = Some(ActiveModal::None);
                        }
                    });
                });
        }
    }

    if let Some(id) = remove_program_id {
        app.desktop_programs.retain(|p| p.id != id);
        app.store_programs.retain(|p| p.id != id);
        if app.selected_id.as_deref() == Some(id.as_str()) {
            app.selected_id = app.filtered_and_sorted().first().map(|p| p.id.clone());
        }
    }
    if let Some(m) = next_modal {
        app.active_modal = m;
    }
}

