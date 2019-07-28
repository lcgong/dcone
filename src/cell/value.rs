
use crate::focus::Focus;

use super::cell::{ListCell, MapCell, ValueCell};
use super::node::NodeValue;


impl ValueCell {

    pub fn is_none(&self) -> bool {
        match self.node.as_ref() {
            NodeValue::None => true,
            _ => false
        }
    }

    pub fn is_integer(&self) -> bool {
        match self.node.as_ref() {
            NodeValue::Integer(_) => true,
            _ => false
        }
    }


    pub fn to_list(&self) -> ListCell {
        ListCell {
            domain: self.domain.clone(),
            focus: Focus::new(),
            node: self.node.clone(),
        }
    }

    pub fn to_map(&self) -> MapCell {
        MapCell {
            domain: self.domain.clone(),
            focus: Focus::new(),
            node: self.node.clone(),
        }
    }

    pub fn to_bool(&self) -> bool {
        if let NodeValue::Bool(value) = self.node.as_ref() {
            return *value;
        }

        panic!("The value is not a NodeValue::Bool");
    }

    pub fn to_i64(&self) -> i64 {
        if let NodeValue::Integer(value) = self.node.as_ref() {
            return *value;
        }

        panic!("The value is not a NodeValue::Integer");
    }

    pub fn to_f64(&self) -> f64 {
        if let NodeValue::Float(value) = self.node.as_ref() {
            return *value;
        }

        panic!("The value is not a NodeValue::Float");
    }

    pub fn to_string(&self) -> String {
        if let NodeValue::String(value) = self.node.as_ref() {
            return value.to_string();
        }

        panic!("The value is not a NodeValue::String");
    }
}




impl ::std::fmt::Debug for ValueCell {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.node.fmt(fmt)
    }
}
