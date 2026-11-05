pub mod constants;
pub mod models;
pub mod services;
pub mod ui;
pub mod utils;

pub use constants::*;
pub use models::*;
pub use services::cache::load_uninstall_cache;
pub use services::leftover::{format_registry_trace_path, scan_leftover_traces};
pub use services::reporter::generate_html_report;
pub use services::scanner::{
    enumerate_desktop_programs, enumerate_store_apps, inspect_install_directory,
    query_regkey_unix_time, resolve_install_date,
};
pub use ui::app::UninstallerApp;
pub use utils::formatting::{format_bytes_str, format_total_bytes_str, parse_yyyymmdd};

