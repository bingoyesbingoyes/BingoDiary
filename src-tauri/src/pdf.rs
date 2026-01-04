#[cfg(not(target_os = "android"))]
use headless_chrome::{Browser, LaunchOptions};
#[cfg(not(target_os = "android"))]
use headless_chrome::types::PrintToPdfOptions;
use pulldown_cmark::{Options, Parser};
use regex::Regex;
use std::fs;
use std::path::Path;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

/// Get MIME type from file extension
fn get_mime_type(path: &str) -> &'static str {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "svg" => "image/svg+xml",
        _ => "application/octet-stream",
    }
}

/// Convert local image path to base64 data URL
fn image_to_base64(path: &str) -> Option<String> {
    let path = Path::new(path);
    if !path.exists() || !path.is_file() {
        return None;
    }

    match fs::read(path) {
        Ok(data) => {
            let mime = get_mime_type(path.to_str().unwrap_or(""));
            let base64_data = BASE64.encode(&data);
            Some(format!("data:{};base64,{}", mime, base64_data))
        }
        Err(_) => None,
    }
}

/// Replace local image paths with base64 data URLs
fn embed_images(content: &str, base_dir: Option<&Path>) -> String {
    // Match both absolute paths (/path/to/img) and relative paths (images/xxx.png)
    let img_re = Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").unwrap();

    img_re.replace_all(content, |caps: &regex::Captures| {
        let alt = &caps[1];
        let path_str = &caps[2];

        // Skip URLs (http/https/data)
        if path_str.starts_with("http://") || path_str.starts_with("https://") || path_str.starts_with("data:") {
            return caps[0].to_string();
        }

        // Resolve relative paths using base_dir
        let full_path = if path_str.starts_with('/') {
            Path::new(path_str).to_path_buf()
        } else if let Some(base) = base_dir {
            base.join(path_str)
        } else {
            return caps[0].to_string();
        };

        match image_to_base64(full_path.to_str().unwrap_or("")) {
            Some(data_url) => format!("![{}]({})", alt, data_url),
            None => caps[0].to_string(), // Keep original if conversion fails
        }
    }).to_string()
}

