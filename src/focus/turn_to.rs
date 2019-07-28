

use std::sync::Arc;
use regex::Regex;

use super::access_key::{AccessKey, CircularZeroIndex};
use super::focus::Focus;

use super::error::{AccessPathError, PathParsingError, OverFocusError};
use super::locator::FocusLocator;

pub trait FocusTurnTo {
    fn turn_to<S: AsRef<str>>(&self, path: S) -> Result<Arc<Focus>, AccessPathError>;
}


impl FocusTurnTo for Arc<Focus> {
    fn turn_to<S: AsRef<str>>(&self, path: S) -> Result<Arc<Focus>, AccessPathError> {
        let path = path.as_ref();

        let path_regex: Regex = Regex::new(r"(\.{2})/?|(?:#(\d+)/?)|(?:([^#/]+)/?)").unwrap();

        let new_focus;
        let rel_path;
        if &path[0..1] == "/" {
            rel_path = &path[1..];
            new_focus = self.get_root();
        } else {
            rel_path = path.as_ref();
            new_focus = &self;
        }

        let mut new_focus = new_focus.clone();

        for caps in path_regex.captures_iter(rel_path) {
            if let Some(matched) = caps.get(3) { // key
                let key = AccessKey::Key(matched.as_str().to_string());

                new_focus = new_focus.focus(key);
            } else if let Some(matched) = caps.get(2) { // index
                let idx_str = matched.as_str();

                match CircularZeroIndex::from_str_radix(idx_str, 10) {
                    Ok(index) => {
                        let index = AccessKey::Index(index);
                        new_focus = new_focus.focus(index);
                    }
                    Err(err) => {
                        return Err(AccessPathError::Parsing(PathParsingError {
                            path: path.to_string(),
                            err: err
                        }));
                    }
                }
            } else if let Some(matched) = caps.get(1) { // parent symbol (..)
                println!("xxx: {:?}", matched.as_str());

                new_focus = match new_focus.parent_focus {
                    Some(ref parent) => parent.clone(),
                    None => {
                        return Err(AccessPathError::OverFocus(OverFocusError {
                            path: path.to_string(),
                        }));
                    }
                };
            }
        }

        Ok(new_focus)
    }
}

impl FocusTurnTo for Result<Arc<Focus>, AccessPathError> {
    fn turn_to<S: AsRef<str>>(&self, path: S) -> Result<Arc<Focus>, AccessPathError> {
        match self {
            Ok(focus) => focus.turn_to(path),
            Err(err) => Err(err.clone()),
        }
    }
}
