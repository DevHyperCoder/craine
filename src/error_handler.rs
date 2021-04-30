use std::fmt;

#[derive(Debug)]
/**
 * ErrorTypes that can be generated during runtime
 */
pub enum ErrorType {
    /**
     * Any error related to work directory like, unable to open, permission etc
     */
    WorkDir(&'static str),
    /**
     * Any error related to workspace configuration files
     */
    Workspace(&'static str),
    /**
     * Permission errors or unable to open build dir
     */
    BuildDir(&'static str),
    /**
     * Any error during parsing or writing to file
     */
    Parse(&'static str),
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dis = match self {
            ErrorType::WorkDir(s) => format!("[work_dir] {}", s),
            ErrorType::Workspace(s) => format!("[workspace] {}", s),
            ErrorType::BuildDir(s) => format!("[build_dir] {}", s),
            ErrorType::Parse(s) => format!("[parse] {}", s),
        };
        write!(f, "{}", dis)
    }
}
