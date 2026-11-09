use std::fs;
use std::path::Path;

use winreg::enums::*;
use winreg::RegKey;

use crate::models::{ProgramEntry, TraceCategory, TraceItem};
use crate::services::cache::remove_cache_entry;

#[derive(Clone, Debug, Default)]
pub struct CleanupSummary {
    pub registry_deleted: usize,
    pub filesystem_deleted: usize,
    pub errors: Vec<String>,
}

impl CleanupSummary {
    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn total_deleted(&self) -> usize {
        self.registry_deleted + self.filesystem_deleted
    }
}

pub fn delete_registry_path(path: &str) -> Result<(), String> {
    let (hive, subkey_path, is_wow64_32) = parse_registry_path(path)?;

    let root = RegKey::predef(hive);
    let wow_flag = if is_wow64_32 {
        KEY_WOW64_32KEY
    } else {
        KEY_WOW64_64KEY
    };

    let flags_try = [
        KEY_ALL_ACCESS | wow_flag,
        KEY_WRITE | KEY_READ | wow_flag,
        KEY_WRITE | wow_flag,
    ];

    let (parent_path, target_name) = match subkey_path.rsplit_once('\\') {
        Some((p, t)) => (p, t),
        None => ("", subkey_path.as_str()),
    };

    if parent_path.is_empty() {
        match root.delete_subkey_all(target_name) {
            Ok(_) => return Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(format!("Falha ao excluir chave de registro '{}': {}", path, e)),
        }
    }

    let mut parent_key = None;
    for &flags in &flags_try {
        if let Ok(k) = root.open_subkey_with_flags(parent_path, flags) {
            parent_key = Some(k);
            break;
        }
    }

    let parent_key = match parent_key {
        Some(k) => k,
        None => {
            // If parent key doesn't even exist, target is already gone
            if root.open_subkey_with_flags(parent_path, KEY_READ | wow_flag).is_err() {
                return Ok(());
            }
            return Err(format!("Acesso negado ou erro ao abrir chave pai '{}'", parent_path));
        }
    };

    match parent_key.delete_subkey_all(target_name) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(format!("Falha ao excluir subchave '{}' em '{}': {}", target_name, parent_path, e)),
    }
}

fn parse_registry_path(full_path: &str) -> Result<(winreg::HKEY, String, bool), String> {
    let clean = full_path.trim().trim_start_matches("\\\\?\\");
    let (prefix, subkey) = clean.split_once('\\').ok_or_else(|| {
        format!("Caminho do Registro inválido: '{}'", full_path)
    })?;

    let is_wow64_32 = subkey.to_lowercase().contains("wow6432node");

    let hive = match prefix.to_uppercase().as_str() {
        "HKEY_LOCAL_MACHINE" | "HKLM" => HKEY_LOCAL_MACHINE,
        "HKEY_CURRENT_USER" | "HKCU" => HKEY_CURRENT_USER,
        "HKEY_CLASSES_ROOT" | "HKCR" => HKEY_CLASSES_ROOT,
        "HKEY_USERS" | "HKU" => HKEY_USERS,
        _ => return Err(format!("Hive desconhecido no Registro: '{}'", prefix)),
    };

    Ok((hive, subkey.to_string(), is_wow64_32))
}

pub fn delete_filesystem_path(path_str: &str) -> Result<(), String> {
    let path = Path::new(path_str);
    if !path.exists() {
        return Ok(());
    }

    if path.is_dir() {
        remove_dir_all_force(path)
            .map_err(|e| format!("Falha ao remover pasta '{}': {}", path_str, e))
    } else {
        if let Ok(meta) = path.metadata() {
            let mut perms = meta.permissions();
            if perms.readonly() {
                perms.set_readonly(false);
                let _ = fs::set_permissions(path, perms);
            }
        }
        fs::remove_file(path)
            .map_err(|e| format!("Falha ao remover arquivo '{}': {}", path_str, e))
    }
}

