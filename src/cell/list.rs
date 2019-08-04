use std::sync::Arc;

use crate::focus::{AccessKey, FocusLocator};
use crate::node::{NodeValue, MapValue, ListValue};
use super::cell::{ValueCell};


use super::upward_update::upward_update_nodes;


impl ValueCell {

    pub fn push_map_item(&self) -> Self {

        let new_item_node = Arc::new(NodeValue::Map(MapValue::new()));
        self._push_item(new_item_node)
    }

    pub fn push_list_item(&self) -> Self {

        let new_item_node = Arc::new(NodeValue::List(ListValue::new()));
        self._push_item(new_item_node)
    }

    pub fn push_item<V: Into<NodeValue>>(&self, item_node: V) -> Self {

        let new_item_node = Arc::new(item_node.into());
        self._push_item(new_item_node)
    }

    #[inline]
    fn _push_item(&self, new_item_node: Arc<NodeValue>) -> Self {
        
        let domain = &self.domain;
        let parent_focus = &self.focus;
        let parent_node = &self.node;            
        
        let logger = &domain.logger;

        match parent_node.as_ref() {
            NodeValue::List(list_value) => {
                let list_value = list_value.push(new_item_node.clone());
                let item_focus = parent_focus.focus(list_value.len() - 1);

                let new_parent_node = Arc::new(NodeValue::List(list_value));

                logger.node_created(&item_focus, parent_node, &new_item_node);

                upward_update_nodes(domain, parent_focus, new_parent_node.clone());
                
                ValueCell {
                    domain: domain.clone(),
                    focus: parent_focus.clone(),
                    node: new_parent_node,
                }            
            },
            _ => {
                panic!("The parent node should be a List")        
            }
        }
    }


    pub fn insert_item<K: Into<AccessKey>, V: Into<NodeValue>>(&self, 
        access_key: K, item_node: V) -> Self {
        
        
        let domain = &self.domain;
        let parent_focus = &self.focus;
        let parent_node = &self.node;            
        
        let access_key = access_key.into();
        let item_focus = parent_focus.focus(access_key.clone());
        let logger = &domain.logger;

        let new_item_node = Arc::new(item_node.into());

        match (parent_node.as_ref(), access_key) {
            (NodeValue::List(list_value), AccessKey::Index(index)) => {

                let new_parent_node = Arc::new(NodeValue::List(
                        list_value.insert(index, new_item_node.clone())
                    ));

                logger.node_created(&item_focus, parent_node, &new_item_node);

                upward_update_nodes(domain, parent_focus, new_parent_node.clone());
                
                ValueCell {
                    domain: domain.clone(),
                    focus: parent_focus.clone(),
                    node: new_parent_node,
                }
            },
            _ => {
                panic!("The parent node should be a List")        
            }
        }
    }
}

