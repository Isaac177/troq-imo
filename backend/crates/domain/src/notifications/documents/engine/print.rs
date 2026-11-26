pub const MARGIN_TOP_IN: f64 = 0.7874;
pub const MARGIN_BOTTOM_IN: f64 = 0.8661;

pub fn footer_template(font_face: &str, label: &str, page_word: &str) -> String {
    format!(
        "<style>{font_face}</style><div style=\"width:100%;font-family:'Onest',sans-serif;font-size:8px;color:#71717a;padding:0 18mm;display:flex;justify-content:space-between;\"><span>{label}</span><span>{page_word} <span class=\"pageNumber\"></span> / <span class=\"totalPages\"></span></span></div>",
        label = escape(label),
        page_word = escape(page_word),
    )
}

fn escape(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}
