use std::path::{Path, PathBuf};

use crate::project::info::ProjectInfo;
use crate::system::runtime::RuntimeInfo;

/// Runtime context for the currently opened project.
///
/// This keeps project-related environment data together so UI components do
/// not need to know where individual pieces of information come from.
#[derive(Debug)]
pub struct AppContext {
    pub project: ProjectContext,
    pub runtime: Option<RuntimeInfo>,
}

impl AppContext {
    pub fn new(
        project_path: PathBuf,
        project: Option<ProjectInfo>,
        runtime: Option<RuntimeInfo>,
    ) -> Self {
        Self {
            project: ProjectContext::new(project_path, project),
            runtime,
        }
    }

    pub fn project_name(&self) -> &str {
        self.project.name()
    }

    pub fn project_version(&self) -> &str {
        self.project
            .info
            .as_ref()
            .map(|project| project.version.as_str())
            .unwrap_or("0.0.0")
    }

    pub fn script_count(&self) -> usize {
        self.project
            .info
            .as_ref()
            .map(|project| project.scripts.len())
            .unwrap_or(0)
    }

    pub fn dependency_count(&self) -> usize {
        self.project
            .info
            .as_ref()
            .map(|project| {
                project.dependencies.len()
                    + project.dev_dependencies.len()
                    + project.peer_dependencies.len()
            })
            .unwrap_or(0)
    }
}

#[derive(Debug)]
pub struct ProjectContext {
    pub path: PathBuf,
    pub info: Option<ProjectInfo>,
}

impl ProjectContext {
    pub fn new(path: PathBuf, info: Option<ProjectInfo>) -> Self {
        Self { path, info }
    }

    pub fn name(&self) -> &str {
        self.info
            .as_ref()
            .map(|project| project.name.as_str())
            .or_else(|| self.directory_name())
            .unwrap_or("Unknown")
    }

    fn directory_name(&self) -> Option<&str> {
        self.path.file_name().and_then(|name| name.to_str())
    }

    pub fn reload(&mut self) {
        self.info = crate::project::package_json::load_from(&self.path)
            .ok()
            .map(ProjectInfo::from);
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
