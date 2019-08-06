
use crate::focus::{AccessKey, FocusLocator, FocusTurnTo};
use super::cell::{ValueCell};
use crate::error::Error;

use crate::domain::get_item_node;


impl ValueCell {
    pub fn focus<K: Into<AccessKey>>(self, access_key: K) -> Result<ValueCell, Error> {
        
        let access_key = access_key.into();
        let item_focus = self.focus.focus(access_key.clone());

        Ok(ValueCell {
            domain: self.domain,
            focus: item_focus,
            node: get_item_node(&self.focus, &self.node, &access_key)?,
            parent: Some(self.node),
        })
    }

    pub fn navigate(&self, path: &str) -> Result<ValueCell, Error> {
        match self.focus.turn_to(path) {
            Ok(ref to_focus) => {
                let (parent_node, new_node) = self.domain.get_focus_node(to_focus)?;
                Ok(ValueCell {
                    domain: self.domain.clone(),
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