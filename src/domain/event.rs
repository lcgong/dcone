use super::domain::Domain;
use crate::focus::{AccessKey, Focus, FocusLocator};
use crate::node::NodeValue;
use std::sync::Arc;
// use crate::cell::upward_update::upward_update_nodes;

use super::log::NodeEvent;

impl Domain {
    pub(crate) fn log_root_updated(
        &self,
        focus: Arc<Focus>,
        old_value: Arc<NodeValue>,
        new_value: Arc<NodeValue>,
    ) {
        let logger = &self.logger;

        let txid = logger.new_txid();

        logger.push(NodeEvent::RootUpdated {
            txid: txid,
            focus: focus,
            value: new_value.clone(),
        });

        self.push_change(old_value, new_value);
    }

    pub(crate) fn log_value_created(
        &self,
        focus: &Arc<Focus>,
        old_parent: &Arc<NodeValue>,
        new_value: &Arc<NodeValue>,
        new_parent: &Arc<NodeValue>,
    ) {
        let logger = &self.logger;

        let txid = logger.new_txid();

        logger.push(NodeEvent::ValueCreated {
            txid: txid,
            focus: focus.clone(),
            value: new_value.clone(),
        });

        self.push_parent_node(new_value.clone(), new_parent.clone());

        self.pending_inode_update(
            focus.get_parent().unwrap().clone(),
            old_parent.clone(),
            new_parent.clone(),
        );
    }

    pub(crate) fn log_value_updated(
        &self,
        focus: Arc<Focus>,
        old_parent: Arc<NodeValue>,
        old_value: Arc<NodeValue>,
        new_value: Arc<NodeValue>,
        new_parent: Arc<NodeValue>,
    ) {
        let logger = &self.logger;

        let txid = logger.new_txid();

        logger.push(NodeEvent::ValueUpdated {
            txid: txid,
            focus: focus.clone(),
            value: new_value.clone(),
        });

        self.push_parent_node(new_value.clone(), new_parent.clone());
        self.push_change(old_value, new_value);

        self.pending_inode_update(
            focus.get_parent().unwrap().clone(),
            old_parent.clone(),
            new_parent.clone(),
        );
    }

    pub(crate) fn log_value_deleted(
        &self,
        focus: &Arc<Focus>,
        old_parent: &Arc<NodeValue>,
        old_value: &Arc<NodeValue>,
        new_parent: &Arc<NodeValue>,
    ) {
        let logger = &self.logger;

        let txid = logger.new_txid();

        logger.push(NodeEvent::ValueUpdated {
            txid: txid,
            focus: focus.clone(),
            value: old_value.clone(),
        });

        self.pending_inode_update(
            focus.get_parent().unwrap().clone(),
            old_parent.clone(),
            new_parent.clone(),
        );
    }

    pub(crate) fn log_listitem_inserted(
        &self,
        focus: &Arc<Focus>,
        old_parent: &Arc<NodeValue>,
        new_value: &Arc<NodeValue>,
        new_parent: &Arc<NodeValue>,
    ) {
        let logger = &self.logger;

        let txid = logger.new_txid();

        logger.push(NodeEvent::ListItemInserted {
            txid: txid,
            focus: focus.clone(),
            value: new_value.clone(),
        });

        self.push_parent_node(new_value.clone(), new_parent.clone());

        self.pending_inode_update(
            focus.get_parent().unwrap().clone(),
            old_parent.clone(),
            new_parent.clone(),
        );
    }

    pub(crate) fn log_listitem_deleted(
        &self,
        focus: &Arc<Focus>,
        old_parent: &Arc<NodeValue>,
        old_value: &Arc<NodeValue>,
        new_parent: &Arc<NodeValue>,
    ) {
        let logger = &self.logger;

        let txid = logger.new_txid();

        logger.push(NodeEvent::ListItemDeleted {
            txid: txid,
            focus: focus.clone(),
            value: old_value.clone(),
        });

        self.pending_inode_update(
            focus.get_parent().unwrap().clone(),
            old_parent.clone(),
            new_parent.clone(),
        );
    }

    pub(crate) fn log_inode_updated(
        &self,
        focus: Arc<Focus>,
        old_parent: Arc<NodeValue>,
        old_value: Arc<NodeValue>,
        new_value: Arc<NodeValue>,
        new_parent: Arc<NodeValue>,
    ) {
        let logger = &self.logger;

        let txid = logger.new_txid();

        logger.push(NodeEvent::InternalNodeUpdated {
            txid: txid,
            focus: focus.clone(),
            value: new_value.clone(),
        });

        self.push_parent_node(new_value.clone(), new_parent.clone());
        self.push_change(old_value, new_value.clone());

        if let Some(parent) = focus.get_parent() {
            self.pending_inode_update(parent.clone(), old_parent.clone(), new_parent.clone());
        } else {
            // the root node
            panic!("")
        }
    }

    fn push_change(&self, old_node: Arc<NodeValue>, new_node: Arc<NodeValue>) {
        let mut changed = self.logger.changed.write().unwrap();
        changed.insert(new_node, old_node);
    }

    fn push_parent_node(&self, node: Arc<NodeValue>, parent: Arc<NodeValue>) {
        let mut parents = self.logger.parents.write().unwrap();
        parents.insert(node, parent);
    }

    fn get_parent_node(&self, node: &Arc<NodeValue>) -> Option<Arc<NodeValue>> {
        let node_parents = self.logger.parents.read().unwrap();

        let parent_node = node_parents.get(node)?;
        Some(parent_node.clone())
    }

    pub(crate) fn pending_inode_update(
        &self,
        focus: Arc<Focus>,
        old_node: Arc<NodeValue>,
        new_node: Arc<NodeValue>,
    ) {
        // println!("111 {:?} {:?} => {:?}", focus.access_path(), old_node, new_node);

        // 取得旧节点的父节点，向上更新
        if let Some(old_parent) = self.get_parent_node(&old_node) {
            // println!("222 {:?} {:?}", old_parent, focus);
            let (new_parent, _old_item) = match (old_parent.as_ref(), focus.get_access_key()) {
                (NodeValue::Map(map_value), AccessKey::Key(ref key)) => {
                    let (new_map, old_item) = map_value.set_item(key.to_string(), new_node.clone());
                    let new_parent_node = Arc::new(NodeValue::Map(new_map));
                    (new_parent_node, old_item)
                }
                (NodeValue::List(list_value), AccessKey::Index(index)) => {
                    let (new_list, old_item) = list_value.set_item(index, new_node.clone());
                    let new_parent_node = Arc::new(NodeValue::List(new_list));
                    (new_parent_node, old_item)
                }
                (_, _) => {
                    panic!(
                        "mismatch map/list with access_key while accessing internal node at '{}'",
                        focus.access_path()
                    );
                }
            };

            self.log_inode_updated(
                focus.clone(),
                old_parent.clone(),
                old_node,
                new_node,
                new_parent,
            );
        } else {

            self.set_root(new_node);
        }
    }
}
