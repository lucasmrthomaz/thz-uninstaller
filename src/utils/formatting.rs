use chrono::NaiveDate;

use crate::constants::{
    INSTALL_DATE_DAY_CHARS, INSTALL_DATE_MONTH_CHARS, INSTALL_DATE_STRING_LEN,
    INSTALL_DATE_YEAR_CHARS,
};

/// Formats byte sizes identically to Windows `StrFormatByteSizeW` (3 significant digits):
/// - `1.00 .. 9.99` -> 2 decimal places (`3,12 GB`)
/// - `10.0 .. 99.9` -> 1 decimal place (`53,1 MB`)
/// - `100 .. 999` -> 0 decimal places (`240 MB`, `876 MB`)
pub fn format_bytes_str(bytes: u64, use_comma: bool) -> String {
    if bytes == 0 {
        return String::new();
    }
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    let b = bytes as f64;

    let (val, unit) = if b >= GB {
        (b / GB, "GB")
    } else if b >= MB {
        (b / MB, "MB")
    } else {
        (b / KB, "KB")
    };

    let raw = if val >= 100.0 || unit == "KB" {
        format!("{:.0} {}", val.floor(), unit)
    } else if val >= 10.0 {
        let trunc1 = (val * 10.0).floor() / 10.0;
        format!("{:.1} {}", trunc1, unit)
    } else {
        let trunc2 = (val * 100.0).floor() / 100.0;
        format!("{:.2} {}", trunc2, unit)
    };

    if use_comma {
        raw.replace('.', ",")
    } else {
        raw
    }
}

pub fn format_total_bytes_str(bytes: u64, use_comma: bool) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    let b = bytes as f64;
    let raw = if b >= GB {
        let g = (b / GB).floor();
        format!("{:.1} GB", g)
    } else if b >= MB {
        format!("{:.1} MB", b / MB)
    } else {
        format!("{:.0} KB", b / KB)
    };
    if use_comma {
        raw.replace('.', ",")
    } else {
        raw
    }
}

pub fn extract_xml_attr(line: &str, attr: &str) -> Option<String> {
    let pattern = format!("{}=\"", attr);
    let start = line.find(&pattern)? + pattern.len();
    let rest = &line[start..];
    let end = rest.find('"')?;
    Some(
        rest[..end]
            .replace("&amp;", "&")
            .replace("&quot;", "\"")
            .replace("&lt;", "<")
            .replace("&gt;", ">"),
    )
}

/// Parses an 8-character YYYYMMDD string per function 0x43dead.
pub fn parse_yyyymmdd(raw: &str) -> Option<NaiveDate> {
    let trimmed = raw.trim();
    if trimmed.len() == INSTALL_DATE_STRING_LEN && trimmed.chars().all(|c| c.is_ascii_digit()) {
        let year_end = INSTALL_DATE_YEAR_CHARS;
        let month_end = year_end + INSTALL_DATE_MONTH_CHARS;
        let day_end = month_end + INSTALL_DATE_DAY_CHARS;
        if let (Ok(y), Ok(m), Ok(d)) = (
            trimmed[0..year_end].parse::<i32>(),
            trimmed[year_end..month_end].parse::<u32>(),
            trimmed[month_end..day_end].parse::<u32>(),
        ) {
            if (1990..=2100).contains(&y) && (1..=12).contains(&m) && (1..=31).contains(&d) {
                return NaiveDate::from_ymd_opt(y, m, d);
            }
        }
    }
    None
}

pub fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes_str_empty_on_zero() {
        assert_eq!(format_bytes_str(0, false), "");
    }

    #[test]
    fn test_format_bytes_str_kb_mb_gb() {
        assert_eq!(format_bytes_str(1024 * 500, false), "500 KB");
        assert_eq!(format_bytes_str((1024 * 1024 * 12) as u64, false), "12.0 MB");
        assert_eq!(format_bytes_str((1024 * 1024 * 12) as u64, true), "12,0 MB");
    }

    #[test]
    fn test_parse_yyyymmdd_valid() {
        let d = parse_yyyymmdd("20261105");
        assert!(d.is_some());
        let date = d.unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2026, 11, 5).unwrap());
    }

    #[test]
    fn test_parse_yyyymmdd_invalid() {
        assert!(parse_yyyymmdd("invalid").is_none());
        assert!(parse_yyyymmdd("20261399").is_none());
        assert!(parse_yyyymmdd("").is_none());
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<b>\"test\" & 'val'</b>"), "&lt;b&gt;&quot;test&quot; &amp; 'val'&lt;/b&gt;");
    }
}

