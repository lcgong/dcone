
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

    pub fn set_item(&self, key: String, value: Arc<NodeValue>) -> (Self, Option<Arc<NodeValue>>) {

        let mut new_hash_map = self.map.clone();
        let old_item = new_hash_map.insert(key, value);

        (MapValue {
            map: new_hash_map
        }, old_item)
    }

    #[inline]
    pub fn get_item(&self, key: &String) -> Option<&Arc<NodeValue>> {
        return self.map.get(key)
    }

    pub fn remove(&self, key: &String) -> MapValue {
        let mut new_map = self.map.clone();
        new_map.remove(key);

        MapValue { 
            map: new_map 
        }
    }

    pub fn clear(&self) -> MapValue {
        let mut new_map = self.map.clone();
        new_map.clear();

        MapValue { map: new_map }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn contains_key(&self, key: &String) -> bool {
        self.map.contains_key(key)
    }
    
}


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


