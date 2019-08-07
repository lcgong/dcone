use std::sync::Arc;

use super::spot::Spot;
use crate::error::Error;
use crate::focus::{AccessKey, FocusLocator};
use crate::node::{ListValue, MapValue, NodeValue};

use crate::domain::Cone;
use crate::focus::Focus;

impl Spot {
    pub fn set_empty_map(self) -> Result<Spot, Error> {
        let new_value_node = Arc::new(NodeValue::Map(MapValue::new()));

        self.set_value_node(new_value_node)
    }

    pub fn set_empty_list(self) -> Result<Spot, Error> {
        let new_value_node = Arc::new(NodeValue::List(ListValue::new()));
        self.set_value_node(new_value_node)
    }

    pub fn set_value<V: Into<NodeValue>>(self, value: V) -> Result<Spot, Error> {
        let new_value_node = Arc::new(value.into());
        self.set_value_node(new_value_node)
    }

    fn set_value_node(self, new_value: Arc<NodeValue>) -> Result<Spot, Error> {

        let focus = &self.focus;
        let old_parent = &self.parent;

        if let Some(old_parent) = old_parent {
            let new_parent = set_item_node(
                &self.cone, 
                old_parent, 
                &focus, 
                new_value.clone(),
            )?;

            Ok(Spot {
                cone: self.cone,
                focus: self.focus,
                parent: Some(new_parent),
                node: new_value,
            })
        } else { // the root node without parent
            
            self.cone.remount_root(new_value.clone());
            self.cone.log_root_updated(
                self.focus.clone(), 
                self.node, 
                new_value.clone()
            );

            Ok(Spot {
                cone: self.cone,
                focus: self.focus,
                parent: None,
                node: new_value,
            })
        }
    }
}

impl Spot {
    pub fn set_map_item<T: Into<AccessKey>>(self, access_key: T) -> Result<Spot, Error> {
        let new_item_node = Arc::new(NodeValue::Map(MapValue::new()));
        self.set_item_node(access_key.into(), new_item_node)
    }

    pub fn set_list_item<T: Into<AccessKey>>(self, access_key: T) -> Result<Spot, Error> {
        let new_item_node = Arc::new(NodeValue::List(ListValue::new()));
        self.set_item_node(access_key.into(), new_item_node)
    }

    pub fn set_item<K: Into<AccessKey>, V: Into<NodeValue>>(
        self,
        access_key: K,
        item_node: V,
    ) -> Result<Spot, Error> {
        let new_item_node = Arc::new(item_node.into());
        self.set_item_node(access_key.into(), new_item_node)
    }

    fn set_item_node(
        self,
        access_key: AccessKey,
        new_item_node: Arc<NodeValue>,
    ) -> Result<Spot, Error> {

        let item_focus = self.focus.focus(access_key.clone());


        let new_parent = set_item_node(&self.cone, &self.node, &item_focus, new_item_node)?;

        Ok(Spot {
            cone: self.cone,
            focus: self.focus,
            node: new_parent,
            parent: self.parent,
        })
    }
}

fn set_item_node(
    domain: &Cone,
    parent: &Arc<NodeValue>,
    item_focus: &Arc<Focus>, 
    new_item: Arc<NodeValue>
) -> Result<Arc<NodeValue>, Error> {
    
    let (new_parent, old_item) = match (parent.as_ref(), item_focus.get_access_key()) {
        (NodeValue::Map(map_value), AccessKey::Key(ref key)) => {
            let (new_map, old_item) = map_value.set_item(key.to_string(), new_item.clone());
            
            let new_parent_node = Arc::new(NodeValue::Map(new_map));
            Ok((new_parent_node, old_item))
        }
        (NodeValue::List(list_value), AccessKey::Index(index)) => {
            let (new_list, old_item) = list_value.set_item(index, new_item.clone());
            let new_parent_node = Arc::new(NodeValue::List(new_list));
            Ok((new_parent_node, old_item))
        }
        (_, access_key) => {
            let parent_focus = item_focus.get_parent().unwrap();
            Error::mismatched_access_key(parent_focus, &access_key)
        }
    }?;

    if let Some(old_item) = old_item {
        domain.log_value_updated(
            item_focus.clone(), 
            parent.clone(), 
            old_item, 
            new_item, 
            new_parent.clone()
        );
    } else {
        domain
            .log_value_created(item_focus, parent, &new_item, &new_parent);
    }

    Ok(new_parent)
}

impl Spot {
    pub fn remove<K: Into<AccessKey>>(self, access_key: K) -> Result<Spot, Error> {
        let collection_focus = self.focus;
        let collection_node = &self.node;

        let access_key = access_key.into();

        let item_focus = collection_focus.focus(access_key.clone());

        let (new_collection, old_value) = match (collection_node.as_ref(), &access_key) {
            (NodeValue::Map(map_value), AccessKey::Key(ref key)) => {
                if let Some(old_value) = map_value.get_item(key) {
                    let new_collection = Arc::new(NodeValue::Map(map_value.remove(key)));
                    Ok((new_collection, old_value))
                } else {
                    Error::no_such_item(&collection_focus, &access_key)
                }
            }
            (NodeValue::List(list_value), AccessKey::Index(index)) => {
                if let Some(old_value) = list_value.get_item(*index) {
                    let new_collection = Arc::new(NodeValue::List(list_value.remove(*index)));
                    Ok((new_collection, old_value))
                } else {
                    Error::no_such_item(&collection_focus, &access_key)
                }
            }
            (_, access_key) => Error::mismatched_access_key(&collection_focus, &access_key),
        }?;

        self.cone.log_value_deleted(
            &item_focus, 
            collection_node, 
            old_value, 
            &new_collection
        );

        Ok(Spot {
            cone: self.cone,
            focus: collection_focus,
            node: new_collection,
            parent: self.parent,
        })
    }
}

impl Spot {
    pub fn len(&self) -> Result<isize, Error> {
        let parent_focus = &self.focus;
        let parent_node = &self.node;

        match parent_node.as_ref() {
            NodeValue::Map(map_value) => Ok(map_value.len() as isize),
            NodeValue::List(list_value) => Ok(list_value.len() as isize),
            _ => Error::should_be_collection(parent_focus),
        }
    }
}