/// Render LaTeX math to HTML using KaTeX
fn render_math(latex: &str, display_mode: bool) -> String {
    let opts = katex::Opts::builder()
        .display_mode(display_mode)
        .throw_on_error(false)
        .build()
        .unwrap();

    match katex::render_with_opts(latex, &opts) {
        Ok(html) => {
            if display_mode {
                format!(r#"<div class="math-display">{}</div>"#, html)
            } else {
                html
            }
        }
        Err(_) => format!("<code class=\"math-error\">{}</code>", latex),
    }
}

/// Preprocess markdown to render math expressions
fn preprocess_math(content: &str) -> String {
    let mut result = content.to_string();

    // Display math: $$...$$
    let display_re = Regex::new(r"\$\$([\s\S]+?)\$\$").unwrap();
    result = display_re
        .replace_all(&result, |caps: &regex::Captures| {
            render_math(&caps[1].trim(), true)
        })
        .to_string();

    // Inline math: $...$
    let inline_re = Regex::new(r"\$([^\$\n]+?)\$").unwrap();
    result = inline_re
        .replace_all(&result, |caps: &regex::Captures| {
            render_math(&caps[1].trim(), false)
        })
        .to_string();

    result
}

/// Strip YAML frontmatter from markdown
fn strip_frontmatter(content: &str) -> &str {
    if content.starts_with("---") {
        if let Some(end) = content[3..].find("\n---") {
            return content[3 + end + 4..].trim_start();
        }
    }
    content
}

/// Convert single line breaks to <br> for soft line breaks
fn preserve_line_breaks(content: &str) -> String {
    let mut result = String::new();
    let mut in_code_block = false;

    for line in content.lines() {
        if line.trim_start().starts_with("```") {
            in_code_block = !in_code_block;
            result.push_str(line);
            result.push('\n');
        } else if in_code_block {
            result.push_str(line);
            result.push('\n');
        } else {
            result.push_str(line);
            // Add <br> for soft breaks (non-empty lines not followed by special elements)
            if !line.is_empty() && !line.trim().is_empty() {
                result.push_str("  "); // Two spaces = soft break in markdown
            }
            result.push('\n');
        }
    }
    result
}

/// Convert markdown to HTML
fn markdown_to_html(markdown: &str, base_dir: Option<&Path>) -> String {
    // Strip YAML frontmatter
    let content = strip_frontmatter(markdown);

    // Preserve single line breaks
    let content = preserve_line_breaks(content);

    // Embed local images as base64
    let content = embed_images(&content, base_dir);

    // Process math expressions
    let processed = preprocess_math(&content);

    // Then convert markdown to HTML
    let options = Options::all();
    let parser = Parser::new_ext(&processed, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    html_output
}

/// Get CSS styles for PDF
fn get_styles() -> &'static str {
    r#"
    * { box-sizing: border-box; }
    body {
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans SC", "PingFang SC", sans-serif;
        font-size: 14px;
        line-height: 1.8;
        color: #1a1a1a;
        max-width: 210mm;
        margin: 0 auto;
        padding: 15mm 20mm;
    }
    h1, h2, h3, h4, h5, h6 {
        color: #2c3e50;
        margin-top: 1.5em;
        margin-bottom: 0.5em;
        page-break-after: avoid;
    }
    h1 { font-size: 2em; border-bottom: 2px solid #3498db; padding-bottom: 0.3em; }
    h2 { font-size: 1.5em; }
    h3 { font-size: 1.25em; }
    p { margin: 0.8em 0; page-break-inside: avoid; }
    code {
        background: #f5f5f5;
        padding: 0.2em 0.4em;
        border-radius: 3px;
        font-family: Monaco, Consolas, "Courier New", monospace;
        font-size: 0.9em;
    }
    pre {
        background: #f8f8f8;
        padding: 1em;
        border-radius: 6px;
        overflow-x: auto;
        border: 1px solid #e0e0e0;
        page-break-inside: avoid;
    }
    pre code { background: none; padding: 0; }
    blockquote {
        border-left: 4px solid #3498db;
        margin: 1em 0;
        padding: 0.5em 1em;
        background: #f9f9f9;
        color: #555;
        page-break-inside: avoid;
    }
    blockquote p { margin: 0.3em 0; }
    table { border-collapse: collapse; width: 100%; margin: 1em 0; }
    th, td { border: 1px solid #ddd; padding: 0.6em; text-align: left; }
    th { background: #f5f5f5; font-weight: 600; }
    tr:nth-child(even) { background: #fafafa; }
    hr { border: none; border-top: 1px solid #ddd; margin: 2em 0; }
    ul, ol { padding-left: 2em; }
    li { margin: 0.3em 0; page-break-inside: avoid; }
    a { color: #3498db; text-decoration: none; }
    img {
        max-width: 100%;
        max-height: 500px;
        object-fit: contain;
        page-break-inside: avoid;
        display: block;
        margin: 1em auto;
    }
    .math-display { text-align: center; margin: 1.2em 0; font-size: 1.1em; }
    .math-error { color: #e74c3c; background: #fef0f0; }
    "#
}

/// Get KaTeX CSS (inline for offline support)
fn get_katex_styles() -> &'static str {
    include_str!("katex.min.css")
}

/// Create full HTML document
fn create_html_document(content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <style>{}</style>
    <style>{}</style>
</head>
<body>
{}
</body>
</html>"#,
        get_katex_styles(),
        get_styles(),
        content
    )
}

/// Export markdown to PDF using headless Chrome (Desktop only)
#[cfg(not(target_os = "android"))]
pub fn export_to_pdf(markdown: &str, output_path: &Path, base_dir: Option<&Path>) -> Result<(), String> {
    // Convert markdown to HTML
    let html_content = markdown_to_html(markdown, base_dir);
    let full_html = create_html_document(&html_content);

    // Create temporary HTML file
    let temp_dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let temp_html = temp_dir.path().join("temp.html");
    fs::write(&temp_html, &full_html).map_err(|e| e.to_string())?;

    // Launch headless Chrome
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .headless(true)
            .build()
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;

    let tab = browser.new_tab().map_err(|e| e.to_string())?;

    // Navigate to the HTML file
    let file_url = format!("file://{}", temp_html.to_string_lossy());
    tab.navigate_to(&file_url).map_err(|e| e.to_string())?;
    tab.wait_until_navigated().map_err(|e| e.to_string())?;

    // Generate PDF with A4 paper size and print_background enabled
    let pdf_options = PrintToPdfOptions {
        print_background: Some(true),
        paper_width: Some(8.27),     // A4: 210mm
        paper_height: Some(11.69),   // A4: 297mm
        margin_top: Some(0.78),      // ~20mm
        margin_bottom: Some(0.78),
        margin_left: Some(0.78),
        margin_right: Some(0.78),
        ..Default::default()
    };
    let pdf_data = tab
        .print_to_pdf(Some(pdf_options))
        .map_err(|e| e.to_string())?;

    // Write PDF to file
    fs::write(output_path, pdf_data).map_err(|e| e.to_string())?;

    Ok(())
}

/// Export markdown to PDF - Android stub (not supported)
#[cfg(target_os = "android")]
pub fn export_to_pdf(_markdown: &str, _output_path: &Path, _base_dir: Option<&Path>) -> Result<(), String> {
    Err("PDF export is not supported on Android".to_string())
}
