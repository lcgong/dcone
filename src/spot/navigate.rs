
use crate::focus::{AccessKey, FocusLocator, FocusTurnTo};
use super::spot::{Spot};
use crate::error::Error;

use crate::domain::get_item_node;


impl Spot {
    pub fn focus<K: Into<AccessKey>>(self, access_key: K) -> Result<Spot, Error> {
        
        let access_key = access_key.into();
        let item_focus = self.focus.focus(access_key.clone());

        Ok(Spot {
            cone: self.cone,
            focus: item_focus,
            node: get_item_node(&self.focus, &self.node, &access_key)?,
            parent: Some(self.node),
        })
    }

    pub fn navigate(&self, path: &str) -> Result<Spot, Error> {
        match self.focus.turn_to(path) {
            Ok(ref to_focus) => {
                let (parent_node, new_node) = self.cone.get_focus_node(to_focus)?;
                Ok(Spot {
                    cone: self.cone.clone(),
                    focus: to_focus.clone(),
                    node: new_node,
                    parent: parent_node,
                })
            },
            Err(err) => {
                Err(Error::AccessPathError(err))
            }
        }
    }

}