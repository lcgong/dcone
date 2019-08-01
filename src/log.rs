use std::sync::Arc;
use std::cell::RefCell;

use crate::focus::{AccessKey, Focus, FocusLocator};

use crate::node::NodeValue;

#[derive(PartialEq, Debug)]
pub enum ValueChangeEvent {
    Created {
        access_key: AccessKey,
        new_value: Arc<NodeValue>
    },
    Changed {
        access_key: AccessKey,
        new_value: Arc<NodeValue>,
        original: Arc<NodeValue>
    },
    Removed {
        access_key: AccessKey,
        original: Arc<NodeValue>
    }
}

type EventLog = Arc<RefCell<Vec<ValueChangeEvent>>>;


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

    fn push(&self, event: ValueChangeEvent) {
        self.log.borrow_mut().push(event);
    }

    pub fn log_value_created(&self, access_key: AccessKey, new_value: Arc<NodeValue>) {
        self.push(ValueChangeEvent::Created {
            access_key: access_key,
            new_value: new_value
        });
    }

    pub fn value_created(&self, root: &Arc<NodeValue>, focus: &Arc<Focus>, new_value: &Arc<NodeValue>) {
        let access_key = focus.get_access_key();
        self.push(ValueChangeEvent::Created {
            access_key: access_key,
            new_value: new_value.clone()
        });
    }

    pub fn value_changed(&self, root: &Arc<NodeValue>, focus: &Arc<Focus>, 
        new_value: &Arc<NodeValue>, original: &Arc<NodeValue>) {
        
        let access_key = focus.get_access_key();
        self.push(ValueChangeEvent::Changed {
            access_key: access_key,
            new_value: new_value.clone(),
            original: original.clone()
        });
    }

    pub fn log_value_changed(&self, access_key: AccessKey, new_value: Arc<NodeValue>, original: Arc<NodeValue>) {
        
        self.push(ValueChangeEvent::Changed {
            access_key: access_key,
            new_value: new_value,
            original: original
        });
    }

    pub fn log_value_removed(&self, access_key: AccessKey, original: Arc<NodeValue>) {
        self.push(ValueChangeEvent::Removed {
            access_key: access_key,
            original: original
        });
    }    

}


// pub struct EventLogIter {
//     iter: std::vec::IntoIter<ValueChangeEvent>
//     // iter: std::slice::Iter<'a, ValueChangeEvent>,
// }

// impl Iterator for EventLogIter {
//     type Item = ValueChangeEvent;
//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(event) = self.iter.next() {
//             Some(event)
//         } else {
//             None
//         }
//     }
// }

// impl<'a> std::iter::IntoIterator for ChangeLogger {
//     type Item = &'a ValueChangeEvent;
//     type IntoIter = EventLogIter;

//     // #[inline]
//     fn into_iter(self) -> Self::IntoIter {
//         // let a: std::cell::RefMut<Vec<ValueChangeEvent>> = self.log.borrow_mut();
//         // let b: i64 = a.into_iter();
//         EventLogIter {
//             iter: self.log.clone().borrow().into_iter(),
//         }
//     }
// }
