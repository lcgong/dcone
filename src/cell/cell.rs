use std::sync::Arc;
use crate::domain::Cone;
use crate::node::NodeValue;

use crate::focus::Focus;

pub struct ValueCell {
    pub(crate) cone: Arc<Cone>,
    pub(crate) parent: Option<Arc<NodeValue>>,
    pub(crate) node: Arc<NodeValue>,
    pub(crate) focus: Arc<Focus>
}
