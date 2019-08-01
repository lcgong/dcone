
use std::sync::Arc;
use im::HashMap;

use super::value::NodeValue;


#[derive(PartialEq)]
pub struct MapValue {
    pub(crate) map: HashMap<String, Arc<NodeValue>>,
}


impl MapValue {
    pub fn new() -> MapValue {
        MapValue {
            map: HashMap::new()
        }
    }

    pub fn set_item(&self, key: String, value: Arc<NodeValue>) -> Self {

        let mut new_hash_map = self.map.clone();
        new_hash_map.insert(key, value);

        MapValue {
            map: new_hash_map
        }
    }

    #[inline]
    pub fn get_item(&self, key: &String) -> Option<&Arc<NodeValue>> {
        return self.map.get(key)
    }
}

// impl MapCell {
//     pub fn to_value(&self) -> ValueCell {
//         ValueCell {
//             domain: self.domain.clone(),
//             focus: Focus::new(),
//             node: self.node.clone(),
//         }
//     }

//     pub fn len(&self) -> usize {
//         if let NodeValue::Map(map) = self.node.as_ref() {
//             return map.map.len();
//         }

//         panic!("This node should be a NodeValue::Map");
//     }

//     pub fn get_item(&self, key: &String) -> Option<ValueCell> {
//         if let NodeValue::Map(map_value) = self.node.as_ref() {
//             return match map_value.map.get(key) {
//                 Some(item) => Some(ValueCell {
//                     domain: self.domain.clone(),
//                     focus: Focus::new(),
//                     node: item.clone(),
//                 }),
//                 _ => None,
//             };
//         }
//         panic!("This node should be a NodeValue::List");
//     }

//     pub fn set_item(&self, key: String, item: ValueCell) -> Self {
//         if let NodeValue::Map(old_map) = self.node.as_ref() {
//             let logger = &self.domain.logger;

//             if let Some(old_item) = old_map.map.get(&key) {
//                 let new_item = item.node;

//                 let mut new_hash_map = old_map.map.clone();
//                 new_hash_map.insert(key.to_string(), new_item.clone());

//                 let new_map = Arc::new(NodeValue::Map(MapValue {
//                     map: new_hash_map.clone()
//                 }));


//                 logger.log_value_changed(
//                     AccessKey::Key(key), new_item, old_item.clone());

//                 return MapCell {
//                     domain: self.domain.clone(),
//                     focus: Focus::new(),
//                     node: new_map,
//                 };

//             } else {
//                 let new_item = item.node;

//                 let mut new_hash_map = old_map.map.clone();
//                 new_hash_map.insert(key.to_string(), new_item.clone());

//                 let new_map = Arc::new(NodeValue::Map(MapValue {
//                     map: new_hash_map.clone()
//                 }));


//                 logger.log_value_created(AccessKey::Key(key), new_item);

//                 return MapCell {
//                     domain: self.domain.clone(),
//                     focus: Focus::new(),
//                     node: new_map,
//                 };
//             }
//         }

//         panic!("This node should be a NodeValue::List");
//     }


//     pub fn remove(&self, key: String) -> Self {
//         if let NodeValue::Map(old_map) = self.node.as_ref() {
//             let logger = &self.domain.logger;

//             if let Some(old_item) = old_map.map.get(&key) {

//                 let mut new_hash_map = old_map.map.clone();
//                 new_hash_map.remove(&key);

//                 let new_map = Arc::new(NodeValue::Map(MapValue {
//                     map: new_hash_map.clone()
//                 }));

//                 logger.log_value_removed(AccessKey::Key(key), old_item.clone());

//                 return MapCell {
//                     domain: self.domain.clone(),
//                     focus: Focus::new(),
//                     node: new_map,
//                 };

//             } else {
//                 return MapCell {
//                     domain: self.domain.clone(),
//                     focus: Focus::new(),
//                     node: self.node.clone(),
//                 };
//             }
//         }

//         panic!("This node should be a NodeValue::List");
//     }


// }

// impl ::std::fmt::Debug for MapCell {
//     fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
//         self.node.fmt(fmt)
//     }
// }


impl Clone for MapValue {
    fn clone(&self) -> Self {
        MapValue {
            map: self.map.clone()
        }
    }
}

