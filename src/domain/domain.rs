use std::sync::Arc;
use std::cell::RefCell;
use crate::focus::{AccessKey, Focus, FocusLocator};
use crate::node::NodeValue;
use crate::error::Error;

use super::log::ChangeLogger;

pub struct Domain {
    pub logger: ChangeLogger,
    pub root_node: RefCell<Arc<NodeValue>>,
    pub root_focus: Arc<Focus>
    
}

impl Domain {
    pub fn new() -> Arc<Domain> {
        Arc::new(Domain {
            logger: ChangeLogger::new(),
            root_node: RefCell::new(Arc::new(NodeValue::None)),
            root_focus: Focus::new()
        })
    }

    /// 取得focus对应的NodeValue，从root开始层层查找
    pub(crate) fn get_focus_node<'a>(
        &self, 
        focus: &'a Arc<Focus>
    ) -> Result<(Option<Arc<NodeValue>>, Arc<NodeValue>), Error> {

        // TODO 使用cache机制以便加速访问
        if let Some(ref parent_focus) = focus.parent_focus {
            let (_, parent_node) = self.get_focus_node(parent_focus)?;
            let item_node = get_item_node(&parent_focus, &parent_node, &focus.get_access_key())?;
            Ok((Some(parent_node), item_node))
        } else {
            let root_node = self.root_node.borrow().clone();
            Ok((None, root_node))
        }
    }

    /// 设置该domain的root节点
    pub(crate) fn set_root(&self, value: Arc<NodeValue>) {

        let root_focus = self.root_focus.clone();
        let mut root_node = self.root_node.borrow_mut();

        let none_node = Arc::new(NodeValue::None);

        self.log_root_updated(root_focus, root_node.clone(), value.clone());

        *root_node = value;
    } 

    /// 取得根节点
    #[inline]
    pub(crate) fn get_root(&self) -> Arc<NodeValue> {
        self.root_node.borrow().clone()
    }
}


 
// 从parent_node里获得其item节点
#[inline]
pub fn get_item_node(parent_focus: &Arc<Focus>, parent_node: &Arc<NodeValue>, 
    access_key: &AccessKey) ->  Result<Arc<NodeValue>, Error> {
    
    match (parent_node.as_ref(), access_key) {
        (NodeValue::Map(map_value), AccessKey::Key(ref key)) => {
            match map_value.get_item(key) {
                Some(item_node) => {
                    Ok(item_node.clone())
                },
                None => {
                    Error::no_such_item(parent_focus, access_key)
                }
            }
        },
        (NodeValue::List(list_value), AccessKey::Index(index)) => {
            match list_value.get_item(*index) {
                Some(item_node) => {
                    Ok(item_node.clone())
                },
                None => {
                    Error::no_such_item(parent_focus, access_key)
                }
            }
        },
        _ => {
            Error::mismatched_access_key(parent_focus, access_key)
        }
    }
}
