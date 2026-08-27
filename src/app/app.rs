use std::path::PathBuf;

use tokio::sync::mpsc::UnboundedReceiver;

use super::{Action, AppState, Focus, OutputState, Screen};
use crate::system::runner::ScriptEvent;

pub struct App {
    pub state: AppState,
    pub project_path: PathBuf,
    pub screen: Screen,
    pub focus: Focus,
    pub script_cursor: usize,
    pub dep_cursor: usize,
    pub output: OutputState,
    receiver: Option<UnboundedReceiver<ScriptEvent>>,
    pending_script: Option<String>,
    pub should_quit: bool,
}

impl App {
    pub fn new(state: AppState, project_path: PathBuf) -> Self {
        Self {
            state,
            project_path,
            screen: Screen::Dashboard,
            focus: Focus::Sidebar,
            script_cursor: 0,
            dep_cursor: 0,
            output: OutputState::default(),
            receiver: None,
            pending_script: None,
            should_quit: false,
        }
    }

    pub fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.should_quit = true,

            Action::NavigateUp => self.move_selection(-1),
            Action::NavigateDown => self.move_selection(1),
            Action::NavigateLeft => self.exit_content(),
            Action::NavigateRight => self.enter_content(),

            Action::Confirm => self.confirm(),
            Action::Back => self.exit_content(),

            Action::Refresh => self.refresh_project(),

            Action::OpenDashboard => self.open(Screen::Dashboard),
            Action::OpenScripts => self.open(Screen::Scripts),
            Action::OpenDependencies => self.open(Screen::Dependencies),
            Action::OpenDoctor => self.open(Screen::Doctor),

            Action::RunScript(name) => self.pending_script = Some(name),
        }
    }

    fn move_selection(&mut self, delta: isize) {
        match self.focus {
            Focus::Sidebar => self.switch_screen(delta),
            Focus::Content => match self.screen {
                Screen::Scripts => {
                    self.script_cursor = step(self.script_cursor, delta, self.script_count());
                }
                Screen::Dependencies => {
                    self.dep_cursor = step(self.dep_cursor, delta, self.dep_count());
                }
                _ => {}
            },
        }
    }

    fn switch_screen(&mut self, delta: isize) {
        let len = Screen::ALL.len() as isize;
        let current = self.screen.as_index() as isize;
        let next = (current + delta).rem_euclid(len);
        self.screen = Screen::from_index(next as usize);
        self.focus = Focus::Sidebar;
        self.script_cursor = 0;
        self.dep_cursor = 0;
    }

    fn open(&mut self, screen: Screen) {
        self.screen = screen;
        self.focus = Focus::Sidebar;
        self.script_cursor = 0;
        self.dep_cursor = 0;
    }

    fn confirm(&mut self) {
        match self.focus {
            Focus::Sidebar => self.enter_content(),
            Focus::Content => {
                if self.screen == Screen::Scripts
                    && let Some(name) = self.script_name_at(self.script_cursor)
                {
                    self.pending_script = Some(name);
                }
            }
        }
    }

    /// Move keyboard focus from the sidebar into the content pane.
    /// Only screens with an interactive list are focusable. (焦点从侧边栏进入内容区)
    fn enter_content(&mut self) {
        if self.focus == Focus::Sidebar
            && matches!(self.screen, Screen::Scripts | Screen::Dependencies)
        {
            self.focus = Focus::Content;
        }
    }

    /// Return keyboard focus from the content pane back to the sidebar. (焦点退回侧边栏)
    fn exit_content(&mut self) {
        if self.focus == Focus::Content {
            self.focus = Focus::Sidebar;
        }
    }

    fn refresh_project(&mut self) {
        self.state.project = crate::project::package_json::load_from(&self.project_path)
            .ok()
            .map(crate::project::info::ProjectInfo::from);
        self.script_cursor = 0;
        self.dep_cursor = 0;
    }

    pub fn script_count(&self) -> usize {
        self.state.project.as_ref().map(|p| p.scripts.len()).unwrap_or(0)
    }

    pub fn script_name_at(&self, index: usize) -> Option<String> {
        self.state
            .project
            .as_ref()
            .and_then(|p| p.scripts.get_index(index))
            .map(|(name, _)| name.clone())
    }

    pub fn dep_count(&self) -> usize {
        self.state
            .project
            .as_ref()
            .map(|p| p.dependencies.len() + p.dev_dependencies.len() + p.peer_dependencies.len())
            .unwrap_or(0)
    }

    /// Spawn the script queued by `Action::RunScript`, attaching its output receiver.
    pub fn start_pending_script(&mut self) {
        let Some(name) = self.pending_script.take() else {
            return;
        };

        let project = self.state.project.as_ref();
        let runtime = self.state.runtime.as_ref();
        let pm = crate::system::pm::preferred(project, runtime);
        let (program, args) = crate::system::pm::build_run_command(&pm, &name);

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        crate::system::runner::spawn(program, args, tx);

        self.output.clear();
        self.output.running = Some(name);
        self.receiver = Some(rx);
    }

    /// Drain any buffered script output into `self.output`.
    pub fn drain_script_output(&mut self) {
        let Some(receiver) = self.receiver.as_mut() else {
            return;
        };

        loop {
            match receiver.try_recv() {
                Ok(ScriptEvent::Line(line)) => self.output.push_line(line),
                Ok(ScriptEvent::Finished(code)) => {
                    self.output.exit_code = code;
                    self.output.running = None;
                    self.receiver = None;
                    break;
                }
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => break,
                Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                    self.output.running = None;
                    self.receiver = None;
                    break;
                }
            }
        }
    }
}

/// Clamp `current + delta` into `[0, len)`; `0` when the list is empty.
fn step(current: usize, delta: isize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    (current as isize + delta).clamp(0, len as isize - 1) as usize
}
