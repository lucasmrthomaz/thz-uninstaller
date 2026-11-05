use crate::constants::{
    HTML_COL_DATE_WIDTH, HTML_COL_NAME_WIDTH, HTML_COL_SIZE_WIDTH, HTML_HEADER_BG,
    HTML_ROW_HEIGHT,
};
use crate::models::ProgramEntry;
use crate::utils::formatting::{format_total_bytes_str, html_escape};

// ============================================================================
// HTML Export (function 0x42bbc9 / 0x60ad20)
// ============================================================================

pub fn generate_html_report(programs: &[&ProgramEntry], total_bytes: u64, use_comma: bool) -> String {
    let mut rows_html = String::new();
    for p in programs {
        rows_html.push_str(&format!(
            "  <tr height=\"{row_h}px\">\n    <td><b>{name}</b><br><font color=\"#666666\">{pub_name} {ver}</font></td>\n    <td>{size}</td>\n    <td>{date}</td>\n  </tr>\n",
            row_h = HTML_ROW_HEIGHT,
            name = html_escape(&p.display_label()),
            pub_name = html_escape(&p.publisher),
            ver = html_escape(&p.display_version),
            size = html_escape(&p.formatted_size(use_comma)),
            date = html_escape(&p.install_date),
        ));
    }

    format!(
        "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">\n\
<html xmlns=\"http://www.w3.org/1999/xhtml\">\n\
<head>\n\
<meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\" />\n\
<title>Installed Programs Report</title>\n\
<style type=\"text/css\">\n\
body {{ font-family: Tahoma; font-size: 8pt; color: #000000; }}\n\
.title {{ font-family: Tahoma; font-size: 10pt; font-weight: bold; }}\n\
table {{ font-family: Tahoma; font-size: 8pt; empty-cells: show; border-collapse: collapse; }}\n\
</style>\n\
</head>\n\
<body>\n\
<div class=\"title\">Installed Programs ({count} programs of {total_size} size in total)</div>\n\
<br/>\n\
<table border=\"1\" cellspacing=\"0\" cellpadding=\"5\" bordercolor=\"#C0C0C0\">\n\
  <tr height=\"{row_h}px\" bgcolor=\"{header_bg}\" style=\"font-weight:bold\">\n\
    <td width=\"{w_name}px\">Program Name</td>\n\
    <td width=\"{w_size}px\">Size</td>\n\
    <td width=\"{w_date}px\">Installed On</td>\n\
  </tr>\n\
{rows}\
</table>\n\
</body>\n\
</html>\n",
        count = programs.len(),
        total_size = format_total_bytes_str(total_bytes, use_comma),
        row_h = HTML_ROW_HEIGHT,
        header_bg = HTML_HEADER_BG,
        w_name = HTML_COL_NAME_WIDTH,
        w_size = HTML_COL_SIZE_WIDTH,
        w_date = HTML_COL_DATE_WIDTH,
        rows = rows_html,
    )
}

