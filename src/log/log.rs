use std::sync::Arc;
use std::cell::RefCell;

use crate::focus::{Focus};

use crate::node::NodeValue;

#[derive(PartialEq, Debug)]
pub enum NodeEvent {
    Created {
        focus: Arc<Focus>,
        parent: Arc<NodeValue>,
        value: Arc<NodeValue>
    },
    Changed {
        focus: Arc<Focus>,
        parent: Arc<NodeValue>,
        from: Arc<NodeValue>,
        value: Arc<NodeValue>
    },
    Removed {
        focus: Arc<Focus>,
        parent: Arc<NodeValue>,
        from: Arc<NodeValue>
    }
}

type EventLog = Arc<RefCell<Vec<NodeEvent>>>;


#[derive(PartialEq)]
pub struct ChangeLogger {
    pub log: EventLog
}


impl ChangeLogger {
    pub fn new() -> ChangeLogger {
        ChangeLogger {
            log: Arc::new(RefCell::new(Vec::new()))
        }
    }

    fn push(&self, event: NodeEvent) {
        self.log.borrow_mut().push(event);
    }

    pub fn node_created(
        &self, 
        focus: &Arc<Focus>,  
        parent: &Arc<NodeValue>, 
        value: &Arc<NodeValue>
    ) {
        self.push(NodeEvent::Created {
            focus: focus.clone(),
            parent: parent.clone(),
            value: value.clone()
        });
    }

    pub fn node_changed(
        &self, 
        focus: &Arc<Focus>, 
        parent: &Arc<NodeValue>, 
        value: &Arc<NodeValue>, 
        from: &Arc<NodeValue>
    ) {
        
        self.push(NodeEvent::Changed {
            focus: focus.clone(),
            parent: parent.clone(),
            from: from.clone(),
            value: value.clone()
        });
    }

    pub fn node_removed(&self, 
        focus: &Arc<Focus>,  
        parent: &Arc<NodeValue>, 
        from: &Arc<NodeValue>
    ) {
        self.push(NodeEvent::Removed {
            focus: focus.clone(),
            parent: parent.clone(),
            from: from.clone(),
        });
    }    
}


// pub struct EventLogIter {
//     iter: std::vec::IntoIter<NodeEvent>
//     // iter: std::slice::Iter<'a, NodeEvent>,
// }

// impl Iterator for EventLogIter {
//     type Item = NodeEvent;
//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(event) = self.iter.next() {
//             Some(event)
//         } else {
//             None
//         }
//     }
// }

// impl<'a> std::iter::IntoIterator for ChangeLogger {
//     type Item = &'a NodeEvent;
//     type IntoIter = EventLogIter;

//     // #[inline]
//     fn into_iter(self) -> Self::IntoIter {
//         // let a: std::cell::RefMut<Vec<NodeEvent>> = self.log.borrow_mut();
//         // let b: i64 = a.into_iter();
//         EventLogIter {
//             iter: self.log.clone().borrow().into_iter(),
//         }
//     }
// }
