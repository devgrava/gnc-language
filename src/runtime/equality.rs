use crate::runtime::Value;

pub fn equals(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => a == b,

        (Value::String(a), Value::String(b)) => a == b,

        (Value::Boolean(a), Value::Boolean(b)) => a == b,

        (Value::Null, Value::Null) => true,

        _ => false,
    }
}
