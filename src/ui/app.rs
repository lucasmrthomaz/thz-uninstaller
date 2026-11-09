use std::path::PathBuf;

use eframe::egui::{self, Color32};

use crate::constants::{
    COLUMN_INSTALLED_ON_WIDTH, COLUMN_PROGRAM_NAME_WIDTH, COLUMN_SIZE_WIDTH,
    ICON_PATH_BUFFER_LEN, LISTVIEW_EX_STYLE, LVM_GETITEMCOUNT_MSG, MAIN_DIALOG_HEIGHT_DLU,
    MAIN_DIALOG_WIDTH_DLU, SAVED_COL_0_WIDTH, SAVED_COL_1_WIDTH, SAVED_COL_2_WIDTH,
};
use crate::models::{ActiveModal, ColorScheme, ProgramEntry, SortColumn, ViewMode};
use crate::services::cleaner::delete_uninstall_entry;
use crate::services::leftover::scan_leftover_traces;
use crate::services::scanner::{enumerate_desktop_programs, enumerate_store_apps};
use crate::services::uninstaller::{check_uninstaller_status, ActiveUninstaller, UninstallerStatus};
use crate::ui::components::menu::render_menu_bar;
use crate::ui::components::search::render_search_bar;
use crate::ui::components::status::render_status_bar;
use crate::ui::components::table::{
    render_column_headers, render_list_rows, render_scrollbar, TableLayout,
};
use crate::ui::components::title_bar::{render_simulated_title_bar, render_window_border};
use crate::ui::dialogs::modals::render_modals;
use crate::ui::handlers::shortcuts::{handle_keyboard_shortcuts, handle_list_navigation_and_scroll};
use crate::ui::platform::screenshot::save_color_image_to_png;

// ============================================================================
// Application State & UI
// ============================================================================

pub struct UninstallerApp {
    pub desktop_programs: Vec<ProgramEntry>,
    pub store_programs: Vec<ProgramEntry>,
    pub view_mode: ViewMode,
    pub color_scheme: ColorScheme,
    pub is_portuguese: bool,
    pub search_query: String,
    pub sort_column: SortColumn,
    pub sort_ascending: bool,
    pub selected_id: Option<String>,
    pub col_name_width: f32,
    pub col_size_width: f32,
    pub col_date_width: f32,
    pub active_modal: ActiveModal,
    pub active_uninstaller: Option<ActiveUninstaller>,
    pub screenshot_path: Option<PathBuf>,
    pub frame_count: u32,
    pub screenshot_requested: bool,
    pub status_banner: Option<String>,
    pub scroll_row_offset: usize,
    pub scroll_accum: f32,
}

impl UninstallerApp {
    pub fn new(screenshot_path: Option<PathBuf>) -> Self {
        let _ex_style = LISTVIEW_EX_STYLE;
        let _dlu = (MAIN_DIALOG_WIDTH_DLU, MAIN_DIALOG_HEIGHT_DLU);
        let _lv_specs = (LVM_GETITEMCOUNT_MSG, ICON_PATH_BUFFER_LEN);
        let desktop_programs = enumerate_desktop_programs();
        let store_programs = enumerate_store_apps();

        // Default sort from prefs.xml: InitialSortParam=2 (InstalledOn), InitialSortOrder=0 (Descending)
        let mut app = Self {
            desktop_programs,
            store_programs,
            view_mode: ViewMode::DesktopApps,
            color_scheme: ColorScheme::Light,
            is_portuguese: true,
            search_query: String::new(),
            sort_column: SortColumn::InstalledOn,
            sort_ascending: false,
            selected_id: None,
            col_name_width: SAVED_COL_0_WIDTH,
            col_size_width: SAVED_COL_1_WIDTH,
            col_date_width: SAVED_COL_2_WIDTH,
            active_modal: ActiveModal::None,
            active_uninstaller: None,
            screenshot_path,
            frame_count: 0,
            screenshot_requested: false,
            status_banner: None,
            scroll_row_offset: 0,
            scroll_accum: 0.0,
        };

        let sorted = app.filtered_and_sorted();
        app.selected_id = sorted.first().map(|p| p.id.clone());
        app
    }

    pub fn refresh(&mut self) {
        self.desktop_programs = enumerate_desktop_programs();
        self.store_programs = enumerate_store_apps();
        let sorted = self.filtered_and_sorted();
        if self
            .selected_id
            .as_ref()
            .map(|id| !sorted.iter().any(|p| &p.id == id))
            .unwrap_or(true)
        {
            self.selected_id = sorted.first().map(|p| p.id.clone());
        }
    }

    pub fn reset_columns(&mut self) {
        let _defaults = (
            COLUMN_PROGRAM_NAME_WIDTH,
            COLUMN_SIZE_WIDTH,
            COLUMN_INSTALLED_ON_WIDTH,
        );
        self.col_name_width = SAVED_COL_0_WIDTH;
        self.col_size_width = SAVED_COL_1_WIDTH;
        self.col_date_width = SAVED_COL_2_WIDTH;
    }

    pub fn active_list(&self) -> &[ProgramEntry] {
        match self.view_mode {
            ViewMode::DesktopApps => &self.desktop_programs,
            ViewMode::StoreApps => &self.store_programs,
        }
    }

