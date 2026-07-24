use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use std::env;
use std::process::ExitCode;

/// 🚀 下一代交互式 package.json 控制台 / 项目驾驶舱
#[derive(Parser, Debug)]
#[command(
    name = "runpkg",
    author,
    version,
    about = "🚀 下一代交互式 package.json 控制台 / 项目驾驶舱",
    long_about = None
)]
struct Cli {
    /// 直接执行指定的脚本名称 (若不传，则默认启动交互式终端菜单)
    #[arg(value_name = "SCRIPT")]
    script: Option<String>,

    /// 是否开启多任务并发执行模式
    #[arg(short, long)]
    parallel: bool,

    /// 跳过交互模式，在 CI/CD 或自动化脚本中使用
    #[arg(short = 'y', long = "yes")]
    non_interactive: bool,

    /// 追加透传给底层脚本的命令行参数 (在 `--` 之后传入)
    #[arg(last = true)]
    extra_args: Vec<String>,
}

#[tokio::main]
async fn main() -> ExitCode {
    // 1. 注册全局 Panic 钩子，提供人性化的崩溃报错 UI
    setup_panic_hook();

    // 2. 解析 CLI 输入参数
    let cli = Cli::parse();

    // 3. 执行主业务流程并捕获顶层错误
    match run_app(cli).await {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!(
                "\n{} {}",
                " ✖ ERROR ".on_red().white().bold(),
                err.to_string().red().bold()
            );

            // 递归打印错误链条 (Caused by)
            for cause in err.chain().skip(1) {
                eprintln!("   {} {}", "↳".dimmed(), cause.to_string().dimmed());
            }
            eprintln!();

            ExitCode::FAILURE
        }
    }
}

/// 核心业务路由控制流
async fn run_app(cli: Cli) -> Result<()> {
    // 1. 获取当前执行路径
    let current_dir = env::current_dir().context("无法读取当前工作目录")?;

    // 2. 模拟自动检测包管理器 (后续由 core::pm::detect 模块提供)
    let detected_pm = detect_package_manager_mock(&current_dir);

    // 3. 打印醒目的状态头部栏
    println!(
        "{} {} {}\n",
        " 📦 [runpkg] ".on_blue().white().bold(),
        "自动识别包管理器:".dimmed(),
        detected_pm.bright_green().bold()
    );

    // 4. 根据用户输入分发执行模式
    match (cli.script, cli.non_interactive) {
        // 模式 A: 用户指定了具体的脚本名称 (例如: runpkg dev)
        (Some(script_name), _) => {
            println!(
                "正在使用 {} 运行脚本: {}...",
                detected_pm.cyan(),
                script_name.yellow().bold()
            );

            if !cli.extra_args.is_empty() {
                println!(
                    "{} 透传参数: {}",
                    "  ↳".dimmed(),
                    cli.extra_args.join(" ").dimmed()
                );
            }

            // TODO: 调用 executor::runner 执行子进程
            // executor::runner::exec_single(&detected_pm, &script_name, &cli.extra_args).await?;
        }

        // 模式 B: 在 CI/CD 无交互模式下，但未指定任何脚本名称，抛出错误
        (None, true) => {
            anyhow::bail!("在非交互模式 (--yes) 下，必须指定要运行的脚本名称。示例: runpkg build");
        }

        // 模式 C: 默认模式 -> 进入交互式控制台菜单
        (None, false) => {
            run_interactive_menu(&detected_pm, cli.parallel).await?;
        }
    }

    Ok(())
}

/// 交互式菜单逻辑 (入口展示)
async fn run_interactive_menu(pm: &str, parallel: bool) -> Result<()> {
    println!(
        "{}",
        "✨ 欢迎进入交互式驾驶舱 (按 Ctrl+C 退出)".bright_yellow()
    );

    // 模拟从 package.json 读取到的脚本列表 (后续由 core::manifest 模块提供)
    let mock_scripts = vec![
        "dev          - 启动本地开发服务器 (Vite)",
        "build        - 打包构建生产环境代码",
        "test         - 运行单元测试 (Vitest)",
        "lint         - 执行 ESLint 代码检查",
        "type-check   - 执行 TypeScript 类型检查",
    ];

    // 使用 inquire 库弹窗供用户选择
    let ans = inquire::Select::new("请选择要执行的项目脚本:", mock_scripts)
        .with_page_size(10)
        .prompt();

    match ans {
        Ok(choice) => {
            let script_name = choice.split_whitespace().next().unwrap_or("");
            println!(
                "\n{} 准备执行 [{}] 模式下的 {} -> {}",
                "🚀".bold(),
                if parallel { "并发" } else { "单任务" }.cyan(),
                pm.green(),
                script_name.yellow().bold()
            );

            // TODO: 调用 executor 模块真正调度进程执行
        }
        Err(inquire::InquireError::OperationCanceled) => {
            println!("{}", "\n已取消操作。".dimmed());
        }
        Err(err) => {
            anyhow::bail!("交互菜单渲染异常: {}", err);
        }
    }

    Ok(())
}

/// 模拟包管理器自动检测算法
fn detect_package_manager_mock(path: &std::path::Path) -> String {
    if path.join("pnpm-lock.yaml").exists() {
        "pnpm".to_string()
    } else if path.join("bun.lockb").exists() || path.join("bun.lock").exists() {
        "bun".to_string()
    } else if path.join("yarn.lock").exists() {
        "yarn".to_string()
    } else {
        "npm".to_string()
    }
}

/// 美化程序崩溃时的异常捕获钩子
fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let message = if let Some(s) = info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "未知的内部致命错误".to_string()
        };

        eprintln!(
            "\n{} {}",
            " 💥 CRASH ".on_red().white().bold(),
            "程序遇到了不可恢复的异常发生了崩溃:".red().bold()
        );
        eprintln!("   {}\n", message);
        eprintln!(
            "{}",
            "如果这是一个 Bug，欢迎提 Issue 到项目仓库进行反馈。".dimmed()
        );
    }));
}