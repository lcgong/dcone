
use std::sync::Arc;
use crate::spot::Spot;
use crate::error::Error;

use super::cone::Cone;
use super::log::ChangeLogger;

pub struct Domain {
    cone: Arc<Cone>
}

impl Domain {
    pub fn new() -> Self { 
        Domain {
            cone: Cone::new()
        }
    }

    pub fn root(&self) -> Spot {
        Spot {
            cone: self.cone.clone(),
            focus: self.cone.root_focus.clone(),
            node: self.cone.get_root_node(),
            parent: None,
        }
    }

    #[inline]
    pub fn navigate(&self, path: &str) -> Result<Spot, Error> {
        self.root().navigate(path)
    }

    pub fn log(&self) -> &ChangeLogger {
        &self.cone.logger
    }
}

