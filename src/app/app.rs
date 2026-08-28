use std::path::PathBuf;

use tokio::sync::mpsc::UnboundedReceiver;

use super::{Action, AppState, Focus, OutputState, Screen};
use crate::project::info::ProjectInfo;
use crate::system::runner::ScriptEvent;
use crate::system::runtime::RuntimeInfo;

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

    // Read-only queries form the boundary between application state and the
    // presentation layer. TUI screens should prefer these over reaching into
    // `state` directly.
    pub fn project(&self) -> Option<&ProjectInfo> {
        self.state.project.as_ref()
    }

    pub fn runtime(&self) -> Option<&RuntimeInfo> {
        self.state.runtime.as_ref()
    }

    pub fn project_name(&self) -> &str {
        self.project()
            .map(|project| project.name.as_str())
            .or_else(|| {
                self.project_path
                    .file_name()
                    .and_then(|name| name.to_str())
            })
            .unwrap_or("Unknown")
    }

    pub fn project_version(&self) -> &str {
        self.project()
            .map(|project| project.version.as_str())
            .unwrap_or("0.0.0")
    }

    pub fn script_count(&self) -> usize {
        self.project().map(|project| project.scripts.len()).unwrap_or(0)
    }

    pub fn dependency_count(&self) -> usize {
        self.project()
            .map(|project| {
                project.dependencies.len()
                    + project.dev_dependencies.len()
                    + project.peer_dependencies.len()
            })
            .unwrap_or(0)
    }

    pub fn node_version(&self) -> Option<&str> {
        self.runtime().and_then(|runtime| runtime.node_version.as_deref())
    }

    pub fn available_package_managers(&self) -> &[String] {
        self.runtime()
            .map(|runtime| runtime.available_package_managers.as_slice())
            .unwrap_or(&[])
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
                    self.dep_cursor = step(self.dep_cursor, delta, self.dependency_count());
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
    fn enter_content(&mut self) {
        if self.focus == Focus::Sidebar
            && matches!(self.screen, Screen::Scripts | Screen::Dependencies)
        {
            self.focus = Focus::Content;
        }
    }

    /// Return keyboard focus from the content pane back to the sidebar.
    fn exit_content(&mut self) {
        if self.focus == Focus::Content {
            self.focus = Focus::Sidebar;
        }
    }

    fn refresh_project(&mut self) {
        self.state.project = crate::project::package_json::load_from(&self.project_path)
            .ok()
            .map(ProjectInfo::from);
        self.script_cursor = 0;
        self.dep_cursor = 0;
    }

    pub fn script_name_at(&self, index: usize) -> Option<String> {
        self.project()
            .and_then(|project| project.scripts.get_index(index))
            .map(|(name, _)| name.clone())
    }

    /// Spawn the script queued by `Action::RunScript`, attaching its output receiver.
    pub fn start_pending_script(&mut self) {
        let Some(name) = self.pending_script.take() else {
            return;
        };

        let project = self.project();
        let runtime = self.runtime();
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
