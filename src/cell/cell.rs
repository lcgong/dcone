use std::sync::Arc;
use crate::domain::Domain;
use crate::node::NodeValue;

use crate::focus::Focus;

pub struct ValueCell {
    pub(crate) domain: Arc<Domain>,
    pub(crate) parent: Option<Arc<NodeValue>>,
    pub(crate) node: Arc<NodeValue>,
    pub(crate) focus: Arc<Focus>
}
