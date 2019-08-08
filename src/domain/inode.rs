use super::cone::Cone;
use super::log::{NodeEvent, PendingUpdate};
use crate::focus::{AccessKey, Focus, FocusLocator};
use crate::node::NodeValue;
use std::sync::Arc;

impl Cone {
    pub(crate) fn pending_inode_update(
        &self,
        focus: Arc<Focus>,
        old_node: Arc<NodeValue>,
        new_node: Arc<NodeValue>,
    ) {
        // self.push_pending(focus, old_node, new_node);
        self.push_pending(focus.clone(), old_node.clone(), new_node.clone());

        // self.update_internal_node(focus, old_node, new_node);
    }

    pub fn solve_pending_at(&self, focus: &Arc<Focus>) {
        let mut pending = self.logger.pending.write().unwrap();
        if pending.len() == 0 {
            return;
        }

        // 将focus对应的节点，连续变更合并一条“线”，由于并发，可能存在多条线，然后合并

        let sorted_focueses = {
            let mut pending_focuses = pending.keys().cloned().collect::<Vec<Arc<Focus>>>();
            pending_focuses.sort_by(|a, b| a.cmp(b).reverse()); // 从深到浅排序
            pending_focuses
            // 取得一个最深的节点
            // pending_focuses.pop().unwrap().clone()
        };

        for deepest_focus in sorted_focueses {
            let updating_lines = build_update_lines(&pending.get(&deepest_focus).unwrap());

            assert!(updating_lines.len() <= 1);

            for line in updating_lines {
                println!("Path: {:?}", deepest_focus.access_path());
                for u in &line {
                    println!("11111 {:p} => {:p}", u.old_node, u.new_node);
                }

                if line.len() >= 2 {
                    for upd in &line {
                        self.log_internal_line_updated(
                            upd.focus.clone(),
                            upd.old_node.clone(),
                            upd.new_node.clone(),
                        );
                    }
                }

                if let Some(_) = (&deepest_focus).get_parent() {
                    let upd = self.update_internal_node(
                        deepest_focus.clone(),
                        line.first().unwrap().old_node.clone(),
                        line.last().unwrap().new_node.clone(),
                    );

                    pending.get_mut(&upd.focus).unwrap().push(upd);
                } else {
                    let old_node = line.first().unwrap().old_node.clone();
                    let new_node = line.last().unwrap().new_node.clone();

                    self.log_internal_root_updated(deepest_focus.clone(), old_node, new_node.clone());
                    self.remount_root(new_node);
                }
            }
            pending.remove(&deepest_focus);
        }

        // println!("ZZZ\n{:?}", lines);
    }

    pub fn update_internal_node(
        &self,
        focus: Arc<Focus>,
        old_node: Arc<NodeValue>,
        new_node: Arc<NodeValue>,
    ) -> PendingUpdate {
        // 取得旧节点的父节点，向上更新
        if let Some(old_parent) = self.get_parent_node(&old_node) {
            // println!("222 {:?} {:?}", old_parent, focus);
            let (new_parent, _old_item) = match (old_parent.as_ref(), focus.get_access_key()) {
                (NodeValue::Map(map_value), AccessKey::Key(ref key)) => {
                    let (new_map, old_item) = map_value.set_item(key.to_string(), new_node.clone());
                    let new_parent_node = Arc::new(NodeValue::Map(new_map));
                    (new_parent_node, old_item)
                }
                (NodeValue::List(list_value), AccessKey::Index(index)) => {
                    let (new_list, old_item) = list_value.set_item(index, new_node.clone());
                    let new_parent_node = Arc::new(NodeValue::List(new_list));
                    (new_parent_node, old_item)
                }
                (_, _) => {
                    panic!(
                        "mismatch map/list with access_key while accessing internal node at '{}'",
                        focus.access_path()
                    );
                }
            };

            self.log_inode_updated(
                focus.clone(),
                old_parent.clone(),
                old_node,
                new_node,
                new_parent.clone(),
            );

            PendingUpdate {
                focus: focus.get_parent().unwrap().clone(),
                old_node: old_parent,
                new_node: new_parent,
            }
        } else {
            panic!("")
            // update root internally

            // self.log_internal_root_updated(focus, old_node.clone(), new_node.clone());
            // self.remount_root(new_node);
        }
    }

