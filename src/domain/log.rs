use std::sync::{Arc, RwLock};

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
    InternalLineUpdated {
        txid: u64,
        focus: Arc<Focus>,
        old_node: Arc<NodeValue>,
        new_node: Arc<NodeValue>,
    },
    InternalRootUpdated {
        txid: u64,
        focus: Arc<Focus>,
        value: Arc<NodeValue>,
    },
}

#[derive(Debug, Clone)]
pub struct PendingUpdate {
    pub focus: Arc<Focus>,
    pub old_node: Arc<NodeValue>,
    pub new_node: Arc<NodeValue>,
}

pub struct ChangeLogger {
    pub pending: RwLock<HashMap<Arc<Focus>, Vec<PendingUpdate>>>,
    pub changed: RwLock<HashMap<Arc<NodeValue>, Arc<NodeValue>>>, // new to old
    pub parents: RwLock<HashMap<Arc<NodeValue>, Arc<NodeValue>>>, // child to parent
    // pub focus_tx: HashMap<Arc<Focus>, RwLock<FocusTx>>,
    pub txid_max: RwLock<u64>,
    pub log: RwLock<Vec<NodeEvent>>,
    // pub pending: RwLock<Vec<PendingUpdate>>,
}

impl ChangeLogger {
    pub fn new() -> ChangeLogger {
        ChangeLogger {
            txid_max: RwLock::new(0),
            log: RwLock::new(Vec::new()),
            // focus_tx: HashMap::new(),
            changed: RwLock::new(HashMap::new()),
            parents: RwLock::new(HashMap::new()),
            pending: RwLock::new(HashMap::new()),
            // pending: RwLock::new(Vec::new()),
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

        let pending = self.pending.read().unwrap();

        println!("Pending:");
        for (focus, updates) in pending.iter() {
            println!("{:?}", focus.access_path());
            for upd in updates.iter() {
                println!(
                    "   {:p} => {:p}",
                    upd.old_node,
                    upd.new_node,
                );
            }
        }

        println!("Change:");
        use NodeEvent::*;

        for event in self.log.read().unwrap().iter().rev() {
            if let InternalLineUpdated { txid, focus, old_node, new_node } = event {
                print!("[{}]#{:^2} ", "IL", txid);
                print!(" {:p} => ", old_node.as_ref());
                print!("{:p}", new_node.as_ref());
                print!(" {:16}", " ");

                println!(" '{}'", focus.access_path());
                continue;
            }

            let (changed_type, txid, focus, value) = match event {
                ValueCreated { txid, focus, value } => ("VC", txid, focus, value),
                ValueUpdated { txid, focus, value } => ("VU", txid, focus, value),
                ValueDeleted { txid, focus, value } => ("VD", txid, focus, value),
                ListItemInserted { txid, focus, value } => ("LI", txid, focus, value),
                ListItemDeleted { txid, focus, value } => ("LD", txid, focus, value),
                RootUpdated { txid, focus, value } => ("RU", txid, focus, value),
                InternalNodeUpdated { txid, focus, value } => ("IU", txid, focus, value),
                InternalRootUpdated { txid, focus, value } => ("IR", txid, focus, value),
                InternalLineUpdated { txid, focus, old_node, new_node } => {
                    ("IL", txid, focus, new_node)
                },
            };

            print!("[{}]#{:^2} ", changed_type, txid);
            if let Some(changed_from) = changed.get(value) {
                print!(" {:p} => ", changed_from.as_ref());
            } else {
                print!("  {:16} ", " ");
            }

            print!("{:p}", value.as_ref());

            if let Some(parent_node) = parents.get(value) {
                print!(" _^{:p}", parent_node.as_ref());
            } else {
                print!(" {:16}", " ");
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
