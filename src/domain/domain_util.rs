
use std::sync::Arc;
use std::cell::RefCell;
use crate::focus::Focus;
use crate::node::NodeValue;
use crate::cell::ValueCell;
use crate::error::Error;

use super::domain::Domain;
use super::log::ChangeLogger;

pub struct DomainUtil(Arc<Domain>);

impl DomainUtil {
    pub fn new() -> Self { 
        DomainUtil(Arc::new(Domain {
            logger: ChangeLogger::new(),
            root_node: RefCell::new(Arc::new(NodeValue::None)),
            root_focus: Focus::new()
        }))
    }

    pub fn root(&self) -> ValueCell {
        ValueCell {
            domain: self.0.clone(),
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

