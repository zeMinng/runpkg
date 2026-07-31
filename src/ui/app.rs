use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{backend::Backend, Terminal};
use std::time::Duration;
use crate::core::env::ProjectEnv;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuOption {
    ScriptRunner = 0,
    DependencyManager = 1,
    ProjectDoctor = 2,
    PackageManager = 3,
    ProjectDashboard = 4,
}

pub struct PendingTask {
    pub script_name: String,
    pub args: Vec<String>,
}

pub struct App {
    pub selected_index: usize,
    pub menu_items: Vec<(&'static str, &'static str)>,
    pub env: ProjectEnv,
    pending_task: Option<PendingTask>,
    pub should_quit: bool,
}

impl App {
    pub async fn new(initial_tab: Option<MenuOption>) -> Result<Self> {
        let env = ProjectEnv::detect();
        let initial_idx = initial_tab.map(|t| t as usize).unwrap_or(0);

        Ok(Self {
            selected_index: initial_idx,
            menu_items: vec![
                ("1. ▶  Script Runner", "运行 package.json 脚本"),
                ("2. 📦 Dependency Manager", "依赖查看与交互式升级"),
                ("3. 🩺 Project Doctor", "锁文件与僵尸依赖诊断"),
                ("4. ⚙  Package Manager", "包管理器切换与配置"),
                ("5. 📊 Project Dashboard", "项目元数据与概览"),
            ],
            env,
            pending_task: None,
            should_quit: false,
        })
    }

    pub async fn run_loop<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|f| crate::ui::layout::render(f, self))?;

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                            KeyCode::Down | KeyCode::Char('j') => {
                                self.selected_index =
                                    (self.selected_index + 1) % self.menu_items.len();
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                if self.selected_index == 0 {
                                    self.selected_index = self.menu_items.len() - 1;
                                } else {
                                    self.selected_index -= 1;
                                }
                            }
                            KeyCode::Char('1') => self.selected_index = 0,
                            KeyCode::Char('2') => self.selected_index = 1,
                            KeyCode::Char('3') => self.selected_index = 2,
                            KeyCode::Char('4') => self.selected_index = 3,
                            KeyCode::Char('5') => self.selected_index = 4,
                            KeyCode::Enter => {
                                // 模拟：如果在 Script Runner 下点回车，将暂时启动 dev 脚本来演示如何无缝退出并接管终端
                                if self.selected_index == 0 {
                                    self.pending_task = Some(PendingTask {
                                        script_name: "dev".to_string(),
                                        args: vec![],
                                    });
                                    self.should_quit = true;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn take_pending_task(&mut self) -> Option<PendingTask> {
        self.pending_task.take()
    }
}