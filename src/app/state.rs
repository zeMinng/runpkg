use crate::project::info::ProjectInfo;
use crate::system::runtime::RuntimeInfo;

#[derive(Debug)]
pub struct AppState {
    pub project: Option<ProjectInfo>,
    pub runtime: Option<RuntimeInfo>,
}

impl AppState {
    pub fn new(project: Option<ProjectInfo>, runtime: Option<RuntimeInfo>) -> Self {
        Self { project, runtime }
    }
}

/// The currently active screen. (当前激活的屏幕)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Dashboard,
    Scripts,
    Dependencies,
    Doctor,
}

impl Screen {
    /// All screens in sidebar order. (侧边栏顺序下的所有屏幕)
    pub const ALL: [Screen; 4] = [
        Screen::Dashboard,
        Screen::Scripts,
        Screen::Dependencies,
        Screen::Doctor,
    ];

    pub fn from_index(index: usize) -> Self {
        Self::ALL.get(index).copied().unwrap_or(Screen::Dashboard)
    }

    pub fn as_index(self) -> usize {
        Self::ALL.iter().position(|s| *s == self).unwrap_or(0)
    }

    pub fn title(self) -> &'static str {
        match self {
            Screen::Dashboard => "Dashboard",
            Screen::Scripts => "Scripts",
            Screen::Dependencies => "Dependencies",
            Screen::Doctor => "Doctor",
        }
    }
}

/// Which pane currently owns keyboard focus. (当前键盘焦点所在区域)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Sidebar,
    Content,
}

/// Buffered output from a running script. (脚本执行的输出缓冲)
#[derive(Debug, Default)]
pub struct OutputState {
    pub lines: Vec<String>,
    pub running: Option<String>,
    pub exit_code: Option<i32>,
}

impl OutputState {
    const MAX_LINES: usize = 500;

    pub fn push_line(&mut self, line: String) {
        self.lines.push(line);
        let len = self.lines.len();
        if len > Self::MAX_LINES {
            self.lines.drain(0..len - Self::MAX_LINES);
        }
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.running = None;
        self.exit_code = None;
    }
}
