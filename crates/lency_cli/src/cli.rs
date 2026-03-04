use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lencyc")]
#[command(
    about = "Lency 编译器 - 简洁、规范、清晰",
    version,
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// 详细输出模式
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// 安静模式 (只输出错误)
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 编译 Lency 源文件为 LLVM IR
    Compile {
        /// 输入文件
        input: String,

        /// 输出文件 (默认: lencyTemp.ll)
        #[arg(short, long, default_value = "lencyTemp.ll")]
        output: String,

        /// 输出目录 (可选)。设置后，输出文件会写入该目录
        #[arg(long, value_name = "DIR")]
        out_dir: Option<String>,
    },

    /// 编译并运行 Lency 程序
    Run {
        /// 输入文件
        input: String,

        /// 传递给程序的参数
        #[arg(last = true)]
        args: Vec<String>,
    },

    /// 检查语法和语义错误
    Check {
        /// 输入文件
        input: String,
    },

    /// 编译并生成可执行文件
    Build {
        /// 输入文件
        input: String,

        /// 输出文件 (默认: lencyTemp.out)
        #[arg(short, long, default_value = "lencyTemp.out")]
        output: String,

        /// 输出目录 (可选)。设置后，输出文件会写入该目录
        #[arg(long, value_name = "DIR")]
        out_dir: Option<String>,

        /// 优化构建 (Release mode)
        #[arg(long)]
        release: bool,

        /// 仅做语法/语义检查，不产出可执行文件
        #[arg(long)]
        check_only: bool,
    },

    /// 交互式 REPL (实验性)
    Repl,
}
