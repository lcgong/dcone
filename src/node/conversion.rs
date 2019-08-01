
use super::value::NodeValue;


impl From<String> for NodeValue {
    
    #[inline]
    fn from(value: String) -> NodeValue {
        NodeValue::String(value)
    }
}

impl From<&str> for NodeValue {
    
    #[inline]
    fn from(value: &str) -> NodeValue {
        NodeValue::String(value.to_string())
    }
}

impl From<i64> for NodeValue {
    
    #[inline]
    fn from(value: i64) -> NodeValue {
        NodeValue::Integer(value)
    }
}

impl From<i32> for NodeValue {
    
    #[inline]
    fn from(value: i32) -> NodeValue {
        NodeValue::Integer(value as i64)
    }
}
