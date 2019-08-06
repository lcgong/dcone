use im::Vector;
use std::sync::Arc;
use super::value::NodeValue;

use crate::focus::{CircularZeroIndex};

pub struct ListValue {
    pub(crate) list: Vector<Arc<NodeValue>>,
}


impl ListValue {
    pub fn new() -> ListValue {
        ListValue {
            list: Vector::new()
        }
    }

    #[inline]
    pub fn len(&self) -> isize {
        self.list.len() as isize
    }

    pub fn get_item(&self, index: CircularZeroIndex) -> Option<&Arc<NodeValue>> {
        let index = if index >= 0 {
            index
        } else { // negative index, -1 means the last one
            self.list.len() as isize + index    
        } as usize;

        return self.list.get(index)                        
    }

    #[inline]
    pub fn set_item(&self, index: CircularZeroIndex, value: Arc<NodeValue>) -> (Self, Option<Arc<NodeValue>>) {
        let index = if index >= 0 {
            index
        } else { // negative index, -1 means the last one
            self.list.len() as isize + index    
        } as usize;                        

        let mut new_vector = self.list.clone();
        let old_item = new_vector.set(index, value);

        (ListValue {
            list: new_vector
        }, Some(old_item))
    }

    #[inline]
    pub fn push(&self, value: Arc<NodeValue>) -> Self {

        let mut new_vector = self.list.clone();
        new_vector.push_back(value);

        ListValue {
            list: new_vector
        }
    }


    #[inline]
    pub fn insert(&self, index: CircularZeroIndex, value: Arc<NodeValue>) -> Self {
        let index = if index >= 0 {
            index
        } else { // negative index, -1 means the last one
            self.list.len() as isize + index    
        } as usize;                        

        let mut new_vector = self.list.clone();
        new_vector.insert(index, value);

        ListValue {
            list: new_vector
        }
    }

    pub fn remove(&self, index: CircularZeroIndex) -> Self {

        let index = if index >= 0 {
            index
        } else { // negative index, -1 means the last one
            self.list.len() as isize + index    
        } as usize;                        

        let mut new_vector = self.list.clone();
        new_vector.remove(index);

        ListValue {
            list: new_vector
        }
    }    
}

// impl ListCell {

    // pub fn set_value(&self, index: usize, item: ValueCell) -> Self {
    //     if let NodeValue::List(old_list) = self.node.as_ref() {
    //         let logger = &self.domain.logger;

    //         let new_item_node = item.node;

    //         let mut new_list = old_list.clone();
    //         let old_item = new_list.list.set(index, new_item_node.clone());

    //         let new_node = Arc::new(NodeValue::List(new_list.clone()));

    //         logger.log_value_changed(
    //             AccessKey::Index(index as CircularZeroIndex), 
    //             new_item_node.clone(), old_item);

    //         return ListCell {
    //             domain: self.domain.clone(),
    //             focus: Focus::new(),
    //             node: new_node,
    //         };
    //     }

    //     panic!("This node should be a NodeValue::List");
    // }

//     pub fn head(&self) -> Option<ValueCell> {
//         if let NodeValue::List(list_value) = self.node.as_ref() {
//             return match list_value.list.head() {
//                 Some(item) => Some(ValueCell {
//                     domain: self.domain.clone(),
//                     focus: Focus::new(),
//                     node: item.clone(),
//                 }),
//                 None => None,
//             };
//         }
//         panic!("This node should be a NodeValue::List");
//     }

//     pub fn tail(&self) -> Option<ValueCell> {
//         if let NodeValue::List(list_value) = self.node.as_ref() {
//             return match list_value.list.back() {
//                 Some(item) => Some(ValueCell {
//                     domain: self.domain.clone(),
//                     focus: Focus::new(),
//                     node: item.clone(),
//                 }),
//                 None => None,
//             };
//         }
//         panic!("This node should be a NodeValue::List");
//     }

//     pub fn push(&self, item: ValueCell) -> Self {
//         if let NodeValue::List(old_list) = self.node.as_ref() {
//             let logger = &self.domain.logger;
//             let new_item_node = item.node;

//             let mut new_list = old_list.clone();
//             new_list.list.push_back(new_item_node.clone());

//             let new_node = Arc::new(NodeValue::List(new_list.clone()));

//             logger.log_value_created(
//                 AccessKey::Index(new_list.list.len() as CircularZeroIndex), 
//                 new_item_node);

//             return ListCell {
//                 domain: self.domain.clone(),
//                 focus: Focus::new(),
//                 node: new_node,
//             };
//         }

//         panic!("This node should be a NodeValue::List");
//     }

//     pub fn push_head(&mut self, item: ValueCell) -> Self {
//         if let NodeValue::List(old_list) = self.node.as_ref() {
//             let logger = &self.domain.logger;
//             let new_item_node = item.node;

//             let mut new_list = old_list.clone();
//             new_list.list.push_front(new_item_node.clone());

//             let new_node = Arc::new(NodeValue::List(new_list.clone()));

//             logger.log_value_created(AccessKey::Index(0), new_item_node);

//             return ListCell {
//                 domain: self.domain.clone(),
//                 focus: Focus::new(),
//                 node: new_node,
//             };
//         }

//         panic!("This node should be a NodeValue::List");
//     }
// }





//     pub fn pop_head(&mut self) -> Option<Arc<NodeValue>> {
//         if let Some(node_ref) = self.list.pop_front() {
//             Some(node_ref.clone())
//         } else {
//             None
//         }
//     }

//     pub fn pop(&mut self) -> Option<Arc<NodeValue>> {
//         if let Some(node_ref) = self.list.pop_back() {
//             Some(node_ref.clone())
//         } else {
//             None
//         }
//     }
// }

impl Clone for ListValue {
    fn clone(&self) -> Self {
        ListValue {
            list: self.list.clone(),
        }
    }
}

impl core::cmp::PartialEq for ListValue {
    fn eq(&self, other: &Self) -> bool {
        self.list == other.list
    }
}

// pub struct ListValueIter<'a> {
//     iter: im::vector::Iter<'a, Arc<NodeValue>>,
// }

// impl<'a> Iterator for ListValueIter<'a> {
//     type Item = Arc<NodeValue>;
//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(item) = self.iter.next() {
//             Some(item.clone())
//         } else {
//             None
//         }
//     }
// }

// impl<'a> std::iter::IntoIterator for &'a ListValue {
//     type Item = Arc<NodeValue>;
//     type IntoIter = ListValueIter<'a>;

//     #[inline]
//     fn into_iter(self) -> Self::IntoIter {
//         ListValueIter {
//             iter: self.list.iter(),
//         }
//     }
// }

impl ::std::fmt::Debug for ListValue {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut first = true;

        fmt.write_str("[")?;

        for v in self.list.iter() {
            if !first {
                fmt.write_str(", ")?;
            }

            v.fmt(fmt)?;
            first = false;
        }

        fmt.write_str("]")
    }
}
