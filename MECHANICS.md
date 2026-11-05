# Application Mechanics & Decompiled Specification

This document records the core constants, formulas, registry structures, UI specifications, and behavioral rules recovered from `C:\geek.exe` (SHA-256: `fe941fd1eea4da9d626773057bd81a713fa34721f8bd01c831c5fcae72a3e768`) via static PE resource extraction and Ghidra decompilation (`native_function_pseudocode`).

---

## 1. Window & UI Layout (`CMainDialog` & `CDlgUninstaller`)

### 1.1 Main Dialog & Viewport Dimensions
- **Main Dialog Template (`RT_DIALOG` ID `105`, initialized in `function 0x420a3f`):**
  - Dialog dimensions in Dialog Units (DLU): `321` width (`0x0141`) x `199` height (`0x00c7`).
  - Window placement persistence (`function 0x421430` / `function 0x41fade`): reads and saves `Window_Rect` and `Window_Maximized` in `%APPDATA%\Geek Uninstaller\prefs.xml` (`function 0x4232ea`).
  - Rendered window pixel dimensions from `Window_Rect` (`function 0x421430` / `reference/picture-1.png`): `579` px width (`default_window_width`) x `750` px height (`default_window_height`).

### 1.2 Program List View (`SysListView32`, ID `1182` / `0x049e`)
- **Extended ListView Style (`function 0x42afd2`):**
  - `listview_ex_style = 65568` (`0x10020` = `LVS_EX_FULLROWSELECT | LVS_EX_DOUBLEBUFFER`).
- **Columns & Pixel Widths (`function 0x42afd2`):**
  - Column widths are packed as `(visible << 16) | width_px` in `column_0`, `column_1`, `column_2`:
  - Column 0 (`ListCtrl/columnProgramName`, `"Program Name"` / `"Nome do Programa"`): initial fallback width = `200` px (`0xc8`), persisted active width = `373` px (`65909 & 0xFFFF`). Appends `" (32-bit)"` (`ListCtrl/mark32bitItem`) in gray for 32-bit programs on 64-bit Windows (`function 0x42d1e6`).
  - Column 1 (`ListCtrl/columnSize`, `"Size"` / `"Tamanho"`): initial fallback width = `70` px (`0x46`), persisted active width = `73` px (`65609 & 0xFFFF`).
  - Column 2 (`ListCtrl/columnInstallationDate`, `"Installed On"` / `"Instalado Em"`): default and persisted width = `110` px (`0x6e` / `65646 & 0xFFFF`).
  - List row height: `24` px (`list_row_height`).
- **Sorting & Highlighting (`function 0x42d1e6`, `function 0x42d54c`, `function 0x4232ea`):**
  - Persists `InitialSortParam` (default `2` = Installed On) and `InitialSortOrder` (`0` = descending).
  - Tracks `LastRunDate` in `HKCU\Software\Geek Uninstaller` (`function 0x4232ea`); recently installed items are highlighted in peach/orange (`RGB(255, 234, 191)` fill, `RGB(255, 183, 128)` border), selected rows in sky blue (`RGB(209, 232, 255)` fill, `RGB(102, 167, 232)` border), and remaining rows alternate between `RGB(226, 229, 234)` and `RGB(240, 240, 240)`.

### 1.3 Search Filter & Status Bar
- **Search Filter (`CFilterEdit`, ID `502` / `0x01f6`, `function 0x42aed2`, `function 0x42dfa2`, `function 0x42bbc9`):**
  - Placeholder text (`ExplorerBar/itemShortSearchTip` in XML resource `242` at `0x6e8330`): `"Type to find a program"` (Portuguese: `"Digite para encontrar um programa"`).
  - Pressing `ESC` (`0x1b` in `function 0x42c6f6` / `function 0x42d75e`) clears the filter string.
  - Queries visible item count via `SendMessageW(hListView, 0x1004, 0, 0)` (`LVM_GETITEMCOUNT = 4100` / `0x1004` at `this + 0x218` in `function 0x42bbc9`); when a filter matches 0 items (`function 0x42da75`), displays centered overlay text: `"No matching items, press ESC to reset"`.
- **ListView Scrolling & Custom Icon Drawing (`function 0x423794`, `function 0x4261c6`, `function 0x43ca5a`):**
  - `function 0x423794` (`FUN_00423737`) draws per-row program icons into an image list buffer (`0xfa` = `250` max path buffer) and updates row painting during scroll.
  - `function 0x4261c6` (`FUN_004261a1`) computes normalized floating-point ratios (`(index + 1) / total_count`) for scroll/progress positioning.
  - `function 0x43ca5a` (`FUN_0043ca04`) formats and dispatches shell/folder actions on the selected program entry.
