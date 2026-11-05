use std::env;
use std::fs;
use std::path::Path;

use winreg::enums::*;
use winreg::RegKey;

use crate::models::{ProgramEntry, TraceCategory, TraceItem};

// ============================================================================
// Leftover Traces Scanner (RT_DIALOG 219 / functions 0x441400, 0x4431cc)
// ============================================================================

pub fn format_registry_trace_path(root_code: i32, subkey: &str) -> String {
    let prefix = match root_code {
        -0x7fffffff => "HKEY_CURRENT_USER\\",
        -0x7ffffffe => "HKEY_LOCAL_MACHINE\\",
        -0x80000000 => "HKEY_CLASSES_ROOT\\",
        -0x7ffffffd => "HKEY_USERS\\",
        _ => "HKEY_LOCAL_MACHINE\\",
    };
    format!("{}{}", prefix, subkey)
}

pub fn scan_leftover_traces(program: &ProgramEntry) -> Vec<TraceItem> {
    let mut traces: Vec<TraceItem> = Vec::new();

    let root_code = match program.reg_root_name.as_str() {
        "HKEY_CURRENT_USER" => -0x7fffffff,
        "HKEY_CLASSES_ROOT" => -0x80000000,
        "HKEY_USERS" => -0x7ffffffd,
        _ => -0x7ffffffe,
    };
    traces.push(TraceItem {
        category: TraceCategory::Registry,
        path: format_registry_trace_path(root_code, &program.reg_subkey_path),
        description: "Uninstall Registry Key".to_string(),
        checked: true,
    });

    let clean_name: String = program
        .display_name
        .split('(')
        .next()
        .unwrap_or(&program.display_name)
        .trim()
        .to_string();
    let token_lower = clean_name.to_lowercase();
    let pub_token = program
        .publisher
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_matches(|c: char| !c.is_alphanumeric())
        .to_lowercase();

    if token_lower.len() >= 3 {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(sw) = hkcu.open_subkey_with_flags("Software", KEY_READ) {
            for key_name in sw.enum_keys().flatten() {
                let kl = key_name.to_lowercase();
                if kl.contains(&token_lower)
                    || (!pub_token.is_empty() && pub_token.len() >= 4 && kl == pub_token)
                {
                    traces.push(TraceItem {
                        category: TraceCategory::Registry,
                        path: format_registry_trace_path(
                            -0x7fffffff,
                            &format!("Software\\{}", key_name),
                        ),
                        description: "Application Settings Key".to_string(),
                        checked: true,
                    });
                }
            }
        }
    }

    if !program.install_location.is_empty() {
        let install_path = Path::new(&program.install_location);
        if install_path.exists() {
            traces.push(TraceItem {
                category: TraceCategory::FileSystem,
                path: program.install_location.clone(),
                description: "Installation Folder".to_string(),
                checked: true,
            });
        }
    }

    let env_dirs = ["APPDATA", "LOCALAPPDATA", "ProgramData"];
    for var in env_dirs {
        if let Ok(base_dir) = env::var(var) {
            if let Ok(entries) = fs::read_dir(&base_dir) {
                for entry in entries.flatten() {
                    let fname = entry.file_name().to_string_lossy().to_string();
                    let fl = fname.to_lowercase();
                    if (token_lower.len() >= 3 && fl.contains(&token_lower))
                        || (!pub_token.is_empty() && pub_token.len() >= 4 && fl == pub_token)
                    {
                        traces.push(TraceItem {
                            category: TraceCategory::FileSystem,
                            path: entry.path().to_string_lossy().to_string(),
                            description: format!("{} Application Data", var),
                            checked: true,
                        });
                    }
                }
            }
        }
    }

    traces
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_registry_trace_path() {
        assert_eq!(
            format_registry_trace_path(-0x7fffffff, "Software\\App"),
            "HKEY_CURRENT_USER\\Software\\App"
        );
        assert_eq!(
            format_registry_trace_path(-0x7ffffffe, "Software\\App"),
            "HKEY_LOCAL_MACHINE\\Software\\App"
        );
    }
}

