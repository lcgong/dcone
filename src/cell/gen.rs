use std::sync::Arc;

use crate::domain::{Domain, set_domain_root, get_domain_root};
use crate::focus::{AccessKey, Focus, FocusLocator};
use crate::node::{NodeValue, MapValue, ListValue};
// use crate::log::ChangeLogger;
// use super::cell::{ValueCell, MapCell, ListCell};
use super::cell::{ValueCell};


impl ValueCell {
    pub fn empty_map(&self) -> ValueCell {

        let domain = &self.domain;
        let parent_focus = &self.focus;
        let parent_node = &self.node;
        
        let root_node =  &get_domain_root(domain);
        let logger = &domain.logger;

        match parent_node.as_ref() {
            NodeValue::None  => {
                let new_value_node = Arc::new(NodeValue::Map(
                        MapValue::new()
                ));

                logger.value_created(root_node, &parent_focus, &new_value_node);

                update_upward_nodes(domain, parent_focus, new_value_node.clone());
                
                ValueCell {
                    domain: domain.clone(),
                    focus: parent_focus.clone(),
                    node: new_value_node,
                }
            },
            _ => {
            panic!("")        
            }
        }
    }

    pub fn empty_list(&self) -> ValueCell {

        let domain = &self.domain;
        let parent_focus = &self.focus;
        let parent_node = &self.node;
        
        let root_node =  &get_domain_root(domain);
        let logger = &domain.logger;

        match parent_node.as_ref() {
            NodeValue::None  => {
                let new_value_node = Arc::new(NodeValue::List(
                        ListValue::new()
                ));

                logger.value_created(root_node, &parent_focus, &new_value_node);

                update_upward_nodes(domain, parent_focus, new_value_node.clone());
                
                ValueCell {
                    domain: domain.clone(),
                    focus: parent_focus.clone(),
                    node: new_value_node,
                }
            },
            _ => {
            panic!("")        
            }
        }
    }


}

impl ValueCell {

    pub fn set_map_item<T: Into<AccessKey>>(&self, access_key: T) -> ValueCell {

        let domain = &self.domain;
        let parent_focus = &self.focus;
        let parent_node = &self.node;
        
        let access_key = access_key.into();
        let root_node =  &get_domain_root(domain);
        let item_focus = parent_focus.focus(access_key.clone());
        let logger = &domain.logger;

        let new_item_node = Arc::new(NodeValue::Map(MapValue::new()));

        match (parent_node.as_ref(), access_key) {
            (NodeValue::Map(map_value), AccessKey::Key(ref key)) => {


                let new_parent_node = Arc::new(NodeValue::Map(
                        map_value.set_item(key.to_string(), new_item_node.clone())
                    ));

                logger.value_created(root_node, &item_focus, &new_item_node);

                update_upward_nodes(domain, parent_focus, new_parent_node);
                
                ValueCell {
                    domain: domain.clone(),
                    focus: item_focus,
                    node: new_item_node,
                }
            },
            _ => {
                panic!("The parent node shoule be a Map")        
            }
        }


    } 

    pub fn set_list_item<T: Into<AccessKey>>(&self, access_key: T) -> ValueCell {

        let domain = &self.domain;
        let parent_focus = &self.focus;
        let parent_node = &self.node;
        
        let access_key = access_key.into();
        let root_node =  &get_domain_root(domain);
        let item_focus = parent_focus.focus(access_key.clone());
        let logger = &domain.logger;

        let new_item_node = Arc::new(NodeValue::List(ListValue::new()));

        match (parent_node.as_ref(), access_key) {
            (NodeValue::List(list_value), AccessKey::Index(index)) => {

                let new_parent_node = Arc::new(NodeValue::List(
                        list_value.set_item(index, new_item_node.clone())
                    ));

                logger.value_created(root_node, &item_focus, &new_item_node);

                update_upward_nodes(domain, parent_focus, new_parent_node);
                
                ValueCell {
                    domain: domain.clone(),
                    focus: item_focus,
                    node: new_item_node,
                }            
            },
            _ => {
                panic!("The parent node should be a List")        
            }
        }


    }

    pub fn set_item<K: Into<AccessKey>, V: Into<NodeValue>>(&self, 
        access_key: K, item_node: V) -> ValueCell {
        
        
        let domain = &self.domain;
        let parent_focus = &self.focus;
        let parent_node = &self.node;            
        
        let access_key = access_key.into();
        let root_node =  &get_domain_root(domain);
        let item_focus = parent_focus.focus(access_key.clone());
        let logger = &domain.logger;

        let new_item_node = Arc::new(item_node.into());

        match (parent_node.as_ref(), access_key) {
            (NodeValue::Map(map_value), AccessKey::Key(ref key)) => {


                let new_parent_node = Arc::new(NodeValue::Map(
                        map_value.set_item(key.to_string(), new_item_node.clone())
                    ));

                logger.value_created(root_node, &item_focus, &new_item_node);

                update_upward_nodes(domain, parent_focus, new_parent_node.clone());
                
                ValueCell {
                    domain: domain.clone(),
                    focus: parent_focus.clone(),
                    node: new_parent_node,
                }
            },
            (NodeValue::List(list_value), AccessKey::Index(index)) => {

                let new_parent_node = Arc::new(NodeValue::List(
                        list_value.set_item(index, new_item_node.clone())
                    ));

                logger.value_created(root_node, &item_focus, &new_item_node);

                update_upward_nodes(domain, parent_focus, new_parent_node.clone());
                
                ValueCell {
                    domain: domain.clone(),
                    focus: parent_focus.clone(),
                    node: new_parent_node,
                }            
            },
            _ => {
            panic!("")        
            }
        }
    }

}

