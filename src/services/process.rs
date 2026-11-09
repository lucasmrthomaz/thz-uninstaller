use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::Path;

use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};

use crate::models::ProgramEntry;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunningProcess {
    pub pid: u32,
    pub name: String,
}

const SYSTEM_PROCESS_EXCLUSIONS: &[&str] = &[
    "system",
    "registry",
    "smss.exe",
    "csrss.exe",
    "wininit.exe",
    "services.exe",
    "lsass.exe",
    "svchost.exe",
    "fontdrvhost.exe",
    "winlogon.exe",
    "dwm.exe",
    "explorer.exe",
    "taskhostw.exe",
    "cmd.exe",
    "powershell.exe",
    "conhost.exe",
    "thz-uninstaller.exe",
    "geek-uninstaller-open.exe",
    "cargo.exe",
    "rustc.exe",
];

pub fn list_all_processes() -> Vec<RunningProcess> {
    let mut list = Vec::new();
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return list;
        }

        let mut entry: PROCESSENTRY32W = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        if Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                let null_pos = entry
                    .szExeFile
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(entry.szExeFile.len());
                let exe_name = OsString::from_wide(&entry.szExeFile[..null_pos])
                    .to_string_lossy()
                    .to_string();

                if entry.th32ProcessID > 4 {
                    list.push(RunningProcess {
                        pid: entry.th32ProcessID,
                        name: exe_name,
                    });
                }

                if Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);
    }
    list
}

pub fn find_program_processes(program: &ProgramEntry) -> Vec<RunningProcess> {
    let all = list_all_processes();
    let mut matched = Vec::new();

    let main_exe = program.main_exe_name.to_lowercase();
    let clean_name = program
        .display_name
        .split('(')
        .next()
        .unwrap_or(&program.display_name)
        .trim()
        .to_lowercase();

    let name_tokens: Vec<&str> = clean_name
        .split_whitespace()
        .filter(|t| t.len() >= 3)
        .collect();

    for proc in all {
        let proc_lower = proc.name.to_lowercase();

        // Skip core OS / helper processes
        if SYSTEM_PROCESS_EXCLUSIONS
            .iter()
            .any(|&ex| proc_lower.eq_ignore_ascii_case(ex))
        {
            continue;
        }

        let mut is_match = false;

        // 1. Direct match with main_exe_name
        if !main_exe.is_empty() && proc_lower == main_exe {
            is_match = true;
        }

        // 2. Exact match with exe stripped (e.g. "discord.exe" -> "discord")
        let proc_stem = proc_lower.trim_end_matches(".exe");
        if proc_stem.len() >= 3 && clean_name.starts_with(proc_stem) {
            is_match = true;
        }

        // 3. Tokens match (e.g. "Google Chrome" matching "chrome.exe")
        for token in &name_tokens {
            if proc_stem == *token {
                is_match = true;
                break;
            }
        }

        // 4. If install_location exists and exe exists inside that folder
        if !is_match && !program.install_location.is_empty() {
            let possible_file = Path::new(&program.install_location).join(&proc.name);
            if possible_file.exists() {
                is_match = true;
            }
        }

        if is_match {
            matched.push(proc);
        }
    }

    matched
}

pub fn kill_process(pid: u32) -> Result<(), String> {
    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if handle.is_null() {
            return Err(format!("Falha ao abrir processo PID {}: Acesso negado ou processo já encerrado", pid));
        }

        let res = TerminateProcess(handle, 1);
        CloseHandle(handle);

        if res == 0 {
            Err(format!("Falha ao encerrar processo PID {}", pid))
        } else {
            Ok(())
        }
    }
}

pub fn kill_processes(processes: &[RunningProcess]) -> (usize, usize) {
    let mut success = 0;
    let mut failed = 0;
    for p in processes {
        if kill_process(p.pid).is_ok() {
            success += 1;
        } else {
            failed += 1;
        }
    }
    (success, failed)
}
