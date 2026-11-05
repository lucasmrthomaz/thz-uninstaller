use std::fs;
use std::path::Path;
use std::process::Command;

use winreg::enums::*;
use winreg::RegKey;

use crate::models::{ActiveModal, ProgramEntry, ViewMode};
use crate::services::leftover::scan_leftover_traces;
use crate::services::reporter::generate_html_report;
use crate::ui::app::UninstallerApp;

impl UninstallerApp {
    pub fn trigger_uninstall(&mut self, program: ProgramEntry) {
        self.active_modal = ActiveModal::ConfirmUninstall { program };
    }

    pub fn trigger_force_removal(&mut self, program: ProgramEntry) {
        let traces = scan_leftover_traces(&program);
        self.active_modal = ActiveModal::LeftoverScanner {
            program,
            is_force_removal: true,
            traces,
            cleaned_count: None,
        };
    }

    pub fn trigger_remove_entry(&mut self, program: ProgramEntry) {
        self.active_modal = ActiveModal::ConfirmRemoveEntry { program };
    }

    pub fn open_registry_entry(&mut self, program: &ProgramEntry) {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok((regedit_key, _)) =
            hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Regedit")
        {
            let _ = regedit_key.set_value("LastKey", &program.full_reg_path());
        }
        let _ = Command::new("regedit.exe").spawn();
        self.status_banner = Some(format!("Regedit: {}", program.full_reg_path()));
    }

    pub fn open_installation_folder(&mut self, program: &ProgramEntry) {
        if !program.install_location.is_empty() && Path::new(&program.install_location).exists() {
            let _ = Command::new("explorer.exe")
                .arg(&program.install_location)
                .spawn();
        } else {
            self.active_modal = ActiveModal::StatusMessage {
                title: "Installation Folder".to_string(),
                message: "Installation folder is not specified or does not exist on disk."
                    .to_string(),
            };
        }
    }

    pub fn open_website_or_store(&mut self, program: &ProgramEntry) {
        if program.is_store_app && !program.store_pfn.is_empty() {
            let uri = format!("ms-windows-store:PDP?PFN={}", program.store_pfn);
            let _ = open::that(uri);
        } else if !program.url_info_about.is_empty() {
            let _ = open::that(&program.url_info_about);
        } else {
            self.google_for(&program.display_name);
        }
    }

    pub fn google_for(&mut self, term: &str) {
        if !term.trim().is_empty() {
            let encoded: String = term
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { '+' })
                .collect();
            let url = format!("https://www.google.com/search?q={}", encoded);
            let _ = open::that(url);
        }
    }

    pub fn export_html(&mut self) {
        let items = self.filtered_and_sorted();
        let refs: Vec<&ProgramEntry> = items.iter().collect();
        let total_bytes: u64 = items.iter().map(|p| p.size_bytes).sum();
        let html = generate_html_report(&refs, total_bytes, self.is_portuguese);

        let default_name = match self.view_mode {
            ViewMode::DesktopApps => "installed_programs.html",
            ViewMode::StoreApps => "store_apps.html",
        };

        if let Some(path) = rfd::FileDialog::new()
            .set_file_name(default_name)
            .add_filter("HTML Files (*.html)", &["html", "htm"])
            .save_file()
        {
            if fs::write(&path, html).is_ok() {
                let _ = open::that(&path);
            }
        }
    }
}

