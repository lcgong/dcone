use im::Vector;

use std::sync::Arc;

use super::node::{NodeValue, NodeValueRef, ListValue};


use crate::focus::{AccessKey, Focus, CircularZeroIndex};
use super::cell::{ValueCell, ListCell};


impl ListValue {
    pub fn new() -> ListValue {
        ListValue {
            list: Vector::new()
        }
    }
}

impl ListCell {
    pub fn as_value(&self) -> ValueCell {
        ValueCell {
            domain: self.domain.clone(),
            focus: Focus::new(),
            node: self.node.clone(),
        }
    }

    pub fn len(&self) -> usize {
        if let NodeValue::List(list_value) = self.node.as_ref() {
            return list_value.list.len();
        }

        panic!("This node should be a NodeValue::List");
    }

    pub fn get_item(&self, index: usize) -> Option<ValueCell> {
        if let NodeValue::List(list_value) = self.node.as_ref() {
                return match list_value.list.get(index) {
                    Some(item) => Some(ValueCell {
                        domain: self.domain.clone(),
                        focus: Focus::new(),
                        node: item.clone()
                    }), 
                    _ => None
                }
        }

        panic!("This node should be a NodeValue::List");
    }

    pub fn set_value(&self, index: usize, item: ValueCell) -> Self {
        if let NodeValue::List(old_list) = self.node.as_ref() {
            let logger = &self.domain.logger;

            let new_item_node = item.node;

            let mut new_list = old_list.clone();
            let old_item = new_list.list.set(index, new_item_node.clone());

            let new_node = Arc::new(NodeValue::List(new_list.clone()));

            logger.log_value_changed(
                AccessKey::Index(index as CircularZeroIndex), 
                new_item_node.clone(), old_item);

            return ListCell {
                domain: self.domain.clone(),
                focus: Focus::new(),
                node: new_node,
            };
        }

        panic!("This node should be a NodeValue::List");
    }

    pub fn head(&self) -> Option<ValueCell> {
        if let NodeValue::List(list_value) = self.node.as_ref() {
            return match list_value.list.head() {
                Some(item) => Some(ValueCell {
                    domain: self.domain.clone(),
                    focus: Focus::new(),
                    node: item.clone(),
                }),
                None => None,
            };
        }
        panic!("This node should be a NodeValue::List");
    }

    pub fn tail(&self) -> Option<ValueCell> {
        if let NodeValue::List(list_value) = self.node.as_ref() {
            return match list_value.list.back() {
                Some(item) => Some(ValueCell {
                    domain: self.domain.clone(),
                    focus: Focus::new(),
                    node: item.clone(),
                }),
                None => None,
            };
        }
        panic!("This node should be a NodeValue::List");
    }

    pub fn push(&self, item: ValueCell) -> Self {
        if let NodeValue::List(old_list) = self.node.as_ref() {
            let logger = &self.domain.logger;
            let new_item_node = item.node;

            let mut new_list = old_list.clone();
            new_list.list.push_back(new_item_node.clone());

            let new_node = Arc::new(NodeValue::List(new_list.clone()));

            logger.log_value_created(
                AccessKey::Index(new_list.list.len() as CircularZeroIndex), 
                new_item_node);

            return ListCell {
                domain: self.domain.clone(),
                focus: Focus::new(),
                node: new_node,
            };
        }

        panic!("This node should be a NodeValue::List");
    }

    pub fn push_head(&mut self, item: ValueCell) -> Self {
        if let NodeValue::List(old_list) = self.node.as_ref() {
            let logger = &self.domain.logger;
            let new_item_node = item.node;

            let mut new_list = old_list.clone();
            new_list.list.push_front(new_item_node.clone());

            let new_node = Arc::new(NodeValue::List(new_list.clone()));

            logger.log_value_created(AccessKey::Index(0), new_item_node);

            return ListCell {
                domain: self.domain.clone(),
                focus: Focus::new(),
                node: new_node,
            };
        }

        panic!("This node should be a NodeValue::List");
    }
}



//     pub fn remove(&mut self, index: usize) -> NodeValueRef {
//         let old_item = self.list.remove(index);
//         old_item.clone()
//     }

//     pub fn pop_head(&mut self) -> Option<NodeValueRef> {
//         if let Some(node_ref) = self.list.pop_front() {
//             Some(node_ref.clone())
//         } else {
//             None
//         }
//     }

//     pub fn pop(&mut self) -> Option<NodeValueRef> {
//         if let Some(node_ref) = self.list.pop_back() {
//             Some(node_ref.clone())
//         } else {
//             None
//         }
//     }
// }

impl ::std::fmt::Debug for ListCell {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.node.fmt(fmt)
    }
}

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

pub struct ListValueIter<'a> {
    iter: im::vector::Iter<'a, NodeValueRef>,
}

impl<'a> Iterator for ListValueIter<'a> {
    type Item = NodeValueRef;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next() {
            Some(item.clone())
        } else {
            None
        }
    }
}

impl<'a> std::iter::IntoIterator for &'a ListValue {
    type Item = NodeValueRef;
    type IntoIter = ListValueIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ListValueIter {
            iter: self.list.iter(),
        }
    }
}

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

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn list_value() {
//         use crate::list::{ListValue, NodeValue};
//         use std::sync::Arc;
//         let mut list1 = ListValue::new();
//         list1.push(Arc::new(NodeValue::from(10)));
//         list1.push(Arc::new(NodeValue::from("AA")));
//         list1.push(Arc::new(NodeValue::from(20)));

//         assert_eq!(list1.get_item(1).unwrap().as_ref(), &NodeValue::from("AA"));
//         assert_eq!(list1.get_item(2).unwrap().as_ref(), &NodeValue::from(20));

//         list1.push_head(Arc::new(NodeValue::from(100)));
//         assert_eq!(list1.len(), 4);
//         assert_eq!(list1.pop_head().unwrap().as_ref(), &NodeValue::from(100));
//         assert_eq!(list1.len(), 3);

//         list1.push(Arc::new(NodeValue::from(100)));
//         list1.push(Arc::new(NodeValue::from(200)));
//         assert_eq!(list1.pop().unwrap().as_ref(), &NodeValue::from(200));
//         assert_eq!(
//             list1.remove(list1.len() - 1).as_ref(),
//             &NodeValue::from(100)
//         );
//         assert_eq!(list1.len(), 3);

//         assert_eq!(list1.get_item(0).unwrap().as_ref(), &NodeValue::from(10));

//         let mut list2 = list1.clone();
//         let old_item = list2.set_value(1, Arc::new(NodeValue::from("BB")));

//         assert_eq!(list1.get_item(1).unwrap().as_ref(), &NodeValue::from("AA"));
//         assert_eq!(list2.get_item(1).unwrap().as_ref(), &NodeValue::from("BB"));

//         assert_eq!(list1.get_item(1), Some(old_item));
//         assert_ne!(list1.get_item(1), list2.get_item(1));

//         println!("List1: {:?}", list1);
//         println!("List2: {:?}", list2);

//         // for item in &list1 {
//         //     println!("e1 {:?}", item);
//         // }
//         // for item in &list2 {
//         //     println!("e2 {:?}", item);
//         // }
//     }
// }
