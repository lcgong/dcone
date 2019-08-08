use std::sync::{Arc};

use super::access_key::AccessKey;
use super::focus::Focus;
use super::locator::FocusLocator;

use std::cmp::Ordering;


impl core::cmp::PartialEq for Focus {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl core::cmp::Eq for Focus {
}

impl std::hash::Hash for Focus {
    fn hash<H: std::hash::Hasher>(&self, into: &mut H) {
        std::ptr::hash(self, into)
    }
}



impl std::cmp::PartialOrd for AccessKey {
    fn partial_cmp(&self, other: &AccessKey) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
        // match (self, other) {
        //     (AccessKey::Key(a), AccessKey::Key(b)) => a.partial_cmp(b),
        //     (AccessKey::Index(a), AccessKey::Index(b)) => a.partial_cmp(b),
        //     _ => None,
        // }
    }
}

impl std::cmp::Ord for AccessKey {
    fn cmp(&self, other: &AccessKey) -> std::cmp::Ordering {
        match (self, other) {
            (AccessKey::Key(a), AccessKey::Key(b)) => a.cmp(b),
            (AccessKey::Index(a), AccessKey::Index(b)) => a.cmp(b),
            (AccessKey::None, AccessKey::None) => Ordering::Equal,
            (AccessKey::Index(_), AccessKey::Key(_)) => Ordering::Less,
            (AccessKey::Key(_), AccessKey::Index(_)) => Ordering::Greater,
            (AccessKey::None, AccessKey::Key(_)) => Ordering::Less,
            (AccessKey::None, AccessKey::Index(_)) => Ordering::Less,
             (AccessKey::Index(_), AccessKey::None) => Ordering::Greater,
            (AccessKey::Key(_), AccessKey::None) => Ordering::Greater,
        }
    }
}


fn access_segments<'a>(focus: &'a Focus) -> Vec<&'a AccessKey> {

    let mut segments = Vec::new();
    let mut current_focus = focus;
    loop {
        match current_focus.parent_focus {
            Some(ref parent_focus) => {
                segments.push(&current_focus.access_key);
                current_focus = parent_focus;
            },
            None => {
                break;
            }
        }
    }

    segments.reverse();
    segments
}

impl PartialOrd for Focus {
    fn partial_cmp(&self, other: &Focus) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl std::cmp::Ord for Focus {
    fn cmp(&self, other: &Focus) -> Ordering {
        let self_segments = access_segments(self);
        let other_segments = access_segments(other);
        self_segments.cmp(&other_segments)
    }
}