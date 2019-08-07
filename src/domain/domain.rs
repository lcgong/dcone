
use std::sync::Arc;
use std::cell::RefCell;
use crate::focus::Focus;
use crate::node::NodeValue;
use crate::cell::ValueCell;
use crate::error::Error;

use super::cone::Cone;
use super::log::ChangeLogger;

pub struct Domain(Arc<Cone>);

impl Domain {
    pub fn new() -> Self { 
        Domain(Cone::new())
    }

    pub fn root(&self) -> ValueCell {
        ValueCell {
            cone: self.0.clone(),
            focus: self.0.root_focus.clone(),
            node: self.0.get_root_node(),
            parent: None,
        }
    }

    #[inline]
    pub fn navigate(&self, path: &str) -> Result<ValueCell, Error> {
        self.root().navigate(path)
    }

    pub fn log(&self) -> &ChangeLogger {
        &self.0.logger
    }
}

