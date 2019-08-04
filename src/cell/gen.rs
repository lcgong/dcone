use std::sync::Arc;

use super::cell::ValueCell;
use crate::error::Error;
use crate::focus::{AccessKey, FocusLocator};
use crate::node::{ListValue, MapValue, NodeValue};

use super::upward_update::upward_update_nodes;

impl ValueCell {
    pub fn set_empty_map(&self) -> Result<ValueCell, Error> {

        let new_value_node = Arc::new(NodeValue::Map(MapValue::new()));

        self.set_value_node(new_value_node)
    }

    pub fn set_empty_list(&self) -> Result<ValueCell, Error> {
        let new_value_node = Arc::new(NodeValue::List(ListValue::new()));
        self.set_value_node(new_value_node)
    }

    pub fn set_value<V: Into<NodeValue>>(&self, value: V) -> Result<ValueCell, Error> {

        let new_value_node = Arc::new(value.into());
        self.set_value_node(new_value_node)
    }


    fn set_value_node(&self, new_value_node: Arc<NodeValue>) -> Result<ValueCell, Error> {
        let domain = &self.domain;
        let focus = &self.focus;

        upward_update_nodes(domain, focus, new_value_node.clone());

        Ok(ValueCell {
            domain: domain.clone(),
            focus: focus.clone(),
            node: new_value_node,
        })
    }
}

impl ValueCell {
    pub fn set_map_item<T: Into<AccessKey>>(&self, access_key: T) -> Result<ValueCell, Error> {
        let new_item_node = Arc::new(NodeValue::Map(MapValue::new()));
        self.set_item_node(access_key.into(), new_item_node)
    }

    pub fn set_list_item<T: Into<AccessKey>>(&self, access_key: T) -> Result<ValueCell, Error> {
        let new_item_node = Arc::new(NodeValue::List(ListValue::new()));
        self.set_item_node(access_key.into(), new_item_node)
    }

    pub fn set_item<K: Into<AccessKey>, V: Into<NodeValue>>(
        &self,
        access_key: K,
        item_node: V,
    ) -> Result<ValueCell, Error> {
        let new_item_node = Arc::new(item_node.into());
        self.set_item_node(access_key.into(), new_item_node)
    }

    fn set_item_node(
        &self,
        access_key: AccessKey,
        new_item_node: Arc<NodeValue>,
    ) -> Result<ValueCell, Error> {
        let domain = &self.domain;
        let parent_focus = &self.focus;
        let parent_node = &self.node;

        let root_node = &domain.get_root();
        let item_focus = parent_focus.focus(access_key.clone());
        let logger = &domain.logger;

        match (parent_node.as_ref(), access_key) {
            (NodeValue::Map(map_value), AccessKey::Key(ref key)) => {
                let new_parent_node = Arc::new(NodeValue::Map(
                    map_value.set_item(key.to_string(), new_item_node.clone()),
                ));

                logger.value_created(root_node, &item_focus, &new_item_node);

                upward_update_nodes(domain, parent_focus, new_parent_node.clone());
                Ok(ValueCell {
                    domain: domain.clone(),
                    focus: parent_focus.clone(),
                    node: new_parent_node,
                })
            }
            (NodeValue::List(list_value), AccessKey::Index(index)) => {
                let new_parent_node = Arc::new(NodeValue::List(
                    list_value.set_item(index, new_item_node.clone()),
                ));

                logger.value_created(root_node, &item_focus, &new_item_node);

                upward_update_nodes(domain, parent_focus, new_parent_node.clone());
                Ok(ValueCell {
                    domain: domain.clone(),
                    focus: parent_focus.clone(),
                    node: new_parent_node,
                })
            }
            (_, access_key) => Error::mismatched_access_key(parent_focus, &access_key),
        }
    }
}

impl ValueCell {

    pub fn remove<K: Into<AccessKey>>(
        &self,
        access_key: K,
    ) -> Result<ValueCell, Error> {

        let domain = &self.domain;
        let parent_focus = &self.focus;
        let parent_node = &self.node;

        let access_key = access_key.into();
        let root_node = &domain.get_root();
        let logger = &domain.logger;

        let item_focus = parent_focus.focus(access_key.clone());

        match (parent_node.as_ref(), &access_key) {
            (NodeValue::Map(map_value), AccessKey::Key(ref key)) => {

                if let Some(original) = map_value.get_item(key) {
                    let new_parent_node = Arc::new(NodeValue::Map(
                        map_value.remove(key)
                    ));

                    logger.value_removed(root_node, &item_focus, original);

                    upward_update_nodes(domain, parent_focus, new_parent_node.clone());

                    Ok(ValueCell {
                        domain: domain.clone(),
                        focus: parent_focus.clone(),
                        node: new_parent_node,
                    })
                } else {
                    Error::no_such_item(parent_focus, &access_key)
                }
                
            }
            (NodeValue::List(list_value), AccessKey::Index(index)) => {

                if let Some(original) = list_value.get_item(*index) {
                    let new_parent_node = Arc::new(NodeValue::List(
                        list_value.remove(*index)
                    ));

                    logger.value_removed(root_node, &item_focus, original);

                    upward_update_nodes(domain, parent_focus, new_parent_node.clone());

                    Ok(ValueCell {
                        domain: domain.clone(),
                        focus: parent_focus.clone(),
                        node: new_parent_node,
                    })
                } else {
                    Error::no_such_item(parent_focus, &access_key)
                }
            }
            (_, access_key) => Error::mismatched_access_key(parent_focus, &access_key),
        }
    }
}


impl ValueCell {

    pub fn len(&self) -> Result<isize, Error> {

        let parent_focus = &self.focus;
        let parent_node = &self.node;

        match parent_node.as_ref() {
            NodeValue::Map(map_value) => {
                Ok(map_value.len() as isize)
            }
            NodeValue::List(list_value) => {
                Ok(list_value.len() as isize)
            }
            _ => Error::should_be_collection(parent_focus)
        }
    }
}

