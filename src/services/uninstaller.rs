use std::process::{Child, Command};
use std::time::Instant;

use crate::models::ProgramEntry;

pub struct ActiveUninstaller {
    pub child: Option<Child>,
    pub program_name: String,
    pub start_time: Instant,
    pub is_store_app: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UninstallerStatus {
    Running { elapsed_secs: u64 },
    Finished { success: bool },
}

pub fn launch_uninstaller(program: &ProgramEntry) -> Result<ActiveUninstaller, String> {
    let program_name = program.display_name.clone();
    let start_time = Instant::now();

    if program.is_store_app {
        let child = Command::new("powershell.exe")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-WindowStyle",
                "Normal",
                "-Command",
                &format!("Remove-AppxPackage -Package '{}'", program.reg_key_name),
            ])
            .spawn()
            .map_err(|e| format!("Falha ao iniciar PowerShell para Store App: {}", e))?;

        return Ok(ActiveUninstaller {
            child: Some(child),
            program_name,
            start_time,
            is_store_app: true,
        });
    }

    let cmd_str = program.uninstall_string.trim();
    if cmd_str.is_empty() {
        return Err("Comando de desinstalação não especificado pelo programa.".to_string());
    }

    let child = spawn_uninstaller_command(cmd_str)
        .map_err(|e| format!("Falha ao iniciar desinstalador: {}", e))?;

    Ok(ActiveUninstaller {
        child: Some(child),
        program_name,
        start_time,
        is_store_app: false,
    })
}

fn spawn_uninstaller_command(cmd_str: &str) -> std::io::Result<Child> {
    let trimmed = cmd_str.trim();

    // 1. Quoted executable path e.g. "C:\Program Files\App\unins000.exe" /SILENT
    if trimmed.starts_with('"') {
        if let Some(end_quote) = trimmed[1..].find('"') {
            let exe_path = &trimmed[1..=end_quote];
            let rest = trimmed[end_quote + 2..].trim();
            let mut cmd = Command::new(exe_path);
            if !rest.is_empty() {
                cmd.args(split_arguments(rest));
            }
            if let Ok(c) = cmd.spawn() {
                return Ok(c);
            }
        }
    }

    // 2. Direct MSI command synthesis (msiexec.exe /X{GUID})
    let lower = trimmed.to_lowercase();
    if lower.starts_with("msiexec") {
        let parts = split_arguments(trimmed);
        if !parts.is_empty() {
            let mut cmd = Command::new("msiexec.exe");
            cmd.args(&parts[1..]);
            if let Ok(c) = cmd.spawn() {
                return Ok(c);
            }
        }
    }

    // 3. Simple space separated command without quotes
    let parts = split_arguments(trimmed);
    if let Some(first) = parts.first() {
        if std::path::Path::new(first).exists() {
            let mut cmd = Command::new(first);
            cmd.args(&parts[1..]);
            if let Ok(c) = cmd.spawn() {
                return Ok(c);
            }
        }
    }

    // 4. Shell fallback via cmd.exe /C
    Command::new("cmd.exe")
        .args(["/C", trimmed])
        .spawn()
}

fn split_arguments(args_str: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in args_str.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
            }
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        args.push(current);
    }

    args
}

pub fn check_uninstaller_status(uninstaller: &mut ActiveUninstaller) -> UninstallerStatus {
    if let Some(ref mut child) = uninstaller.child {
        match child.try_wait() {
            Ok(Some(status)) => UninstallerStatus::Finished {
                success: status.success(),
            },
            Ok(None) => UninstallerStatus::Running {
                elapsed_secs: uninstaller.start_time.elapsed().as_secs(),
            },
            Err(_) => UninstallerStatus::Finished { success: false },
        }
    } else {
        UninstallerStatus::Finished { success: true }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_arguments_quotes_and_spaces() {
        let args = split_arguments("\"C:\\Program Files\\App\\unins.exe\" /SILENT /NORESTART");
        assert_eq!(
            args,
            vec!["C:\\Program Files\\App\\unins.exe", "/SILENT", "/NORESTART"]
        );
    }
}