impl ::std::fmt::Debug for MapValue {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut first = true;

        fmt.write_str("{")?;

        for (k, v) in self.map.iter() {
            if !first {
                fmt.write_str(", ")?;
            }

            k.fmt(fmt)?;
            fmt.write_str(": ")?;
            v.fmt(fmt)?;
            first = false;
        }

        fmt.write_str("}")
    }
}

// impl MapValue {
//     pub fn new() -> Self {
//         MapValue {
//             map: HashMap::new(),
//         }
//     }

//     pub fn get_item(&self, key: &String) -> Option<Arc<NodeValue>> {
//         match self.map.get(key) {
//             Some(old_node) => Some(old_node.clone()),
//             _ => None
//         }
//     }

//     pub fn set_value(&mut self, key: String, value: Arc<NodeValue>) -> &mut Self {

//         self.map.insert(key,  value);
//         self
//     }

//     pub fn remove(&self, key: &String) -> MapValue {
//         let mut new_map = self.map.clone();
//         new_map.remove(key);

//         MapValue { map: new_map }
//     }

//     pub fn clear(&self) -> MapValue {
//         let mut new_map = self.map.clone();
//         new_map.clear();

//         MapValue { map: new_map }
//     }

//     pub fn len(&self) -> usize {
//         self.map.len()
//     }

//     pub fn keys(&self) -> KeysIter {
//         KeysIter {
//             iter: self.map.keys(),
//         }
//     }

//     pub fn values(&self) -> ValuesIter {
//         ValuesIter {
//             iter: self.map.values(),
//         }
//     }

//     pub fn contains_key(&self, key: &String) -> bool {
//         self.map.contains_key(key)
//     }
// }



// pub struct Iter<'a> {
//     iter: im::hashmap::Iter<'a, String, Arc<NodeValue>>,
// }

// impl<'a> Iterator for Iter<'a> {
//     type Item = (String, Arc<NodeValue>);
//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some((key, item_node)) = self.iter.next() {
//             Some((key.to_string(), item_node.clone()))
//         } else {
//             None
//         }
//     }
// }

// impl<'a> std::iter::IntoIterator for &'a MapValue {
//     type Item = (String, Arc<NodeValue>);
//     type IntoIter = Iter<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         Iter {
//             iter: self.map.iter(),
//         }
//     }
// }

// pub struct KeysIter<'a> {
//     iter: im::hashmap::Keys<'a, String, Arc<NodeValue>>,
// }

// impl<'a> Iterator for KeysIter<'a> {
//     type Item = &'a String;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.next()
//     }
// }

// pub struct ValuesIter<'a> {
//     iter: im::hashmap::Values<'a, String, Arc<NodeValue>>,
// }

// impl<'a> Iterator for ValuesIter<'a> {
//     type Item = &'a Arc<NodeValue>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.next()
//     }
// }




// #[cfg(test)]
// mod tests {

//     #[test]
//     fn map_value() {
//         use crate::map::{MapValue, NodeValue};

//         let map1 = MapValue::new()
//             .set_value("a".to_string(), NodeValue::from(10))
//             .set_value("b".to_string(), NodeValue::from("b20"))
//             .set_value("c".to_string(), NodeValue::from(3.5))
//             .set_value("d".to_string(), NodeValue::from(true));

//         println!("xxx: {:?}", map1);

//         assert_eq!(
//             map1.get_node(&"a".to_string())
//                 .unwrap()
//                 .clone()
//                 .borrow()
//                 .get_value(),
//             &NodeValue::from(10)
//         );

//         // for k in map1.keys() {
//         //     println!("key: {:?}", k);
//         // }

//         // for v in map1.values() {
//         //     println!("value: {:?}", v);
//         // }

//         // for (k, node) in &map1 {
//         //     println!("xx1: {:?} => {:?}", k, node);
//         // }

//         let map2 = map1
//             .set_value("c".to_string(), NodeValue::from(5.5))
//             .remove(&"d".to_string());

//         // for (k, node) in &map2 {
//         //     println!("xx2: {:?} => {:?}", k, node);
//         // }

//         println!("xxx: {:?}", map2);
//     }
// }