    pub fn filtered_and_sorted(&self) -> Vec<ProgramEntry> {
        let query = self.search_query.trim().to_lowercase();
        let mut items: Vec<ProgramEntry> = self
            .active_list()
            .iter()
            .filter(|p| {
                if query.is_empty() {
                    return true;
                }
                p.display_name.to_lowercase().contains(&query)
                    || p.publisher.to_lowercase().contains(&query)
                    || p.display_version.to_lowercase().contains(&query)
            })
            .cloned()
            .collect();

        items.sort_by(|a, b| {
            let ord = match self.sort_column {
                SortColumn::ProgramName => a
                    .display_name
                    .to_lowercase()
                    .cmp(&b.display_name.to_lowercase()),
                SortColumn::Size => a.size_bytes.cmp(&b.size_bytes),
                SortColumn::InstalledOn => a
                    .install_date_sort
                    .cmp(&b.install_date_sort)
                    .then_with(|| {
                        b.display_name
                            .to_lowercase()
                            .cmp(&a.display_name.to_lowercase())
                    }),
            };
            if self.sort_ascending {
                ord
            } else {
                ord.reverse()
            }
        });

        items
    }

    pub fn selected_program(&self) -> Option<ProgramEntry> {
        let id = self.selected_id.as_ref()?;
        self.active_list().iter().find(|p| &p.id == id).cloned()
    }

    pub fn on_uninstaller_finished(&mut self, program: ProgramEntry, _success: bool) {
        let traces = scan_leftover_traces(&program);
        if traces.is_empty() {
            let _ = delete_uninstall_entry(&program);
            self.desktop_programs.retain(|p| p.id != program.id);
            self.store_programs.retain(|p| p.id != program.id);
            if self.selected_id.as_deref() == Some(program.id.as_str()) {
                self.selected_id = self.filtered_and_sorted().first().map(|p| p.id.clone());
            }
            self.active_modal = ActiveModal::StatusMessage {
                title: "Desinstalação Concluída".to_string(),
                message: format!(
                    "\"{}\" foi desinstalado com sucesso!\nNenhum vestígio residual foi detectado no sistema.",
                    program.display_name
                ),
            };
        } else {
            self.active_modal = ActiveModal::LeftoverScanner {
                program,
                is_force_removal: false,
                traces,
                cleaned_summary: None,
            };
        }
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        match self.color_scheme {
            ColorScheme::Dark => ctx.set_visuals(egui::Visuals::dark()),
            ColorScheme::Light | ColorScheme::System => {
                let mut visuals = egui::Visuals::light();
                visuals.panel_fill = Color32::WHITE;
                visuals.window_fill = Color32::WHITE;
                visuals.extreme_bg_color = Color32::WHITE;
                ctx.set_visuals(visuals);
            }
        }
    }
}

impl eframe::App for UninstallerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_theme(ctx);
        handle_keyboard_shortcuts(self, ctx);

        // Check active background uninstaller status
        if let ActiveModal::UninstallingProgress {
            ref program,
            ref mut elapsed_secs,
        } = self.active_modal
        {
            if let Some(ref mut uninstaller) = self.active_uninstaller {
                match check_uninstaller_status(uninstaller) {
                    UninstallerStatus::Running { elapsed_secs: secs } => {
                        *elapsed_secs = secs;
                        ctx.request_repaint_after(std::time::Duration::from_millis(500));
                    }
                    UninstallerStatus::Finished { success } => {
                        self.active_uninstaller = None;
                        let prog = program.clone();
                        self.on_uninstaller_finished(prog, success);
                    }
                }
            } else {
                let prog = program.clone();
                self.on_uninstaller_finished(prog, true);
            }
        }

        let filtered_items = self.filtered_and_sorted();
        let total_size_bytes: u64 = filtered_items.iter().map(|p| p.size_bytes).sum();

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(Color32::WHITE))
            .show(ctx, |ui| {
                let full_rect = ui.max_rect();
                let w = full_rect.width();
                let h = full_rect.height();

                let is_screenshot = self.screenshot_path.is_some();
                let top_offset = if is_screenshot { 34.0 } else { 0.0 };

                // 1. Simulated OS Window Title Bar (when capturing screenshot)
                if is_screenshot {
                    render_simulated_title_bar(ui, ctx, full_rect);
                }

                // 2. Menu Bar
                render_menu_bar(self, ui, ctx, w, top_offset);

                // 3 & 4. Table Layout, Scrolling, Column Headers, Rows & Scrollbar
                let layout = TableLayout::compute(self, top_offset, h, filtered_items.len());
                handle_list_navigation_and_scroll(
                    self,
                    ctx,
                    &filtered_items,
                    layout.max_visible_rows,
                    layout.max_scroll,
                );
                render_column_headers(self, ui, &layout);
                render_list_rows(self, ui, &layout, &filtered_items);
                render_scrollbar(self, ui, &layout, w);

                // 5. Full-Width Search Filter Bar
                render_search_bar(self, ui, ctx, w, h);

                // 6. Bottom Status Bar
                render_status_bar(self, ui, w, h, &filtered_items, total_size_bytes);

                // Outer border in screenshot mode
                if is_screenshot {
                    render_window_border(ui, full_rect);
                }
            });

        // Modals & Dialogs
        render_modals(self, ctx);

        // Screenshot Automation (--screenshot <file.png>)
        if let Some(shot_path) = self.screenshot_path.clone() {
            self.frame_count = self.frame_count.saturating_add(1);

            let mut captured_img: Option<std::sync::Arc<egui::ColorImage>> = None;
            ctx.input(|i| {
                for event in &i.events {
                    if let egui::Event::Screenshot { image, .. } = event {
                        captured_img = Some(image.clone());
                    }
                }
            });

            if let Some(color_img) = captured_img {
                save_color_image_to_png(&color_img, &shot_path);
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                std::process::exit(0);
            }

            if self.frame_count >= 115 && !self.screenshot_requested {
                self.screenshot_requested = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default()));
            }

            if self.frame_count >= 122 {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                std::process::exit(0);
            }

            ctx.request_repaint();
        }
    }
}
