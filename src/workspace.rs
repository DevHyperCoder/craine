///adsfasdf
use super::error_handler::ErrorType;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

/**
 * Contains Workspace related configuration
 *
 * - build_dir: Directory to output final, compiled HTML
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceConfig {
    /**
     * - build_dir: Directory to output final, compiled HTML
     */
    pub build_dir: Option<PathBuf>,
}

impl WorkspaceConfig {
    /**
     * Returns a new WorkspaceConfig
     * - build_dir = "./build"
     */
    fn new() -> Self {
         WorkspaceConfig {
            build_dir: Some(PathBuf::new().join("./build")),
        }
    }
}

/**
 * Returns a <PathBuf> from the first argument given to program
 * NOTE: env[0] is the shell/program
 * */
pub fn get_work_dir() -> Option<PathBuf> {
    let i: Vec<String> = env::args().collect();

    let work_dir = PathBuf::from(&i[1]);

    if !work_dir.is_dir() {
        return None;
    }

    Some(work_dir)
}

/**
 * Returns a Option<PathBuf> containing workspace config
 *
 * Order:
 * - .craine
 * - craine.json
 */
pub fn get_workspace_config_path(work_dir: PathBuf) -> Option<PathBuf> {
    let dot_craine = PathBuf::new().join(&work_dir).join(".craine");
    let craine_json = PathBuf::new().join(&work_dir).join("craine.json");

    if dot_craine.exists() {
        return Some(dot_craine);
    }

    if craine_json.exists() {
        return Some(craine_json);
    }
    None
}

/**
 * Returns Result<WorkspaceConfig,ErrorType> of the workspace configuration
 * If there is no workspace config file, (via `get_workspace_config_path()`), a new WorkspaceConfig is returned.
 */
pub fn get_workspace_config(work_dir: PathBuf) -> Result<WorkspaceConfig, ErrorType> {
    let path = get_workspace_config_path(work_dir);

    match path {
        Some(path) => match serde_json::from_str(&fs::read_to_string(path).unwrap()) {
            Ok(workspace) => Ok(workspace),
            Err(_) => Err(ErrorType::Workspace("Can not parse workspace config file")),
        },
        None => {
             Ok(WorkspaceConfig::new())
        }
    }
}