fn remove_dir_all_force(dir: &Path) -> std::io::Result<()> {
    if !dir.is_dir() {
        return fs::remove_file(dir);
    }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if let Ok(meta) = entry_path.metadata() {
                let mut perms = meta.permissions();
                if perms.readonly() {
                    perms.set_readonly(false);
                    let _ = fs::set_permissions(&entry_path, perms);
                }
            }
            if entry_path.is_dir() {
                let _ = remove_dir_all_force(&entry_path);
            } else {
                let _ = fs::remove_file(&entry_path);
            }
        }
    }

    if let Ok(meta) = dir.metadata() {
        let mut perms = meta.permissions();
        if perms.readonly() {
            perms.set_readonly(false);
            let _ = fs::set_permissions(dir, perms);
        }
    }

    fs::remove_dir_all(dir)
}

pub fn delete_uninstall_entry(program: &ProgramEntry) -> Result<(), String> {
    if program.is_store_app {
        // Store apps: Remove via PowerShell package command
        let out = std::process::Command::new("powershell.exe")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-WindowStyle",
                "Hidden",
                "-Command",
                &format!("Remove-AppxPackage -Package '{}'", program.reg_key_name),
            ])
            .output();

        if let Ok(o) = out {
            if !o.status.success() {
                let err_msg = String::from_utf8_lossy(&o.stderr).trim().to_string();
                if !err_msg.is_empty() {
                    return Err(format!("Erro ao desregistrar pacote Store: {}", err_msg));
                }
            }
        }
    }

    // Delete uninstall registry key
    let reg_path = program.full_reg_path();
    let reg_res = delete_registry_path(&reg_path);

    // Also remove from cache.dat
    let _ = remove_cache_entry(&program.reg_key_name);

    reg_res
}

pub fn clean_traces(
    traces: &[TraceItem],
    program: Option<&ProgramEntry>,
    is_force_removal: bool,
) -> CleanupSummary {
    let mut summary = CleanupSummary::default();

    for item in traces.iter().filter(|t| t.checked) {
        match item.category {
            TraceCategory::Registry => {
                match delete_registry_path(&item.path) {
                    Ok(_) => summary.registry_deleted += 1,
                    Err(e) => summary.errors.push(e),
                }
            }
            TraceCategory::FileSystem => {
                match delete_filesystem_path(&item.path) {
                    Ok(_) => summary.filesystem_deleted += 1,
                    Err(e) => summary.errors.push(e),
                }
            }
        }
    }

    if is_force_removal {
        if let Some(p) = program {
            let _ = delete_uninstall_entry(p);
        }
    }

    summary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_filesystem_path_file_and_dir() {
        let temp_dir = std::env::temp_dir().join("thz_cleaner_test");
        let sub_dir = temp_dir.join("nested");
        fs::create_dir_all(&sub_dir).unwrap();

        let file_path = sub_dir.join("test.txt");
        fs::write(&file_path, "test data").unwrap();

        assert!(file_path.exists());
        assert!(delete_filesystem_path(file_path.to_str().unwrap()).is_ok());
        assert!(!file_path.exists());

        // Test directory deletion with files inside
        let file_path2 = sub_dir.join("test2.txt");
        fs::write(&file_path2, "more data").unwrap();
        assert!(delete_filesystem_path(temp_dir.to_str().unwrap()).is_ok());
        assert!(!temp_dir.exists());
    }

    #[test]
    fn test_delete_registry_path_roundtrip() {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok((test_parent, _)) = hkcu.create_subkey("Software\\THZTestKey") {
            let (sub, _) = test_parent.create_subkey("ChildKey").unwrap();
            sub.set_value("TestVal", &"hello").unwrap();

            let full_path = "HKEY_CURRENT_USER\\Software\\THZTestKey\\ChildKey";
            assert!(delete_registry_path(full_path).is_ok());

            // Verify child key was deleted
            assert!(test_parent.open_subkey("ChildKey").is_err());

            // Clean up parent
            let _ = hkcu.delete_subkey("Software\\THZTestKey");
        }
    }
}
