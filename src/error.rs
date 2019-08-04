
// 
use crate::focus::AccessPathError;
use std::sync::Arc;
use crate::focus::{Focus, AccessKey, FocusLocator};


#[derive(Debug, PartialEq)]
pub enum Error {
    NoSuchItem {
        focus: Arc<Focus>,
        access_key: AccessKey
    },
    FailedToNavigate {
        from: Arc<Focus>,
        to: Arc<Focus>
    },
    WrongItemAccess {
        focus: Arc<Focus>,
        access_key: AccessKey
    },
    CollectionRequired {
        focus: Arc<Focus>,
    },
    AccessPathError(AccessPathError),


    // MismatchedType,
    // OverFocus(OverFocusError),
    // Parsing(PathParsingError)
}


impl Error {
    
    #[inline]
    pub fn no_such_item<T>(parent_focus: &Arc<Focus>, access_key: &AccessKey) -> Result<T, Error> {
        Err(Error::NoSuchItem {focus: parent_focus.clone(), access_key: access_key.clone()})
    }

    pub fn mismatched_access_key<T>(parent_focus: &Arc<Focus>, access_key: &AccessKey) -> Result<T, Error> {
        Err(Error::WrongItemAccess {focus: parent_focus.clone(), access_key: access_key.clone()})
    }

    pub fn should_be_collection<T>(parent_focus: &Arc<Focus>) -> Result<T, Error> {
        Err(Error::CollectionRequired {focus: parent_focus.clone()})
    }
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Error::*;

        match self {
            NoSuchItem {focus, access_key}   => {
                write!(f, "No such item {} in {}", access_key, focus.access_path())
            },
            FailedToNavigate {from, to}   => {
                write!(f, "Failed to navigate {} from {}", 
                        to.access_path(), from.access_path())
            },
            WrongItemAccess {focus, access_key}   => {
                match access_key {
                    AccessKey::Key(_) => {
                        write!(f, "The parent node '{}' of {} should be a Map", 
                                focus.access_path(), access_key)
                    },
                    AccessKey::Index(_) => {
                        write!(f, "The parent node '{}' of {} should be a List", 
                                focus.access_path(), access_key)
                    },
                    AccessKey::None => {
                        write!(f, "To access item should be a Key or index: {}", 
                                focus.access_path())
                    },
                }
            },
            CollectionRequired {focus} => {
                write!(f, "The node should be a Map or List: {}", 
                                focus.access_path())
            },
            AccessPathError(err) => {
                write!(f, "{}", err)
            }
            // UnexpectedCharacter {
            //     ref ch,
            //     ref line,
            //     ref column,
            // } => write!(f, "Unexpected character: {} at ({}:{})", ch, line, column),
            // UnexpectedEndOfJson   => write!(f, "Unexpected end of JSON"),
            // ExceededDepthLimit    => write!(f, "Exceeded depth limit"),
            // FailedUtf8Parsing     => write!(f, "Failed to parse UTF-8 bytes"),
            // WrongType(ref s)      => write!(f, "Wrong type, expected: {}", s),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        use Error::*;

        match *self {
            NoSuchItem { .. } => "No such item in parent collection",
            FailedToNavigate { .. } => "Failed to navigate the focus from other",
            WrongItemAccess {..} => "wrong item access",
            CollectionRequired {..} => "The node should be a Map or List",
            AccessPathError(_) => "access path error",

            // UnexpectedEndOfJson        => "Unexpected end of JSON",
            // ExceededDepthLimit         => "Exceeded depth limit",
            // FailedUtf8Parsing          => "Failed to read bytes as UTF-8 from JSON",
            // WrongType(_)               => "Wrong type",
        }
    }
}
