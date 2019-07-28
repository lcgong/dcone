use std::sync::Arc;
use std::cell::RefCell;
use crate::log::ChangeLogger;

use crate::cell::node::NodeValue;
// use crate::cell::cell::ValueCell;

#[derive(PartialEq)]
pub struct Domain {
    pub logger: ChangeLogger,
    // pub root: Arc<RefCell<Arc<NodeValue>>>
    pub root: RefCell<Arc<NodeValue>>
    
}

impl Domain {
    pub fn new() -> Arc<Domain> {
        Arc::new(Domain {
            logger: ChangeLogger::new(),
            root: RefCell::new(Arc::new(NodeValue::None))
        })
    }
}
