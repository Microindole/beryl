//! Emitter - 诊断输出器
//!
//! 负责将诊断信息格式化输出

use crate::diagnostic::Diagnostic;
use colored::*;

/// 诊断输出器
pub struct Emitter {
    /// 是否使用颜色
    use_colors: bool,
}

impl Default for Emitter {
    fn default() -> Self {
        Self::new()
    }
}

impl Emitter {
    /// 创建新的输出器
    pub fn new() -> Self {
        Self { use_colors: true }
    }

    /// 创建无颜色的输出器
    pub fn without_colors() -> Self {
        Self { use_colors: false }
    }

    /// 输出单个诊断
    pub fn emit(&self, diagnostic: &Diagnostic) {
        if self.use_colors {
            self.emit_colored(diagnostic, None);
        } else {
            self.emit_plain(diagnostic, None);
        }
    }

    /// 输出带源文件的诊断（支持 line:col）
    pub fn emit_with_source(&self, diagnostic: &Diagnostic, source: &str) {
        if self.use_colors {
            self.emit_colored(diagnostic, Some(source));
        } else {
            self.emit_plain(diagnostic, Some(source));
        }
    }

    /// 输出所有诊断
    pub fn emit_all(&self, diagnostics: &[Diagnostic]) {
        for diagnostic in diagnostics {
            self.emit(diagnostic);
            println!(); // 诊断之间空行
        }
    }

    /// 输出带颜色的诊断
    fn emit_colored(&self, diagnostic: &Diagnostic, source: Option<&str>) {
        // 级别和消息
        println!(
            "{}: {}",
            diagnostic.level.colored_name(),
            diagnostic.message.bold()
        );

        // 位置信息（如果有）
        if let Some(span) = &diagnostic.span {
            if let (Some(src), Some(file)) = (source, &diagnostic.file_path) {
                let (line, col) = resolve_line_col(src, span.start);
                println!("  {} {}:{}:{}", "-->".blue().bold(), file, line, col);

                // 显示源代码片段
                self.emit_source_snippet_colored(src, span.start, span.len(), line);
            } else {
                println!("  {} {:?}", "-->".blue().bold(), span);
            }
        }

        // 注释
        for note in &diagnostic.notes {
            println!(
                "  {} {}",
                "=".blue().bold(),
                format!("note: {}", note).bright_black()
            );
        }

        // 建议
        for suggestion in &diagnostic.suggestions {
            println!(
                "  {} {}",
                "=".green().bold(),
                format!("help: {}", suggestion.message).green()
            );
            if let Some(replacement) = &suggestion.replacement {
                println!("        try: {}", replacement.green().italic());
            }
        }
    }

    /// 输出纯文本诊断
    fn emit_plain(&self, diagnostic: &Diagnostic, source: Option<&str>) {
        // 标准格式: error: file:line:col: message
        let pos = if let (Some(span), Some(src), Some(file)) =
            (&diagnostic.span, source, &diagnostic.file_path)
        {
            let (line, col) = resolve_line_col(src, span.start);
            format!(" {}:{}:{}:", file, line, col)
        } else {
            String::new()
        };

        println!("{}:{} {}", diagnostic.level, pos, diagnostic.message);

        // 详细位置信息
        if let Some(span) = &diagnostic.span {
            if let (Some(src), Some(_)) = (source, &diagnostic.file_path) {
                let (line, _) = resolve_line_col(src, span.start);
                // 显示源代码片段 (Plain)
                self.emit_source_snippet_plain(src, span.start, span.len(), line);
            } else {
                println!("  --> {:?}", span);
            }
        }

        // 注释
        for note in &diagnostic.notes {
            println!("  = note: {}", note);
        }

        // 建议
        for suggestion in &diagnostic.suggestions {
            println!("  = help: {}", suggestion.message);
            if let Some(replacement) = &suggestion.replacement {
                println!("        try: {}", replacement);
            }
        }
    }

    // --- Snippet Helpers ---

    fn emit_source_snippet_colored(&self, source: &str, start: usize, len: usize, line_num: usize) {
        let (line_content, line_start_offset) = get_line_content(source, start);
        if line_content.trim().is_empty() {
            return;
        }

        let line_num_str = line_num.to_string();
        let gutter_width = line_num_str.len();
        let padding = " ".repeat(gutter_width);

        // 1. Empty line before
        println!("  {} |", padding.blue().bold());

        // 2. Source line
        println!("  {} | {}", line_num_str.blue().bold(), line_content);

        // 3. Pointer line
        // Calculate offset within the line
        let col_offset = start.saturating_sub(line_start_offset);
        // Ensure len is at least 1, and doesn't overflow line
        let mark_len = len
            .max(1)
            .min(line_content.len().saturating_sub(col_offset));

        let pointer_padding = " ".repeat(col_offset);
        let pointer = "^".repeat(mark_len);

        println!(
            "  {} | {}{}",
            padding.blue().bold(),
            pointer_padding,
            pointer.red().bold()
        );
    }

    fn emit_source_snippet_plain(&self, source: &str, start: usize, len: usize, line_num: usize) {
        let (line_content, line_start_offset) = get_line_content(source, start);
        if line_content.trim().is_empty() {
            return;
        }

        let line_num_str = line_num.to_string();
        let gutter_width = line_num_str.len();
        let padding = " ".repeat(gutter_width);

        println!("  {} |", padding);
        println!("  {} | {}", line_num_str, line_content);

        let col_offset = start.saturating_sub(line_start_offset);
        let mark_len = len
            .max(1)
            .min(line_content.len().saturating_sub(col_offset));

        let pointer_padding = " ".repeat(col_offset);
        let pointer = "^".repeat(mark_len);

        println!("  {} | {}{}", padding, pointer_padding, pointer);
    }
}

/// 解析行列
fn resolve_line_col(source: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for (i, c) in source.char_indices() {
        if i >= offset {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

/// 获取指定偏移量所在的行内容和该行的起始偏移量
fn get_line_content(source: &str, offset: usize) -> (&str, usize) {
    let mut line_start = 0;
    let mut line_end = source.len();

    // Find start of line
    for i in (0..=offset.min(source.len().saturating_sub(1))).rev() {
        if source.as_bytes()[i] == b'\n' {
            line_start = i + 1;
            break;
        }
    }

    // Find end of line
    for i in offset..source.len() {
        if source.as_bytes()[i] == b'\n' {
            line_end = i;
            break;
        }
    }

    // Handle case where offset is at EOF or somehow out of bounds gracefully
    if line_start > source.len() || line_start > line_end {
        return ("", line_start);
    }

    (&source[line_start..line_end], line_start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emitter_creation() {
        let emitter = Emitter::new();
        assert!(emitter.use_colors);

        let emitter_no_color = Emitter::without_colors();
        assert!(!emitter_no_color.use_colors);
    }

    #[test]
    fn test_emit_basic() {
        let emitter = Emitter::without_colors();
        let diag = Diagnostic::error("test error");

        // 这个测试只是确保不会panic
        emitter.emit(&diag);
    }

    #[test]
    fn test_emit_with_details() {
        let emitter = Emitter::without_colors();
        let diag = Diagnostic::error("test error")
            .span(10..20)
            .with_note("this is a note")
            .suggest("try this instead");

        emitter.emit(&diag);
    }
}
