

#[derive(Debug, Clone)]
pub struct PathParsingError {
    pub(super) path: String,
    pub(super) err: std::num::ParseIntError
}

#[derive(Debug, Clone)]
pub struct OverFocusError {
    pub(super) path: String,
}


#[derive(Debug, Clone)]
pub enum AccessPathError {
    OverFocus(OverFocusError),
    Parsing(PathParsingError)
}

impl std::fmt::Display for OverFocusError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Over the root of the path: {}", self.path)
    }
}

impl std::fmt::Display for PathParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "parsing error: {}", self.path)
    }
}

impl std::fmt::Display for AccessPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AccessPathError::OverFocus(err) => err.fmt(f),
            AccessPathError::Parsing(err) => err.fmt(f)
        }
    }
}

impl std::error::Error for PathParsingError {
}

impl std::error::Error for OverFocusError {
}

impl std::error::Error for AccessPathError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            AccessPathError::OverFocus(ref err) => Some(err),
            AccessPathError::Parsing(ref err) => Some(err),
        }
    }
}
