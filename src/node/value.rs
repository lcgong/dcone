use std::sync::Arc;
use super::list::ListValue;
use super::map::MapValue;

#[derive(Clone)]
pub enum NodeValue {
    None,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    List(ListValue),
    Map(MapValue),
}


impl std::fmt::Debug for NodeValue {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        match self {
            NodeValue::None => {
                fmt.write_str("None")
            },
            NodeValue::Bool(v) => {
                fmt.write_str("Bool(")?;
                v.fmt(fmt)?;
                fmt.write_str(")")
            },
            NodeValue::Integer(v) => {
                fmt.write_str("Integer(")?;
                v.fmt(fmt)?;
                fmt.write_str(")")
            },
            NodeValue::Float(v) => {
                fmt.write_str("Float(")?;
                v.fmt(fmt)?;
                fmt.write_str(")")                
            },
            NodeValue::String(v) => {
                fmt.write_str("String(")?;
                v.fmt(fmt)?;
                fmt.write_str(")")
            },
            NodeValue::Map(v) => {
                std::fmt::Debug::fmt(&v, fmt)
            },
            NodeValue::List(v) => {
                std::fmt::Debug::fmt(&v, fmt)
            },            
        }
    }
}



// impl core::cmp::PartialEq for NodeValue {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (NodeValue::None, NodeValue::None) => true,
//             (NodeValue::Bool(a), NodeValue::Bool(b)) => a == b,
//             (NodeValue::Integer(a), NodeValue::Integer(b)) => a == b,
//             (NodeValue::Float(a), NodeValue::Float(b)) => a == b,
//             (NodeValue::String(a), NodeValue::String(b)) => a == b,
//             (NodeValue::Map(a), NodeValue::Map(b)) => a == b,
//             (NodeValue::List(a), NodeValue::List(b)) => a == b,
//             _ => false,
//         }
//     }
// }


impl core::cmp::PartialEq for NodeValue {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}


impl core::cmp::Eq for NodeValue {}


impl std::hash::Hash for NodeValue {
    fn hash<H: std::hash::Hasher>(&self, into: &mut H) {
        std::ptr::hash(self, into)
    }
}