    pub fn log_inode_updated(
        &self,
        focus: Arc<Focus>,
        old_parent: Arc<NodeValue>,
        old_value: Arc<NodeValue>,
        new_value: Arc<NodeValue>,
        new_parent: Arc<NodeValue>,
    ) {
        let logger = &self.logger;

        let txid = logger.new_txid();

        logger.push(NodeEvent::InternalNodeUpdated {
            txid: txid,
            focus: focus.clone(),
            value: new_value.clone(),
        });

        self.push_parent_node(new_value.clone(), new_parent.clone());
        self.push_change(old_value, new_value.clone());

        // if let Some(parent) = focus.get_parent() {
        //     self.pending_inode_update(parent.clone(), old_parent.clone(), new_parent.clone());
        // } else {
        //     // the root node
        //     panic!("")
        // }
    }

    // pub fn update_internal_node(
    //     &self,
    //     focus: Arc<Focus>,
    //     old_node: Arc<NodeValue>,
    //     new_node: Arc<NodeValue>,
    // ) {
    //     // 取得旧节点的父节点，向上更新
    //     if let Some(old_parent) = self.get_parent_node(&old_node) {
    //         // println!("222 {:?} {:?}", old_parent, focus);
    //         let (new_parent, _old_item) = match (old_parent.as_ref(), focus.get_access_key()) {
    //             (NodeValue::Map(map_value), AccessKey::Key(ref key)) => {
    //                 let (new_map, old_item) = map_value.set_item(key.to_string(), new_node.clone());
    //                 let new_parent_node = Arc::new(NodeValue::Map(new_map));
    //                 (new_parent_node, old_item)
    //             }
    //             (NodeValue::List(list_value), AccessKey::Index(index)) => {
    //                 let (new_list, old_item) = list_value.set_item(index, new_node.clone());
    //                 let new_parent_node = Arc::new(NodeValue::List(new_list));
    //                 (new_parent_node, old_item)
    //             }
    //             (_, _) => {
    //                 panic!(
    //                     "mismatch map/list with access_key while accessing internal node at '{}'",
    //                     focus.access_path()
    //                 );
    //             }
    //         };

    //         self.log_inode_updated(
    //             focus.clone(),
    //             old_parent.clone(),
    //             old_node,
    //             new_node,
    //             new_parent,
    //         );
    //     } else {
    //         // update root internally

    //         self.log_internal_root_updated(focus, old_node.clone(), new_node.clone());
    //         self.remount_root(new_node);
    //     }
    // }

    // pub fn log_inode_updated(
    //     &self,
    //     focus: Arc<Focus>,
    //     old_parent: Arc<NodeValue>,
    //     old_value: Arc<NodeValue>,
    //     new_value: Arc<NodeValue>,
    //     new_parent: Arc<NodeValue>,
    // ) {
    //     let logger = &self.logger;

    //     let txid = logger.new_txid();

    //     logger.push(NodeEvent::InternalNodeUpdated {
    //         txid: txid,
    //         focus: focus.clone(),
    //         value: new_value.clone(),
    //     });

    //     self.push_parent_node(new_value.clone(), new_parent.clone());
    //     self.push_change(old_value, new_value.clone());

    //     if let Some(parent) = focus.get_parent() {
    //         self.pending_inode_update(parent.clone(), old_parent.clone(), new_parent.clone());
    //     } else {
    //         // the root node
    //         panic!("")
    //     }
    // }
}

fn build_update_lines(updates: &Vec<PendingUpdate>) -> Vec<Vec<PendingUpdate>> {
    let mut lines: Vec<Vec<PendingUpdate>> = Vec::new();
    let mut prev_new_nodes: Vec<&Arc<NodeValue>> = Vec::new();
    let mut update_iter = updates.iter();

    if let Some(upd) = update_iter.next() {
        let mut line: Vec<PendingUpdate> = Vec::new();
        line.push((*upd).clone());

        lines.push(line);
        prev_new_nodes.push(&upd.new_node);
    } else {
        return lines;
    }

    // 假设updates是按照时间顺序追加的，
    // 按顺序检查updates，如果前后new_node和old_node相等连续，合并成一线，
    // 否则另开一条线
    for upd in update_iter {
        for line_idx in 0..lines.len() {
            if *prev_new_nodes[line_idx] == upd.old_node {
                lines[line_idx].push((*upd).clone());
            } else {
                // a new updating line
                let mut line: Vec<PendingUpdate> = Vec::new();
                line.push((*upd).clone());

                lines.push(line);
                prev_new_nodes.push(&upd.new_node);
            }
        }
    }

    lines
}
