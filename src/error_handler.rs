use std::fmt;

#[derive(Debug)]
pub enum ErrorType {
    WorkDir(&'static str),
    Workspace(&'static str),
    BuildDir(&'static str),
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
