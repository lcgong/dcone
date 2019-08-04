use std::sync::Arc;

use crate::domain::Domain;
use crate::focus::{AccessKey, Focus, FocusLocator};
use crate::node::NodeValue;


pub(crate) fn upward_update_nodes(domain: &Arc<Domain>, focus: &Arc<Focus>, 
    new_node: Arc<NodeValue>) {

    let logger = &domain.logger;

    let orig_root = &domain.get_root();
    
    let mut new_node = new_node;
    traverse_upward_nodes(orig_root, &focus, 
        | focus, parent_node, children_node | {
        
            match (parent_node.as_ref(), focus.get_access_key()) {
                (NodeValue::Map(map_value),  AccessKey::Key(ref key)) => {

                    let node = Arc::new(NodeValue::Map(
                            map_value.set_item(key.to_string(), new_node.clone())
                        ));

                    logger.node_changed(focus, parent_node, &new_node, children_node);

                    new_node = node;
                },
                (NodeValue::List(list), AccessKey::Index(index)) => {

                    let node = Arc::new(NodeValue::List(
                            list.set_item(index, new_node.clone())
                        ));

                    logger.node_changed(focus, parent_node, &new_node, children_node);

                    new_node = node;
                },
                _ => {
                    panic!("mismatched access_key with value node");
                }
            }
    });


    domain.set_root(new_node);
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