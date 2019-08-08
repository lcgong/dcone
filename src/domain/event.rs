use super::cone::Cone;
use crate::focus::{AccessKey, Focus, FocusLocator};
use crate::node::NodeValue;
use std::sync::Arc;

use super::log::{NodeEvent, PendingUpdate};

impl Cone {
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

    pub(crate) fn log_internal_root_updated(
        &self,
        focus: Arc<Focus>,
        old_value: Arc<NodeValue>,
        new_value: Arc<NodeValue>,
    ) {
        let logger = &self.logger;

        let txid = logger.new_txid();

        logger.push(NodeEvent::InternalRootUpdated {
            txid: txid,
            focus: focus,
            value: new_value.clone(),
        });

        self.push_change(old_value, new_value);
    }

    pub(crate) fn log_internal_line_updated(
        &self,
        focus: Arc<Focus>,
        old_value: Arc<NodeValue>,
        new_value: Arc<NodeValue>,
    ) {
        let logger = &self.logger;

        let txid = logger.new_txid();

        logger.push(NodeEvent::InternalLineUpdated {
            txid: txid,
            focus: focus,
            old_node: old_value.clone(),
            new_node: new_value.clone(),
        });

        self.push_change(old_value, new_value);
    }

    #[inline]
    pub(super) fn push_pending(
        &self,
        focus: Arc<Focus>,
        old_node: Arc<NodeValue>,
        new_node: Arc<NodeValue>,
    ) {
        let pending_update = PendingUpdate {
                focus: focus.clone(),
                old_node: old_node,
                new_node: new_node,
        };

        let mut pending = self.logger.pending.write().unwrap();
        if let Some(updates) = pending.get_mut(&focus) {
            updates.push(pending_update);
        } else {
            let mut updates = Vec::new();
            updates.push(pending_update);
            pending.insert(focus, updates);
        }
    }

    #[inline]
    pub(super) fn push_change(&self, old_node: Arc<NodeValue>, new_node: Arc<NodeValue>) {
        let mut changed = self.logger.changed.write().unwrap();
        changed.insert(new_node, old_node);
    }

    #[inline]
    pub(super) fn push_parent_node(&self, node: Arc<NodeValue>, parent: Arc<NodeValue>) {
        let mut parents = self.logger.parents.write().unwrap();
        parents.insert(node, parent);
    }

    #[inline]
    pub(super) fn get_parent_node(&self, node: &Arc<NodeValue>) -> Option<Arc<NodeValue>> {
        let node_parents = self.logger.parents.read().unwrap();

        let parent_node = node_parents.get(node)?;
        Some(parent_node.clone())
    }
}
