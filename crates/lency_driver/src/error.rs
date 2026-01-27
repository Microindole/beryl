//! Compilation Errors
//!
//! 编译器驱动层的错误类型，集成统一诊断系统

use lency_codegen::CodegenError;
use lency_diagnostics::{Diagnostic, DiagnosticSink, Emitter};
use lency_sema::SemanticError;
use thiserror::Error;

/// 编译错误
#[derive(Debug, Error)]
pub enum CompileError {
    /// 词法错误
    #[error("Lexical error: {0}")]
    LexError(String),

    /// 语法错误
    #[error("Parse error: {0}")]
    ParseError(String),

    /// 语义错误（可能有多个）
    #[error("Semantic errors:\n{}", format_semantic_errors(.0))]
    SemanticErrors(Vec<SemanticError>),

    /// 代码生成错误
    #[error("Code generation error: {0}")]
    CodegenError(#[from] CodegenError),

    /// IO 错误
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl CompileError {
    /// 转换为诊断列表并收集到 DiagnosticSink
    pub fn collect_to_sink(&self, sink: &mut DiagnosticSink) {
        match self {
            CompileError::LexError(msg) => {
                sink.add(Diagnostic::error(format!("Lexical error: {}", msg)));
            }
            CompileError::ParseError(msg) => {
                sink.add(Diagnostic::error(format!("Parse error: {}", msg)));
            }
            CompileError::SemanticErrors(errors) => {
                for err in errors {
                    sink.add(err.to_diagnostic());
                }
            }
            CompileError::CodegenError(err) => {
                sink.add(err.to_diagnostic());
            }
            CompileError::IoError(err) => {
                sink.add(Diagnostic::error(format!("IO error: {}", err)));
            }
        }
    }

    /// 使用统一诊断系统输出错误
    pub fn emit(&self) {
        let mut sink = DiagnosticSink::new();
        self.collect_to_sink(&mut sink);

        let emitter = Emitter::new();
        emitter.emit_all(sink.diagnostics());
    }
}

/// 格式化语义错误列表
fn format_semantic_errors(errors: &[SemanticError]) -> String {
    errors
        .iter()
        .enumerate()
        .map(|(i, e)| format!("  {}. {}", i + 1, e))
        .collect::<Vec<_>>()
        .join("\n")
}

/// 编译结果类型
pub type CompileResult<T> = Result<T, CompileError>;
