use std::sync::Arc;
use crate::domain::Domain;
use crate::node::NodeValue;
// use super::node::NodeValueRef;

use crate::focus::Focus;

pub struct ValueCell {
    pub(crate) domain: Arc<Domain>,
    pub(crate) node: Arc<NodeValue>,
    pub(crate) focus: Arc<Focus>
}

// pub struct ListCell {
//     pub(crate) domain: Arc<Domain>,
//     pub(crate) node: Arc<NodeValue>,
//     pub(crate) focus: Arc<Focus>
// }

// pub struct MapCell {
//     pub(crate) domain: Arc<Domain>,
//     pub(crate) node: Arc<NodeValue>,
//     pub(crate) focus: Arc<Focus>
// }

