#[derive(Debug, Clone)]
pub enum Action {
    Quit,

    NavigateUp,
    NavigateDown,
    NavigateLeft,
    NavigateRight,

    Confirm,
    Back,

    Refresh,

    OpenDashboard,
    OpenScripts,
    OpenDependencies,
    OpenDoctor,

    RunScript(String),
}