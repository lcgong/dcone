use std::sync::Arc;
use std::cell::RefCell;
use crate::log::ChangeLogger;
use crate::focus::{AccessKey, Focus};

use crate::cell::ValueCell;
use crate::node::NodeValue;
// use crate::cell::cell::ValueCell;
use crate::error::Error;

#[derive(PartialEq)]
pub struct Domain {
    pub logger: ChangeLogger,
    pub root_node: RefCell<Arc<NodeValue>>,
    pub root_focus: Arc<Focus>
    
}

pub struct DomainUtil(Arc<Domain>);

impl DomainUtil {
    pub fn new() -> Self { 
        DomainUtil(Arc::new(Domain {
            logger: ChangeLogger::new(),
            root_node: RefCell::new(Arc::new(NodeValue::None)),
            root_focus: Focus::new()
        }))
    }


    pub fn root(&self) -> ValueCell {
        let root_node = self.0.root_node.borrow().clone();
        ValueCell {
            domain: self.0.clone(),
            focus: self.0.root_focus.clone(),
            node: root_node
        }
    }

    #[inline]
    pub fn navigate(&self, path: &str) -> Result<ValueCell, Error> {
        self.root().navigate(path)
    }


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
    pub(crate) fn get_focus_node<'a>(&self, focus: &'a Arc<Focus>) 
        -> Result<Arc<NodeValue>, Error> {

        // TODO 使用cache机制以便加速访问
        if let Some(ref parent_focus) = focus.parent_focus {
            let parent_node = self.get_focus_node(parent_focus)?;
            get_item_node(&parent_focus, &parent_node, &focus.access_key)
        } else {
            let root_node = self.root_node.borrow().clone();
            Ok(root_node)
        }
    }

    /// 设置该domain的root节点
    pub(crate) fn set_root(&self, value: Arc<NodeValue>) {

        let root_focus = self.root_focus.clone();
        // let old_root = domain.root_node.replace(value);
        let mut root_node = self.root_node.borrow_mut();
        
        self.logger.value_changed(&root_node, &root_focus, &value, &root_node);

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
fn get_item_node<>(parent_focus: &Arc<Focus>, parent_node: &Arc<NodeValue>, 
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
