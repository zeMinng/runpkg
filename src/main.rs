use anyhow::{Context, Result};
use clap::Parser;
use std::io;

mod cli;
mod constants;
mod core;
mod ui;

use cli::{CliArgs, SubCommand};

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    match args.subcommand {
        Some(SubCommand::Run { script, args }) => {
            cli::printer::print_welcome_banner();
            core::runner::exec_script(&script, &args)?;
        }
        Some(SubCommand::Dep) => {
            run_interactive_tui(Some(ui::MenuOption::DependencyManager)).await?;
        }
        Some(SubCommand::Doctor) => {
            run_interactive_tui(Some(ui::MenuOption::ProjectDoctor)).await?;
        }
        None => {
            run_interactive_tui(None).await?;
        }
    }

    Ok(())
}

async fn run_interactive_tui(initial_tab: Option<ui::MenuOption>) -> Result<()> {
    let mut terminal = setup_terminal().context("初始化终端失败")?;
    let mut app = ui::App::new(initial_tab).await?;

    let run_result = app.run_loop(&mut terminal).await;

    // ⚠️ 核心规则：无论是按 q 正常退出还是出错，都必须先恢复终端状态！
    restore_terminal(&mut terminal).context("恢复终端失败")?;

    // 如果用户在菜单里面敲回车运行了某个脚本，此时终端已恢复，优雅启动前端脚本
    if let Some(task_to_run) = app.take_pending_task() {
        println!("🚀 正在启动脚本: {}\n", task_to_run.script_name);
        core::runner::exec_script(&task_to_run.script_name, &task_to_run.args)?;
    }

    run_result?;
    Ok(())
}

fn setup_terminal() -> Result<ratatui::Terminal<ratatui::backend::CrosstermBackend<io::Stdout>>> {
    use crossterm::{
        execute,
        terminal::{enable_raw_mode, EnterAlternateScreen},
    };
    use ratatui::{backend::CrosstermBackend, Terminal};

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<io::Stdout>>,
) -> Result<()> {
    use crossterm::{
        execute,
        terminal::{disable_raw_mode, LeaveAlternateScreen},
    };

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}