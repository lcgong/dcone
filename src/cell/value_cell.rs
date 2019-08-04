// use crate::domain::Domain;
// use crate::focus::{AccessKey, Focus, FocusLocator};
use crate::node::NodeValue;
// use crate::node::{NodeValue, MapValue, ListValue};
// use crate::log::ChangeLogger;
use super::cell::ValueCell;

// use std::sync::Arc;


// pub fn update_value(logger: &ChangeLogger, root: &Arc<NodeValue>, 
//     focus: Arc<Focus>, new_value: Arc<NodeValue>) -> Arc<NodeValue> {
    
//     let mut ancestors: Vec<(&Arc<Focus>, &Arc<NodeValue>)> = Vec::new(); 
//     {
//         let mut focuses_of_ancestors = focus.ancestors().collect::<Vec<&Arc<Focus>>>();
//         focuses_of_ancestors.pop(); // the last one is the None focus

//         let mut current = root.as_ref();

//         for f in focuses_of_ancestors.iter().rev() {
//             match current {
//                 NodeValue::Map(map_node) => {
//                     if let AccessKey::Key(ref key) = f.get_access_key() {
//                         if let Some(item_node) = map_node.map.get(key) {
//                             ancestors.push((*f, item_node));
//                             current = item_node;
//                             continue;
//                         }
//                     }
//                     panic!("mismatched access_key with value node");
//                 },
//                 NodeValue::List(list_node) => {
//                     if let AccessKey::Index(index) = f.get_access_key() {
                        
//                         // TODO isize as usize 
//                         let uindex = if index >= 0 {
//                             index
//                         } else { // negative index, -1 means the last one
//                             list_node.list.len() as isize + index    
//                         } as usize;

//                         if let Some(item_node) = list_node.list.get(uindex) {
//                             ancestors.push((*f, item_node));
//                             current = item_node;
//                             continue;
//                         }
//                     }
//                     panic!("mismatched access_key with value node");
//                 },
//                 _ => {
//                     panic!("mismatched access_key with value node");
//                 }
//             }
//         }
//     }

//     let mut new_node = new_value;
//     for (focus, parent_node) in ancestors.iter().rev() { // from leaf to root
//         new_node = set_node_item(&logger, &focus, &parent_node, new_node);
//     }

//     new_node
// }

// fn set_node_item(logger: &ChangeLogger, focus: &Arc<Focus>,
//     parent_node: &Arc<NodeValue>, item_node: Arc<NodeValue>) -> Arc<NodeValue> {
    
//     match parent_node.as_ref() { 
//         NodeValue::Map(ref map_value) => {
//             set_map_item(logger,  focus, map_value, item_node)
//         },
//         NodeValue::List(ref list_value) => {
//             set_list_item(logger, focus, list_value, item_node)
//         },
//         _ => {
//             panic!("This node should be a NodeValue::List or NodeValue::Map");
//         }
//     }
// }

// fn set_map_item(logger: &ChangeLogger,  focus: &Arc<Focus>,
//     map_value: &MapValue, item_node: Arc<NodeValue>) -> Arc<NodeValue> {

//     let access_key = focus.get_access_key();
    
//     if let AccessKey::Key(key) = &access_key {

//         if let Some(old_item_node) = map_value.map.get(key) { // key已经有值
            
//             let mut new_hash_map = map_value.map.clone();
//             new_hash_map.insert(key.to_string(), item_node.clone());

//             let new_map_node = Arc::new(NodeValue::Map(MapValue {
//                 map: new_hash_map.clone()
//             }));


//             logger.log_value_changed(
//                 access_key, item_node.clone(), old_item_node.clone());

//             return new_map_node;

//         } else { // key为新值
//             let mut new_hash_map = map_value.map.clone();
//             new_hash_map.insert(key.to_string(), item_node.clone());

//             let new_map_node = Arc::new(NodeValue::Map(MapValue {
//                 map: new_hash_map.clone()
//             }));


//             logger.log_value_created(access_key, item_node);

//             return new_map_node;
//         }
//     }
//     panic!("This node should be a NodeValue::Map");
// }

// fn set_list_item(logger: &ChangeLogger, focus: &Arc<Focus>,
//     list_value: &ListValue, item_node: Arc<NodeValue>) -> Arc<NodeValue> {

//     let access_key = focus.get_access_key();
    
//     if let AccessKey::Index(index) = access_key {

//         let mut new_list = list_value.clone();
//         let old_item = new_list.list.set(index as usize, item_node.clone());

//         let new_list_node = Arc::new(NodeValue::List(new_list.clone()));

//         logger.log_value_changed(access_key, item_node, old_item);

//         return new_list_node;
//     }

//     panic!("This node should be a NodeValue::List");
// }



impl ValueCell {

    pub fn is_none(&self) -> bool {
        match self.node.as_ref() {
            NodeValue::None => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self.node.as_ref() {
            NodeValue::Integer(_) => true,
            _ => false,
        }
    }

    pub fn to_bool(&self) -> bool {
        if let NodeValue::Bool(value) = self.node.as_ref() {
            return *value;
        }

        panic!("The value is not a NodeValue::Bool");
    }

    pub fn to_i64(&self) -> i64 {
        if let NodeValue::Integer(value) = self.node.as_ref() {
            return *value;
        }

        panic!("The value is not a NodeValue::Integer");
    }

    pub fn to_f64(&self) -> f64 {
        if let NodeValue::Float(value) = self.node.as_ref() {
            return *value;
        }

        panic!("The value is not a NodeValue::Float");
    }

    pub fn to_string(&self) -> String {
        if let NodeValue::String(value) = self.node.as_ref() {
            return value.to_string();
        }

        panic!("The value is not a NodeValue::String");
    }
}

impl ::std::fmt::Debug for ValueCell {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.node.fmt(fmt)
    }
}

