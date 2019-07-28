
use std::sync::Arc;
use super::cell::{ValueCell, MapCell, ListCell};
use super::node::{NodeValue, ListValue, MapValue};
use crate::domain::Domain;
use crate::focus::{AccessKey, Focus};



pub trait DomainRootSetter<T> {
    fn set_root(&self, value: T) -> T;
}

pub trait CellBuilder {
    fn new_list(&self) -> ListCell;

    fn new_map(&self) -> MapCell;

    fn get_root(&self) -> ValueCell;
}

pub trait ValueBuilder<T> {
    fn value(&self, t: T) -> ValueCell;
}


impl CellBuilder for Arc<Domain> {

    fn new_map(&self) -> MapCell {
        MapCell {
            domain: self.clone(),
            focus: Focus::new(),
            node: Arc::new(NodeValue::Map(MapValue::new())),
        }
    }

    fn new_list(&self) -> ListCell {
        ListCell {
            domain: self.clone(),
            focus: Focus::new(),
            node: Arc::new(NodeValue::List(ListValue::new())),
        }
    }

    fn get_root(&self) -> ValueCell {
        let root_node = self.root.borrow().clone();
        ValueCell {
            domain: self.clone(),
            focus: Focus::new(),
            node: root_node
        }
    }
}

impl DomainRootSetter<ValueCell> for Arc<Domain> {
    fn set_root(&self, value: ValueCell) -> ValueCell {
        
        let new_value_node = value.node.clone();

        let old_value_node = self.root.replace(new_value_node.clone());

        self.logger.log_value_changed(AccessKey::None, 
            new_value_node.clone(), old_value_node);
        
        ValueCell {
            domain: self.clone(),
            focus: Focus::new(),
            node: new_value_node,
        }
    }    
}

impl DomainRootSetter<MapCell> for Arc<Domain> {
    fn set_root(&self, value: MapCell) -> MapCell {
        
        let new_value_node = value.node.clone();

        let old_value_node = self.root.replace(new_value_node.clone());

        self.logger.log_value_changed(AccessKey::None, 
            new_value_node.clone(), old_value_node);
        
        MapCell {
            domain: self.clone(),
            focus: Focus::new(),
            node: new_value_node,
        }
    }    
}

impl DomainRootSetter<ListCell> for Arc<Domain> {
    fn set_root(&self, value: ListCell) -> ListCell {
        
        let new_value_node = value.node.clone();

        let old_value_node = self.root.replace(new_value_node.clone());

        self.logger.log_value_changed(AccessKey::None, 
            new_value_node.clone(), old_value_node);
        
        ListCell {
            domain: self.clone(),
            focus: Focus::new(),
            node: new_value_node,
        }
    }    
}



impl ValueBuilder<&str> for Arc<Domain> {
    fn value(&self, value: &str) -> ValueCell  {
        ValueCell {
            domain: self.clone(),
            focus: Focus::new(),
            node: Arc::new(NodeValue::String(value.to_string())),
        }
    }    
}

impl ValueBuilder<String> for Arc<Domain> {
    fn value(&self, value: String) -> ValueCell  {
        ValueCell {
            domain: self.clone(),
            focus: Focus::new(),
            node: Arc::new(NodeValue::String(value)),
        }
    }    
}

impl ValueBuilder<&String> for Arc<Domain> {
    fn value(&self, value: &String) -> ValueCell  {
        ValueCell {
            domain: self.clone(),
            focus: Focus::new(),
            node: Arc::new(NodeValue::String(value.to_string())),
        }
    }    
}

impl ValueBuilder<i64> for Arc<Domain> {
    fn value(&self, value: i64) -> ValueCell  {
        ValueCell {
            domain: self.clone(),
            focus: Focus::new(),
            node: Arc::new(NodeValue::Integer(value)),
        }
    }    
}

impl ValueBuilder<f64> for Arc<Domain> {
    fn value(&self, value: f64) -> ValueCell  {
        ValueCell {
            domain: self.clone(),
            focus: Focus::new(),
            node: Arc::new(NodeValue::Float(value)),
        }
    }    
}

impl ValueBuilder<f32> for Arc<Domain> {
    fn value(&self, value: f32) -> ValueCell  {
        ValueCell {
            domain: self.clone(),
            focus: Focus::new(),
            node: Arc::new(NodeValue::Float(value as f64)),
        }
    } 
}

impl ValueBuilder<bool> for Arc<Domain> {
    fn value(&self, value: bool) -> ValueCell  {
        ValueCell {
            domain: self.clone(),
            focus: Focus::new(),
            node: Arc::new(NodeValue::Bool(value)),
        }
    }    
}
