use std::sync::Arc;
use std::cell::RefCell;
use crate::log::ChangeLogger;
use crate::focus::Focus;

use crate::cell::ValueCell;
use crate::node::NodeValue;
// use crate::cell::cell::ValueCell;

#[derive(PartialEq)]
pub struct Domain {
    pub logger: ChangeLogger,
    pub root_node: RefCell<Arc<NodeValue>>,
    pub root_focus: Arc<Focus>
    
}

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
        let root_node = self.0.root_node.borrow().clone();
        ValueCell {
            domain: self.0.clone(),
            focus: self.0.root_focus.clone(),
            node: root_node
        }
    }

}

impl Domain {
    pub fn new() -> Arc<Domain> {
        Arc::new(Domain {
            logger: ChangeLogger::new(),
            root_node: RefCell::new(Arc::new(NodeValue::None)),
            root_focus: Focus::new()
        })
    }
}

pub(crate) fn set_domain_root(domain: &Arc<Domain>, value: Arc<NodeValue>) {

    let root_focus = domain.root_focus.clone();
    // let old_root = domain.root_node.replace(value);
    let mut root_node = domain.root_node.borrow_mut();
    
    domain.logger.value_changed(&root_node, &root_focus, &value, &root_node);

    *root_node = value;
}  

pub(crate) fn get_domain_root(domain: &Arc<Domain>) -> Arc<NodeValue> {
    domain.root_node.borrow().clone()
}