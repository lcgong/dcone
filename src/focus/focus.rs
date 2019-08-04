use std::sync::{Arc, Weak, RwLock};
use std::collections::HashMap;

use super::access_key::AccessKey;

pub struct Focus {
    pub(crate) parent_focus: Option<Arc<Focus>>,
    pub(crate) access_key: AccessKey,
    pub(super) directions: RwLock<HashMap<AccessKey, Weak<Focus>>>,
}

impl Focus {
    pub(crate) fn new() -> Arc<Focus> {
        Arc::new(Focus {
            parent_focus: None,
            access_key: AccessKey::None,
            directions: RwLock::new(HashMap::new()),
        })
    }
}


impl core::cmp::PartialEq for Focus {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl std::hash::Hash for Focus {
    fn hash<H: std::hash::Hasher>(&self, into: &mut H) {
        std::ptr::hash(&self, into)
    }
}


impl ::std::fmt::Debug for Focus {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("<Focus ")?;
        self.access_key.fmt(fmt)?;
        fmt.write_fmt(format_args!(" at {:p}", self))?;
        if let Some(parent_focus) = &self.parent_focus {
            fmt.write_fmt(format_args!(" <= {:p}", parent_focus.as_ref()))?;
        }

        fmt.write_str(" >")
    }
}

impl Drop for Focus {
    fn drop(&mut self) {
        if let Some(ref parent_focus) = self.parent_focus {
            let mut directions = parent_focus.directions.write().unwrap();
            directions.remove(&self.access_key);
            // println!("Drop {:?}", self.access_key);
        }
    }
}

