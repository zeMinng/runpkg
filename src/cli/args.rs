use clap::{Parser, Subcommand};
use crate::constants::{ASCII_BANNER, SLOGAN, HELP_FOOTER};

#[derive(Parser, Debug)]
#[command(
    name = "runpkg",
    author = "Your Name <your@email.com>",
    // 从 Cargo.toml 自动读取版本号，作为 -v/--version 的输出
    version = env!("CARGO_PKG_VERSION"),
    // 简短描述
    about = SLOGAN,
    // 🎯 核心黑科技：在 -h / --help 的最上方打印 ASCII 艺术字！
    before_help = ASCII_BANNER,
    // 🎯 在帮助文档的最下方追加 Tip
    after_help = HELP_FOOTER
)]
pub struct CliArgs {
    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,

    /// 是否显示冗余的调试日志
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// 运行指定的 package.json 脚本
    Run {
        script: String,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// 查看并交互式升级依赖
    Dep,
    /// 诊断项目健康状态与僵尸依赖
    Doctor,
}