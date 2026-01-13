//! Compilation Errors
//!
//! 编译器驱动层的错误类型

use lency_codegen::CodegenError;
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
