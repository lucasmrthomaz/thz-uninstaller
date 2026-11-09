use eframe::egui::{self, Color32, RichText};

use crate::models::{ActiveModal, TraceCategory};
use crate::services::cleaner::{clean_traces, delete_uninstall_entry};
use crate::services::process::kill_processes;
use crate::services::uninstaller::launch_uninstaller;
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
        ActiveModal::ConfirmUninstall {
            program,
            running_processes,
        } => {
            let prog = program.clone();
            let procs = running_processes.clone();

            egui::Window::new("Desinstalar Programa")
                .collapsible(false)
                .resizable(false)
                .default_width(460.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(
                        RichText::new(format!(
                            "Deseja desinstalar \"{}\"?",
                            prog.display_name
                        ))
                        .strong(),
                    );

                    if !prog.uninstall_string.is_empty() {
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(format!("Comando: {}", prog.uninstall_string))
                                .monospace()
                                .small(),
                        );
                    }

                    // Pre-uninstall: process warning & termination action
                    if !procs.is_empty() {
                        ui.add_space(8.0);
                        ui.colored_label(
                            Color32::from_rgb(220, 120, 20),
                            format!(
                                "⚠ Atenção: {} processo(s) deste programa estão em execução:",
                                procs.len()
                            ),
                        );
                        egui::ScrollArea::vertical()
                            .max_height(80.0)
                            .show(ui, |ui| {
                                for p in &procs {
                                    ui.label(format!("• {} (PID {})", p.name, p.pid));
                                }
                            });
                    }

                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if !procs.is_empty() {
                            if ui.button("Encerrar Processos e Desinstalar").clicked() {
                                let _ = kill_processes(&procs);
                                match launch_uninstaller(&prog) {
                                    Ok(active) => {
                                        app.active_uninstaller = Some(active);
                                        next_modal = Some(ActiveModal::UninstallingProgress {
                                            program: prog.clone(),
                                            elapsed_secs: 0,
                                        });
                                    }
                                    Err(e) => {
                                        next_modal = Some(ActiveModal::StatusMessage {
                                            title: "Erro ao Iniciar Desinstalador".to_string(),
                                            message: e,
                                        });
                                    }
                                }
                            }
                            if ui.button("Desinstalar Sem Encerrar").clicked() {
                                match launch_uninstaller(&prog) {
                                    Ok(active) => {
                                        app.active_uninstaller = Some(active);
                                        next_modal = Some(ActiveModal::UninstallingProgress {
                                            program: prog.clone(),
                                            elapsed_secs: 0,
                                        });
                                    }
                                    Err(e) => {
                                        next_modal = Some(ActiveModal::StatusMessage {
                                            title: "Erro ao Iniciar Desinstalador".to_string(),
                                            message: e,
                                        });
                                    }
                                }
                            }
                        } else {
                            if ui.button("Iniciar Desinstalação").clicked() {
                                match launch_uninstaller(&prog) {
                                    Ok(active) => {
                                        app.active_uninstaller = Some(active);
                                        next_modal = Some(ActiveModal::UninstallingProgress {
                                            program: prog.clone(),
                                            elapsed_secs: 0,
                                        });
                                    }
                                    Err(e) => {
                                        next_modal = Some(ActiveModal::StatusMessage {
                                            title: "Erro ao Iniciar Desinstalador".to_string(),
                                            message: e,
                                        });
                                    }
                                }
                            }
                        }

                        if ui.button("Cancelar").clicked() {
                            next_modal = Some(ActiveModal::None);
                        }
                    });
                });
        }
        ActiveModal::UninstallingProgress {
            program,
            elapsed_secs,
        } => {
            let prog = program.clone();
            let secs = *elapsed_secs;

            egui::Window::new("Desinstalação em Andamento")
                .collapsible(false)
                .resizable(false)
                .default_width(440.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(
                        RichText::new(format!(
                            "Aguardando a conclusão do desinstalador de \"{}\"...",
                            prog.display_name
                        ))
                        .strong(),
                    );
                    ui.add_space(6.0);
                    ui.label("O assistente oficial de desinstalação está em execução. Quando ele finalizar, a varredura profunda de vestígios residuais no Registro e no Disco será iniciada automaticamente.");
                    ui.add_space(8.0);
                    ui.colored_label(
                        Color32::from_rgb(0, 102, 204),
                        format!("⏱ Tempo decorrido: {} segundo(s)", secs),
                    );

                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if ui.button("Pular Espera e Verificar Vestígios Agora").clicked() {
                            app.active_uninstaller = None;
                            app.on_uninstaller_finished(prog, true);
                        }
                        if ui.button("Cancelar Acompanhamento").clicked() {
                            next_modal = Some(ActiveModal::None);
                        }
                    });
                });
        }
        ActiveModal::ConfirmRemoveEntry { program } => {
            let prog = program.clone();
            egui::Window::new("Remover Entrada do Registro")
                .collapsible(false)
                .resizable(false)
                .default_width(420.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(
                        RichText::new(format!(
                            "Remover entrada de \"{}\" do Registro?",
                            prog.display_name
                        ))
                        .strong(),
                    );
                    ui.add_space(6.0);
                    ui.label("Atenção: Esta ação exclui permanentemente a chave de desinstalação do Registro do Windows. Use esta opção caso o programa já tenha sido removido manualmente do disco ou seu desinstalador não exista mais.");
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(format!("Chave: {}", prog.full_reg_path()))
                            .monospace()
                            .small(),
                    );

                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        if ui.button("Remover Entrada do Registro").clicked() {
                            match delete_uninstall_entry(&prog) {
                                Ok(_) => {
                                    remove_program_id = Some(prog.id.clone());
                                    app.status_banner = Some(format!(
                                        "Entrada de \"{}\" removida do Registro com sucesso.",
                                        prog.display_name
                                    ));
                                    next_modal = Some(ActiveModal::None);
                                }
                                Err(e) => {
                                    next_modal = Some(ActiveModal::StatusMessage {
                                        title: "Erro ao Remover Entrada".to_string(),
                                        message: format!(
                                            "Não foi possível remover a chave do Registro:\n{}\n\nCertifique-se de executar o aplicativo como Administrador.",
                                            e
                                        ),
                                    });
                                }
                            }
                        }
                        if ui.button("Cancelar").clicked() {
                            next_modal = Some(ActiveModal::None);
                        }
                    });
                });
        }
        ActiveModal::LeftoverScanner {
            program,
            is_force_removal,
            traces,
            cleaned_summary,
        } => {
            let title = if *is_force_removal {
                format!("Remoção Forçada - {}", program.display_name)
            } else {
                format!("Vestígios Residuais - {}", program.display_name)
            };
            let prog_id = program.id.clone();
            let prog_clone = program.clone();
            let force = *is_force_removal;

            egui::Window::new(title)
                .collapsible(false)
                .resizable(true)
                .default_size([540.0, 380.0])
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

                    let checked_count = traces.iter().filter(|t| t.checked).count();

                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "Vestígios detectados: {} ({} marcados)",
                            traces.len(),
                            checked_count
                        ));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("Desmarcar Todos").clicked() {
                                for t in traces.iter_mut() {
                                    t.checked = false;
                                }
                            }
                            if ui.small_button("Selecionar Todos").clicked() {
                                for t in traces.iter_mut() {
                                    t.checked = true;
                                }
                            }
                        });
                    });

                    ui.add_space(4.0);

                    egui::ScrollArea::vertical()
                        .max_height(220.0)
                        .show(ui, |ui| {
                            if fs_count > 0 {
                                egui::CollapsingHeader::new(
                                    RichText::new(format!(
                                        "Sistema de Arquivos ({} itens)",
                                        fs_count
                                    ))
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
                            }

                            if reg_count > 0 {
                                egui::CollapsingHeader::new(
                                    RichText::new(format!(
                                        "Registro do Windows ({} itens)",
                                        reg_count
                                    ))
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
                            }
                        });

                    if let Some(ref summary) = cleaned_summary {
                        ui.add_space(6.0);
                        ui.colored_label(
                            Color32::from_rgb(20, 130, 50),
                            format!(
                                "✓ Removidos com sucesso: {} chave(s) do Registro e {} item(ns) de arquivo/pasta.",
                                summary.registry_deleted, summary.filesystem_deleted
                            ),
                        );
                        if !summary.errors.is_empty() {
                            ui.colored_label(
                                Color32::from_rgb(200, 40, 40),
                                format!(
                                    "⚠ Falha ao remover {} item(ns) (permissão ou arquivo em uso):\n{}",
                                    summary.errors.len(),
                                    summary.errors.join("\n")
                                ),
                            );
                        }
                    }

                    ui.separator();
                    ui.horizontal(|ui| {
                        let can_clean = cleaned_summary.is_none() && checked_count > 0;
                        if ui.add_enabled(can_clean, egui::Button::new("Limpar Vestígios Selecionados")).clicked() {
                            let summary = clean_traces(traces, Some(&prog_clone), force);
                            *cleaned_summary = Some(summary);
                            remove_program_id = Some(prog_id.clone());
                        }
                        if ui.button("Fechar").clicked() {
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