impl ValueCell {

    pub fn push_map_item(&self) -> ValueCell {

        let new_item_node = Arc::new(NodeValue::Map(MapValue::new()));
        self._push_item(new_item_node)
    }

    pub fn push_list_item(&self) -> ValueCell {

        let new_item_node = Arc::new(NodeValue::List(ListValue::new()));
        self._push_item(new_item_node)
    }

    pub fn push_item<V: Into<NodeValue>>(&self, item_node: V) -> ValueCell {

        let new_item_node = Arc::new(item_node.into());
        self._push_item(new_item_node)
    }

    #[inline]
    fn _push_item(&self, new_item_node: Arc<NodeValue>) -> ValueCell {
        
        let domain = &self.domain;
        let parent_focus = &self.focus;
        let parent_node = &self.node;            
        
        let root_node =  &get_domain_root(domain);
        let logger = &domain.logger;

        match parent_node.as_ref() {
            NodeValue::List(list_value) => {
                let list_value = list_value.push(new_item_node.clone());
                let item_focus = parent_focus.focus(list_value.len() - 1);

                let new_parent_node = Arc::new(NodeValue::List(list_value));

                logger.value_created(root_node, &item_focus, &new_item_node);

                update_upward_nodes(domain, parent_focus, new_parent_node.clone());
                
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
        access_key: K, item_node: V) -> ValueCell {
        
        
        let domain = &self.domain;
        let parent_focus = &self.focus;
        let parent_node = &self.node;            
        
        let access_key = access_key.into();
        let root_node =  &get_domain_root(domain);
        let item_focus = parent_focus.focus(access_key.clone());
        let logger = &domain.logger;

        let new_item_node = Arc::new(item_node.into());

        match (parent_node.as_ref(), access_key) {
            (NodeValue::List(list_value), AccessKey::Index(index)) => {

                let new_parent_node = Arc::new(NodeValue::List(
                        list_value.insert(index, new_item_node.clone())
                    ));

                logger.value_created(root_node, &item_focus, &new_item_node);

                update_upward_nodes(domain, parent_focus, new_parent_node.clone());
                
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

impl ValueCell {
    pub fn focus<K: Into<AccessKey>>(&self, access_key: K) -> ValueCell {
        
        let access_key = access_key.into();
        let item_focus = self.focus.focus(access_key.clone());

        let access_key = access_key.into();
        match (self.node.as_ref(), access_key) {
            (NodeValue::Map(map_value), AccessKey::Key(ref key)) => {
                match map_value.get_item(key) {
                    Some(item_node) => {
                        ValueCell {
                            domain: self.domain.clone(),
                            focus: item_focus,
                            node: item_node.clone(),
                        }                
                    },
                    None => {
                        panic!("");
                    }
                }
            },
            (NodeValue::List(list_value), AccessKey::Index(index)) => {
                match list_value.get_item(index) {
                    Some(item_node) => {
                        ValueCell {
                            domain: self.domain.clone(),
                            focus: item_focus,
                            node: item_node.clone(),
                        }                
                    },
                    None => {
                        panic!("");
                    }
                }
            },
            _ => {
                panic!("");
            }
        }

    }
}



// /// 返回(需要更新的容器节点, 保留或新建的项目节点)
// fn gen_new_map_node(logger: &ChangeLogger, root: &Arc<NodeValue>, focus: &Arc<Focus>, 
//     key: &String, node: &Arc<NodeValue>) -> (Option<Arc<NodeValue>>, Arc<NodeValue>) {

//     match node.as_ref() {
//         NodeValue::None => {
//             // 当前节点为None，根据access_key类型将当前节点更新为空Map或List，
//             // 然后建立access_key的None项目节点
//             let new_item_node = Arc::new(NodeValue::None);

//             let new_map_value = MapValue::new();
//             let new_map_node = Arc::new(NodeValue::Map(
//                 new_map_value.set_item(key.to_string(), new_item_node.clone())
//             ));

//             logger.value_created(root, focus, &new_item_node);

//             return (Some(new_map_node), new_item_node)
//         },
//         NodeValue::Map(map_value) => {
//             // 当前节点access_key所需的Map已经存在，返回item节点或新建item节点
//             if let Some(item_node) = map_value.map.get(key) { 
//                 // key的value node的已经存在
//                 (None, item_node.clone())

//             } else { // key为新值
//                 let new_item_node = Arc::new(NodeValue::None);

//                 let mut new_hash_map = map_value.map.clone();
//                 new_hash_map.insert(key.to_string(), new_item_node.clone());

//                 let new_map_node = Arc::new(NodeValue::Map(MapValue {
//                     map: new_hash_map.clone()
//                 }));

//                 logger.value_created(root, focus, &new_item_node);

//                 return (Some(new_map_node), new_item_node);
//             }
//         },
//         _ => {
//             panic!("This node should be a NodeValue::Map or NodeValue::None");
//         }
//     }
// }


// fn generate_new_list_node(logger: &ChangeLogger, root: &Arc<NodeValue>, 
//     focus: &Arc<Focus>, index: isize, node: &Arc<NodeValue>) 
//     -> (Option<Arc<NodeValue>>, Arc<NodeValue>) {

//     let index = index as usize;
//     match node.as_ref() {
//         NodeValue::None => {
//             // 当前节点为None，根据access_key类型将当前节点更新为空List，
//             // 然后建立access_key的None项目节点
//             let new_item_node = Arc::new(NodeValue::None);

//             let mut new_value = ListValue::new();
//             // TODO unsafe isize as usize
//             new_value.list.insert(index, new_item_node.clone());

//             let new_list_node = Arc::new(NodeValue::List(new_value));

//             logger.value_created(root, focus, &new_item_node);

//             return (Some(new_list_node), new_item_node)
//         },
//         NodeValue::List(list_value) => {
//             // 当前节点access_key所需的Map已经存在，返回item节点或新建item节点
//             if let Some(item_node) = list_value.list.get(index) { 
//                 // key的value node的已经存在
//                 (None, item_node.clone())

//             } else { // key为新值
//                 let new_item_node = Arc::new(NodeValue::None);

//                 let mut new_vector = list_value.list.clone();
//                 new_vector.insert(index, new_item_node.clone());

//                 let new_list_node = Arc::new(NodeValue::List(ListValue {
//                     list: new_vector.clone()
//                 }));

//                 logger.value_created(root, focus, &new_item_node);

//                 return (Some(new_list_node), new_item_node);
//             }
//         },
//         _ => {
//             panic!("This node should be a NodeValue::List or NodeValue::None");
//         }
//     }
// }

fn update_upward_nodes(domain: &Arc<Domain>, focus: &Arc<Focus>, 
    new_node: Arc<NodeValue>) {

    let logger = &domain.logger;

    
    let orig_root = &get_domain_root(domain);
    
    let mut new_node = new_node;
    traverse_upward_nodes(orig_root, &focus, 
        | focus, parent_node, children_node | {
        
            match (parent_node.as_ref(), focus.get_access_key()) {
                (NodeValue::Map(map_value),  AccessKey::Key(ref key)) => {

                    let node = Arc::new(NodeValue::Map(
                            map_value.set_item(key.to_string(), new_node.clone())
                        ));

                    logger.value_changed(orig_root, focus, &new_node, children_node);

                    new_node = node;
                },
                (NodeValue::List(list), AccessKey::Index(index)) => {

                    let node = Arc::new(NodeValue::List(
                            list.set_item(index, new_node.clone())
                        ));

                    logger.value_changed(orig_root, focus, &new_node, children_node);

                    new_node = node;
                },
                _ => {
                    panic!("mismatched access_key with value node");
                }
            }
    });
    
    set_domain_root(&domain, new_node);
}

pub fn traverse_upward_nodes<F>(root: &Arc<NodeValue>, focus: &Arc<Focus>, mut func: F)
    where F : FnMut(&Arc<Focus>, &Arc<NodeValue>, &Arc<NodeValue>) {
    
    let mut ancestors: Vec<(&Arc<Focus>, &Arc<NodeValue>, &Arc<NodeValue>)>;
    ancestors = Vec::new(); 
    {
        let mut focuses_of_ancestors = focus.ancestors().collect::<Vec<&Arc<Focus>>>();
        focuses_of_ancestors.pop(); // the last one is the None focus

        let mut current = root;

        for f in focuses_of_ancestors.iter().rev() {
            match current.as_ref() {
                NodeValue::Map(map_node) => {
                    if let AccessKey::Key(ref key) = f.get_access_key() {
                        if let Some(item_node) = map_node.map.get(key) {
                            ancestors.push((*f, current, item_node));
                            current = item_node;
                            continue;
                        }
                    }
                    panic!("mismatched access_key with value node");
                },
                NodeValue::List(list_node) => {
                    if let AccessKey::Index(index) = f.get_access_key() {
                        
                        // TODO isize as usize 
                        let uindex = if index >= 0 {
                            index
                        } else { // negative index, -1 means the last one
                            list_node.list.len() as isize + index    
                        } as usize;

                        if let Some(item_node) = list_node.list.get(uindex) {
                            ancestors.push((*f, current, item_node));
                            current = item_node;
                            continue;
                        }
                    }
                    panic!("mismatched access_key with value node");
                },
                _ => {
                    // panic!("mismatched access_key with value node");
                }
            }
        }
    }

    for (focus, parent_node, children_node) in ancestors.iter().rev() { 
        // from leaf to root
        func(&focus, parent_node, children_node);
    }
}