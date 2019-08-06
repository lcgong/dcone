use super::domain::Domain;
use crate::focus::{Focus, FocusLocator};
use std::sync::Arc;
use crate::node::NodeValue;
use crate::cell::upward_update::upward_update_nodes;

impl Domain {
    pub(crate) fn log_node_deleted(
        &self,
        focus: &Arc<Focus>,
        old_parent: &Arc<NodeValue>,
        old_value: &Arc<NodeValue>,
        new_parent: &Arc<NodeValue>,
    ) {
        // let txid = self.new_txid();

        // self.push(NodeEvent::Deleted {
        //     txid: txid,
        //     focus: focus.clone(),
        //     value: value.clone(),
        // });

        self.logger.node_deleted(focus, old_parent, old_value);

        if let Some(parent_focus) = focus.get_parent() {
            upward_update_nodes(&self, &parent_focus, new_parent.clone());
        }
    }

    pub(crate) fn log_node_created(
        &self,
        focus: &Arc<Focus>,
        old_parent: &Arc<NodeValue>,
        new_value: &Arc<NodeValue>,
        new_parent: &Arc<NodeValue>,
    ) {

        self.logger.node_created(focus, old_parent, new_value);

        if let Some(parent_focus) = focus.get_parent() {
            upward_update_nodes(&self, &parent_focus, new_parent.clone());
        }

    }

}