use std::sync::Arc;

use crate::focus::{AccessKey, FocusLocator};
use crate::node::{NodeValue, MapValue, ListValue};
use super::cell::{ValueCell};
use crate::error::Error;


impl ValueCell {

    pub fn push_map_item(self) -> Result<Self, Error> {

        let new_item_node = Arc::new(NodeValue::Map(MapValue::new()));
        self._push_item(new_item_node)
    }

    pub fn push_list_item(self) -> Result<Self, Error> {

        let new_item_node = Arc::new(NodeValue::List(ListValue::new()));
        self._push_item(new_item_node)
    }

    pub fn push_item<V: Into<NodeValue>>(self, item_node: V) -> Result<Self, Error> {

        let new_item_node = Arc::new(item_node.into());
        self._push_item(new_item_node)
    }

    #[inline]
    fn _push_item(self, new_item_node: Arc<NodeValue>) -> Result<Self, Error> {
        
        let domain = &self.domain;
        let parent_focus = &self.focus;
        let parent_node = &self.node;            
        
        let (new_index, new_parent_node) = match parent_node.as_ref() {
            NodeValue::List(list_value) => {
                let index = list_value.len();
                let list_value = list_value.push(new_item_node.clone());

                let new_parent_node = Arc::new(NodeValue::List(list_value));

                Ok((index, new_parent_node))
            },
            _ => Error::should_be_list(parent_focus)
        }?;

        let item_focus = parent_focus.focus(new_index);

        self.domain.log_listitem_inserted(
            &item_focus, 
            parent_node, 
            &new_item_node, 
            &new_parent_node
        );

        Ok(ValueCell {
            domain: domain.clone(),
            focus: parent_focus.clone(),
            node: new_parent_node,
            parent: self.parent,
        })          
    }


    pub fn insert_item<K: Into<AccessKey>, V: Into<NodeValue>>(
        self, 
        access_key: K, 
        item_node: V
    ) -> Result<Self, Error> {
        
        
        let domain = &self.domain;
        let parent_focus = &self.focus;
        let parent_node = &self.node;            
        
        let access_key = access_key.into();
        let item_focus = parent_focus.focus(access_key.clone());

        let new_item_node = Arc::new(item_node.into());

        let new_parent_node = match (parent_node.as_ref(), access_key) {
            (NodeValue::List(list_value), AccessKey::Index(index)) => {

                let new_list = list_value.insert(index, new_item_node.clone());
                let new_parent_node = Arc::new(NodeValue::List(new_list));
                Ok(new_parent_node)
            },
            _ => Error::should_be_list(parent_focus)
        }?;

        self.domain.log_listitem_inserted(
            &item_focus, 
            parent_node,
            &new_item_node, 
            &new_parent_node
        );

        Ok(ValueCell {
            domain: domain.clone(),
            focus: parent_focus.clone(),
            node: new_parent_node,
            parent: self.parent,
        })

    }
}

