
use crate::focus::{AccessKey, FocusLocator, FocusTurnTo};
use super::cell::{ValueCell};
use crate::error::Error;

use crate::domain::get_item_node;


impl ValueCell {
    pub fn focus<K: Into<AccessKey>>(&self, access_key: K) -> Result<ValueCell, Error> {
        
        let access_key = access_key.into();
        let item_focus = self.focus.focus(access_key.clone());

        Ok(ValueCell {
            domain: self.domain.clone(),
            node: get_item_node(&self.focus, &self.node, &access_key)?,
            // node: self.domain.get_focus_node(&item_focus)?,
            focus: item_focus,
        })
    }

    pub fn navigate(&self, path: &str) -> Result<ValueCell, Error> {
        match self.focus.turn_to(path) {
            Ok(ref to_focus) => {
                let to_node = self.domain.get_focus_node(to_focus)?;
                Ok(ValueCell {
                    domain: self.domain.clone(),
                    focus: to_focus.clone(),
                    node: to_node
                })
            },
            Err(err) => {
                Err(Error::AccessPathError(err))
            }
        }
    }

}