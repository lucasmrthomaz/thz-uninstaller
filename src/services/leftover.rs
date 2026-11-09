use std::collections::HashSet;
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

fn registry_key_exists(hive: winreg::HKEY, subkey: &str, is_wow64_32: bool) -> bool {
    let root = RegKey::predef(hive);
    let flags = if is_wow64_32 {
        KEY_READ | KEY_WOW64_32KEY
    } else {
        KEY_READ | KEY_WOW64_64KEY
    };
    root.open_subkey_with_flags(subkey, flags).is_ok()
}

pub fn scan_leftover_traces(program: &ProgramEntry) -> Vec<TraceItem> {
    let mut traces: Vec<TraceItem> = Vec::new();
    let mut seen_paths: HashSet<String> = HashSet::new();

    let root_code = match program.reg_root_name.as_str() {
        "HKEY_CURRENT_USER" => -0x7fffffff,
        "HKEY_CLASSES_ROOT" => -0x80000000,
        "HKEY_USERS" => -0x7ffffffd,
        _ => -0x7ffffffe,
    };

    let hive = match root_code {
        -0x7fffffff => HKEY_CURRENT_USER,
        -0x80000000 => HKEY_CLASSES_ROOT,
        -0x7ffffffd => HKEY_USERS,
        _ => HKEY_LOCAL_MACHINE,
    };

    // 1. Uninstall registry key (if it still exists in the registry)
    if registry_key_exists(hive, &program.reg_subkey_path, program.is_32bit) {
        let path = format_registry_trace_path(root_code, &program.reg_subkey_path);
        if seen_paths.insert(path.to_lowercase()) {
            traces.push(TraceItem {
                category: TraceCategory::Registry,
                path,
                description: "Chave de Desinstalação no Registro".to_string(),
                checked: true,
            });
        }
    }

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

    // 2. Scan Registry Software keys (HKCU and HKLM)
    if token_lower.len() >= 3 {
        let registry_targets = [
            (HKEY_CURRENT_USER, "Software", -0x7fffffff, false),
            (HKEY_LOCAL_MACHINE, "SOFTWARE", -0x7ffffffe, false),
            (HKEY_LOCAL_MACHINE, "SOFTWARE\\WOW6432Node", -0x7ffffffe, true),
        ];

        for &(hive, base_subkey, root_code, wow32) in &registry_targets {
            let root = RegKey::predef(hive);
            let flags = if wow32 {
                KEY_READ | KEY_WOW64_32KEY
            } else {
                KEY_READ | KEY_WOW64_64KEY
            };

            if let Ok(sw) = root.open_subkey_with_flags(base_subkey, flags) {
                for key_name in sw.enum_keys().flatten() {
                    let kl = key_name.to_lowercase();
                    if kl.contains(&token_lower)
                        || (!pub_token.is_empty() && pub_token.len() >= 4 && kl == pub_token)
                    {
                        let subkey_full = format!("{}\\{}", base_subkey, key_name);
                        let path = format_registry_trace_path(root_code, &subkey_full);
                        if seen_paths.insert(path.to_lowercase()) {
                            traces.push(TraceItem {
                                category: TraceCategory::Registry,
                                path,
                                description: "Configurações da Aplicação no Registro".to_string(),
                                checked: true,
                            });
                        }
                    }
                }
            }
        }
    }

    // 3. Installation Folder
    if !program.install_location.is_empty() {
        let install_path = Path::new(&program.install_location);
        if install_path.exists() {
            let path = program.install_location.clone();
            if seen_paths.insert(path.to_lowercase()) {
                traces.push(TraceItem {
                    category: TraceCategory::FileSystem,
                    path,
                    description: "Pasta de Instalação Residual".to_string(),
                    checked: true,
                });
            }
        }
    }

    // 4. Application Data folders (%APPDATA%, %LOCALAPPDATA%, %ProgramData%)
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
                        let path = entry.path().to_string_lossy().to_string();
                        if seen_paths.insert(path.to_lowercase()) {
                            traces.push(TraceItem {
                                category: TraceCategory::FileSystem,
                                path,
                                description: format!("Dados Residuais em {}", var),
                                checked: true,
                            });
                        }
                    }
                }
            }
        }
    }

    // 5. Start Menu & Desktop shortcuts
    let shortcut_folders = [
        env::var("APPDATA").map(|a| Path::new(&a).join("Microsoft\\Windows\\Start Menu\\Programs")),
        env::var("ProgramData").map(|p| Path::new(&p).join("Microsoft\\Windows\\Start Menu\\Programs")),
        env::var("USERPROFILE").map(|u| Path::new(&u).join("Desktop")),
        env::var("PUBLIC").map(|p| Path::new(&p).join("Desktop")),
    ];

    for folder_opt in shortcut_folders.into_iter().flatten() {
        if let Ok(entries) = fs::read_dir(folder_opt) {
            for entry in entries.flatten() {
                let fname = entry.file_name().to_string_lossy().to_string();
                let fl = fname.to_lowercase();
                if fl.contains(&token_lower) && (fl.ends_with(".lnk") || entry.path().is_dir()) {
                    let path = entry.path().to_string_lossy().to_string();
                    if seen_paths.insert(path.to_lowercase()) {
                        traces.push(TraceItem {
                            category: TraceCategory::FileSystem,
                            path,
                            description: "Atalho Residual".to_string(),
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
