// ============================================================================
// Mechanics Constants (synchronized with app/MECHANICS.md & app/mechanics.json)
// ============================================================================

/// Default pixel width for Column 0 ("Program Name") from function 0x42afd2 (0xc8)
pub const COLUMN_PROGRAM_NAME_WIDTH: f32 = 200.0;
/// Default pixel width for Column 1 ("Size") from function 0x42afd2 (0x46)
pub const COLUMN_SIZE_WIDTH: f32 = 70.0;
/// Default pixel width for Column 2 ("Installed On") from function 0x42afd2 (0x6e)
pub const COLUMN_INSTALLED_ON_WIDTH: f32 = 110.0;

/// Saved pixel width for Column 0 from prefs.xml / function 0x42afd2 (65909 & 0xFFFF)
pub const SAVED_COL_0_WIDTH: f32 = 373.0;
/// Saved pixel width for Column 1 from prefs.xml / function 0x42afd2 (65609 & 0xFFFF)
pub const SAVED_COL_1_WIDTH: f32 = 73.0;
/// Saved pixel width for Column 2 from prefs.xml / function 0x42afd2 (65646 & 0xFFFF)
pub const SAVED_COL_2_WIDTH: f32 = 110.0;

/// ListView row height in pixels from function 0x42afd2
pub const LIST_ROW_HEIGHT: f32 = 24.0;

/// Main dialog template width in dialog units (DLU) from RT_DIALOG 105 / function 0x420a3f
pub const MAIN_DIALOG_WIDTH_DLU: u32 = 321;
/// Main dialog template height in dialog units (DLU) from RT_DIALOG 105 / function 0x420a3f
pub const MAIN_DIALOG_HEIGHT_DLU: u32 = 199;

/// Default window pixel width from Window_Rect / function 0x421430
pub const DEFAULT_WINDOW_WIDTH: f32 = 579.0;
/// Default window pixel height from Window_Rect / function 0x421430
pub const DEFAULT_WINDOW_HEIGHT: f32 = 750.0;

/// Extended ListView style flags (LVS_EX_FULLROWSELECT | LVS_EX_DOUBLEBUFFER = 0x10020) from function 0x42afd2
pub const LISTVIEW_EX_STYLE: u32 = 65568;

/// Search box placeholder string from XML resource 242 / function 0x42dfa2
pub const SEARCH_PLACEHOLDER: &str = "Type to find a program";

/// UninstallCache format version from function 0x4250cd
pub const CACHE_VERSION: u32 = 3;
/// Sentinel value for unknown size in cache.dat (0xFFFFFFFF) from function 0x4250cd
pub const CACHE_UNKNOWN_SIZE: u64 = 4294967295;
/// CacheItem in-memory struct allocation size in bytes (0x10) from function 0x424f39
pub const CACHE_ITEM_STRUCT_SIZE: usize = 16;
/// HRESULT E_INVALIDARG (0x80070057) raised on invalid FILETIME conversion from function 0x4367ce
pub const FILETIME_INVALID_ARG_HR: u32 = 2147942487;

/// Expected character length of YYYYMMDD InstallDate strings from function 0x43dead
pub const INSTALL_DATE_STRING_LEN: usize = 8;
/// Year substring length in InstallDate from function 0x43dead
pub const INSTALL_DATE_YEAR_CHARS: usize = 4;
/// Month substring length in InstallDate from function 0x43dead
pub const INSTALL_DATE_MONTH_CHARS: usize = 2;
/// Day substring length in InstallDate from function 0x43dead
pub const INSTALL_DATE_DAY_CHARS: usize = 2;

/// Minimum Windows OS major version for Store app enumeration from function 0x4378a1
pub const STORE_APPS_MIN_OS_MAJOR: u32 = 6;
/// Minimum Windows OS minor version for Store app enumeration from function 0x4378a1
pub const STORE_APPS_MIN_OS_MINOR: u32 = 2;

/// Primary registry subkey scanned for installed desktop programs from function 0x4378a1
pub const UNINSTALL_REG_SUBKEY: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall";

/// Synthesized MSI uninstall prefix from function 0x43dbb4
pub const MSI_UNINSTALL_PREFIX: &str = "msiexec /X";
/// Synthesized MSI modify prefix from function 0x43dbb4
pub const MSI_MODIFY_PREFIX: &str = "msiexec /I";

/// Excluded Inno Setup uninstaller binary name when sizing/finding main exe from function 0x43180c
pub const INNO_UNINSTALLER_EXCLUDE: &str = "unins000.exe";

/// HTML export Program Name column width from function 0x42bbc9
pub const HTML_COL_NAME_WIDTH: u32 = 600;
/// HTML export Size column width from function 0x42bbc9
pub const HTML_COL_SIZE_WIDTH: u32 = 80;
/// HTML export Installed On column width from function 0x42bbc9
pub const HTML_COL_DATE_WIDTH: u32 = 120;
/// HTML export table row height from function 0x42bbc9
pub const HTML_ROW_HEIGHT: u32 = 34;
/// HTML export table header background color from function 0x42bbc9
pub const HTML_HEADER_BG: &str = "#ECE9D8";
/// Win32 LVM_GETITEMCOUNT message constant (0x1004 = 4100) from function 0x42bbc9
pub const LVM_GETITEMCOUNT_MSG: u32 = 4100;
/// Stack buffer capacity in characters (0xfa = 250) for icon path resolution from function 0x423794
pub const ICON_PATH_BUFFER_LEN: usize = 250;

