use serde::{Deserialize, Serialize};

use crate::utils::formatting::format_bytes_str;

// ============================================================================
// Domain Models
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewMode {
    DesktopApps,
    StoreApps,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorScheme {
    System,
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortColumn {
    ProgramName,
    Size,
    InstalledOn,
}

#[derive(Clone, Debug, Default)]
pub struct CachedEntry {
    pub location: String,
    pub icon_path: String,
    pub size_kb: u64,
}

#[derive(Clone, Debug)]
pub struct ProgramEntry {
    pub id: String,
    pub reg_key_name: String,
    pub display_name: String,
    pub display_version: String,
    pub publisher: String,
    pub install_date: String,
    pub install_date_sort: i64,
    pub is_recent: bool,
    pub size_bytes: u64,
    pub install_location: String,
    pub uninstall_string: String,
    pub modify_path: String,
    pub no_modify: bool,
    pub reg_root_name: String,
    pub reg_subkey_path: String,
    pub url_info_about: String,
    pub is_32bit: bool,
    pub is_msi: bool,
    pub is_store_app: bool,
    pub store_pfn: String,
    pub main_exe_name: String,
}

impl ProgramEntry {
    pub fn display_label(&self) -> String {
        if self.is_32bit && !self.is_store_app && !self.display_name.contains("(32-bit)") {
            format!("{} (32-bit)", self.display_name)
        } else {
            self.display_name.clone()
        }
    }

    pub fn formatted_size(&self, use_comma: bool) -> String {
        format_bytes_str(self.size_bytes, use_comma)
    }

    pub fn full_reg_path(&self) -> String {
        format!("{}\\{}", self.reg_root_name, self.reg_subkey_path)
    }
}

#[derive(Clone, Debug)]
pub struct TraceItem {
    pub category: TraceCategory,
    pub path: String,
    pub description: String,
    pub checked: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraceCategory {
    FileSystem,
    Registry,
}

#[derive(Clone, Debug)]
pub enum ActiveModal {
    None,
    About,
    ConfirmUninstall {
        program: ProgramEntry,
    },
    ConfirmRemoveEntry {
        program: ProgramEntry,
    },
    LeftoverScanner {
        program: ProgramEntry,
        is_force_removal: bool,
        traces: Vec<TraceItem>,
        cleaned_count: Option<usize>,
    },
    StatusMessage {
        title: String,
        message: String,
    },
}