- **Status Bar (`function 0x41f293`, `0x608f20`):**
  - Displays `"%d programs of %s size in total"` / `"%d programas com tamanho total de %s"` (`Misc/statusTotalInfo`).

---

## 2. Program Enumeration, Cache & Registry Rules (`CUninstaller` & `CUninstallCache`)

### 2.1 Registry Locations Scanned (`function 0x4378a1`)
1. `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall` (64-bit and 32-bit `KEY_WOW64_32KEY` / `KEY_WOW64_64KEY` views with `KEY_READ = 0x20019` via `function 0x495850`).
2. `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall`.
3. `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Installer\UserData\<UserSID>\Products\<GUID>\InstallProperties` (`function 0x436e8c`).
4. `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Installer\UserData\S-1-5-18\Products\<GUID>\InstallProperties` (`function 0x436e8c`).
5. Windows Store / AppX Packages (`function 0x558580`), enabled when OS version `>= 6.2` (`store_apps_min_os_major = 6`, `store_apps_min_os_minor = 2` checked via `VerifyVersionInfoW` in `function 0x4378a1`).

### 2.2 Uninstall Cache (`CUninstallCache::Load`, `function 0x4250cd`, `function 0x424f39`)
- Loads GZIP-compressed (`\x1f\x8b`) XML from `%APPDATA%\Geek Uninstaller\cache.dat` (`<UninstallCache version="3">`, `cache_version = 3`).
- Reads `<CacheItem key="..." location="..." iconPath="..." iconIndex="..." size="..." />` where `size` is in KB (`4294967295` / `0xFFFFFFFF` = unknown size, `cache_unknown_size = 4294967295`).
- Allocates a `16`-byte (`0x10`, `cache_item_struct_size = 16`) item structure in `function 0x424f39` storing `location` (`+0x0`), `iconPath` (`+0x4`), `size` (`+0x8`), and `iconIndex` (`+0xc`).

### 2.3 Per-Item Registry Values & Synthesis (`function 0x43725a`, `function 0x43dbb4`, `function 0x43d7c4`, `function 0x43dead`, `function 0x4367ce`)
- **MSI Command Synthesis (`function 0x43dbb4`):**
  - If `WindowsInstaller != 0` and `UninstallString` is empty, synthesizes `UninstallString` using prefix `"msiexec /X"` followed by `{GUID}`.
  - If `NoModify == 0` and `ModifyPath` is empty, synthesizes `ModifyPath` using prefix `"msiexec /I"` followed by `{GUID}`.
- **Install Date Resolution (`function 0x43d7c4`, `function 0x43dead`, `function 0x4367ce`):**
  - Queries registry key `LastWriteTime` (`FILETIME` via `RegQueryInfoKeyW` in `function 0x495580`), converts `FILETIME` to local `SYSTEMTIME` via `FileTimeToSystemTime` and `SystemTimeToTzSpecificLocalTime` in `function 0x4367ce` (raising `0x80070057` / `2147942487` `E_INVALIDARG` on invalid `FILETIME`), parses 8-character `InstallDate` (`install_date_string_len = 8`: `4` year chars, `2` month chars, `2` day chars in `function 0x43dead`), and checks `InstallLocation` directory timestamps (`function 0x43e0ea`).

---

## 3. Size Calculation & Leftover Trace Scanner (`CInstallLocationFinder` & `CScannerNormal`)

- **Main Executable Detection (`function 0x43180c`):**
  - Scans executables in the install folder to identify the primary binary by largest file size while excluding `"unins000.exe"` (`inno_uninstaller_exclude = "unins000.exe"`).
- **Leftover Trace Scanner (`RT_DIALOG` ID `219`, `function 0x441400`, `function 0x4431cc`):**
  - Formats registry trace paths (`HKEY_CURRENT_USER\`, `HKEY_LOCAL_MACHINE\`, `HKEY_CLASSES_ROOT\`, `HKEY_USERS\` in `function 0x4431cc`) and presents a checkable tree of `File System` and `Registry` leftovers.

---

## 4. HTML Export Specification (`function 0x42bbc9`, `0x60ad20`)

- Generates an HTML table with `Tahoma` font, header background `#ECE9D8` (`html_header_bg`), row height `34` px (`html_row_height`), and fixed column widths:
  - `Program Name`: `600` px (`html_col_name_width`)
  - `Size`: `80` px (`html_col_size_width`)
  - `Installed On`: `120` px (`html_col_date_width`)
