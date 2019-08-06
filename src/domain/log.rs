use std::sync::{Arc, RwLock, Weak};

use crate::focus::{Focus, FocusLocator};

use crate::node::NodeValue;
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum NodeEvent {
    Created {
        txid: u64,
        focus: Arc<Focus>,
        value: Arc<NodeValue>,
    },
    Updated {
        txid: u64,
        focus: Arc<Focus>,
        value: Arc<NodeValue>,
    },
    Deleted {
        txid: u64,
        focus: Arc<Focus>,
        value: Arc<NodeValue>,
    },
}

pub struct FocusTx {
    txid: u64,
    children_max: u64,
}

pub struct UpdatePending {
    focus: Arc<Focus>,
    new_value: Arc<NodeValue>,
    old_value: Arc<NodeValue>,
}

pub struct ChangeLogger {
    pub pending: RwLock<HashMap<Arc<Focus>, Vec<UpdatePending>>>,
    pub changed: RwLock<HashMap<Arc<NodeValue>, Arc<NodeValue>>>, // old to new
    pub parents: RwLock<HashMap<Arc<NodeValue>, Arc<NodeValue>>>, // child to parent
    pub focus_tx: HashMap<Arc<Focus>, RwLock<FocusTx>>,
    pub txid_max: RwLock<u64>,
    pub log: RwLock<Vec<NodeEvent>>,
}

impl ChangeLogger {
    pub fn new() -> ChangeLogger {
        ChangeLogger {
            txid_max: RwLock::new(0),
            log: RwLock::new(Vec::new()),
            focus_tx: HashMap::new(),
            changed: RwLock::new(HashMap::new()),
            parents: RwLock::new(HashMap::new()),
            pending: RwLock::new(HashMap::new()),
        }
    }

    fn new_txid(&self) -> u64 {
        let mut txid_max = self.txid_max.write().unwrap();
        *txid_max += 1;
        *txid_max
    }

    fn push(&self, event: NodeEvent) {
        let mut log = self.log.write().unwrap();
        log.push(event);
    }

    // pub fn pending_update(
    //     &self,
    //     focus: &Arc<Focus>,
    //     new_value: &Arc<NodeValue>,
    //     old_value: &Arc<NodeValue>,
    // ) {
    //     match self.pending.get(focus)
    // }

    pub fn node_created(
        &self,
        focus: &Arc<Focus>,
        parent: &Arc<NodeValue>,
        value: &Arc<NodeValue>,
    ) {
        let txid = self.new_txid();
        self.push(NodeEvent::Created {
            txid: txid,
            focus: focus.clone(),
            value: value.clone(),
        });

        let mut parents = self.parents.write().unwrap();
        parents.insert(value.clone(), parent.clone());
    }

    pub fn node_updated(
        &self,
        focus: &Arc<Focus>,
        parent: &Arc<NodeValue>,
        value: &Arc<NodeValue>,
        from: &Arc<NodeValue>,
    ) {
        let txid = self.new_txid();
        self.push(NodeEvent::Updated {
            txid: txid,
            focus: focus.clone(),
            value: value.clone(),
        });

        let mut parents = self.parents.write().unwrap();
        parents.insert(value.clone(), parent.clone());

        let mut changed = self.changed.write().unwrap();
        changed.insert(from.clone(), value.clone());
    }

    pub fn node_deleted(
        &self,
        focus: &Arc<Focus>,
        parent: &Arc<NodeValue>,
        value: &Arc<NodeValue>,
    ) {
        let txid = self.new_txid();

        self.push(NodeEvent::Deleted {
            txid: txid,
            focus: focus.clone(),
            value: value.clone(),
        });
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

        for event in self.log.read().unwrap().iter().rev() {
            let (changed_type, txid, focus, value) = match event {
                NodeEvent::Created { txid, focus, value } => ("C", txid, focus, value),
                NodeEvent::Updated { txid, focus, value } => ("U", txid, focus, value),
                NodeEvent::Deleted { txid, focus, value } => ("D", txid, focus, value),
            };

            print!("[{}]#{:^2} ", changed_type, txid);
            print!("{:p}", value.as_ref());

            if let Some(parent_node) = parents.get(value) {
                print!(" _^{:p}", parent_node.as_ref());
            }

            if let Some(changed_to) = changed.get(value) {
                print!(" => {:p}", changed_to.as_ref());
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
