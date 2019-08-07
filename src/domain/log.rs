use std::sync::{Arc, RwLock, Weak};

use crate::focus::{Focus, FocusLocator};

use crate::node::NodeValue;
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum NodeEvent {
    RootUpdated {
        txid: u64,
        focus: Arc<Focus>,
        value: Arc<NodeValue>,
    },
    ValueCreated {
        txid: u64,
        focus: Arc<Focus>,
        value: Arc<NodeValue>,
    },
    ValueUpdated {
        txid: u64,
        focus: Arc<Focus>,
        value: Arc<NodeValue>,
    },
    ValueDeleted {
        txid: u64,
        focus: Arc<Focus>,
        value: Arc<NodeValue>,
    },
    ListItemInserted {
        txid: u64,
        focus: Arc<Focus>,
        value: Arc<NodeValue>,
    },
    ListItemDeleted {
        txid: u64,
        focus: Arc<Focus>,
        value: Arc<NodeValue>,
    },
    InternalNodeUpdated {
        txid: u64,
        focus: Arc<Focus>,
        value: Arc<NodeValue>,
    },
    InternalRootUpdated {
        txid: u64,
        focus: Arc<Focus>,
        value: Arc<NodeValue>,
    },

}

pub struct ChangeLogger {
    // pub pending: RwLock<HashMap<Arc<Focus>, Vec<UpdatePending>>>,
    pub changed: RwLock<HashMap<Arc<NodeValue>, Arc<NodeValue>>>, // new to old
    pub parents: RwLock<HashMap<Arc<NodeValue>, Arc<NodeValue>>>, // child to parent
    // pub focus_tx: HashMap<Arc<Focus>, RwLock<FocusTx>>,
    pub txid_max: RwLock<u64>,
    pub log: RwLock<Vec<NodeEvent>>,
}

impl ChangeLogger {
    pub fn new() -> ChangeLogger {
        ChangeLogger {
            txid_max: RwLock::new(0),
            log: RwLock::new(Vec::new()),
            // focus_tx: HashMap::new(),
            changed: RwLock::new(HashMap::new()),
            parents: RwLock::new(HashMap::new()),
            // pending: RwLock::new(HashMap::new()),
        }
    }

    pub fn new_txid(&self) -> u64 {
        let mut txid_max = self.txid_max.write().unwrap();
        *txid_max += 1;
        *txid_max
    }

    pub fn push(&self, event: NodeEvent) {
        let mut log = self.log.write().unwrap();
        log.push(event);
    }


    pub fn foreach<F>(&self, mut func: F)
    where
        F: FnMut(&ChangeLogger, &NodeEvent),
    {
        for event in self.log.read().unwrap().iter().rev() {
            func(self, event)
        }
    }

    pub fn print_history(&self) {
        let parents = self.parents.read().unwrap();
        let changed = self.changed.read().unwrap();

        // for (k, v) in parents.iter() {
        //     println!("{:p} => {:p}", k.as_ref(), v.as_ref());
        // }

        use NodeEvent::*;

        for event in self.log.read().unwrap().iter().rev() {
            let (changed_type, txid, focus, value) = match event {
                ValueCreated { txid, focus, value } => ("VC", txid, focus, value),
                ValueUpdated { txid, focus, value } => ("VU", txid, focus, value),
                ValueDeleted { txid, focus, value } => ("VD", txid, focus, value),
                ListItemInserted { txid, focus, value } => ("LI", txid, focus, value),
                ListItemDeleted { txid, focus, value } => ("LD", txid, focus, value),
                RootUpdated { txid, focus, value } => ("RU", txid, focus, value),
                InternalNodeUpdated { txid, focus, value } => ("IU", txid, focus, value),
                InternalRootUpdated { txid, focus, value } => ("IR", txid, focus, value),
            };

            print!("[{}]#{:^2} ", changed_type, txid);
            print!("{:p}", value.as_ref());

            if let Some(parent_node) = parents.get(value) {
                print!(" _^{:p}", parent_node.as_ref());
            } else {
                print!(" {:16}", " ");
            }

            if let Some(changed_to) = changed.get(value) {
                print!(" => {:p}", changed_to.as_ref());
            } else {
                print!("  {:16}", " ");
            }

            println!(" '{}'", focus.access_path());
        }
    }
}

// impl ::std::fmt::Debug for NodeEvent {
//     fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
//     }
// }

// pub struct EventLogIter<'a> {
//     iter: std::slice::Iter<'a, NodeEvent>,
// }

// impl<'a> Iterator for EventLogIter<'a> {
//     type Item = &'a NodeEvent;
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
