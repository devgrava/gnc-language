use crate::runtime::{Value, equals};

pub fn len(value: Value) -> Value {
    match value {
        Value::Array(values) => {
            Value::Number(values.borrow().len() as f64)
        }

        Value::String(text) => {
            Value::Number(text.len() as f64)
        }

        _ => {
            panic!("len() hanya menerima array atau string");
        }
    }
}
pub fn push(array: Value, value: Value) -> Value {
    match array {
        Value::Array(values) => {
            values.borrow_mut().push(value);

            Value::Null
        }

        _ => {
            panic!("push() hanya menerima array");
        }
    }
}

pub fn pop(array: Value) -> Value {
    match array {
        Value::Array(values) => {
            values.borrow_mut().pop().unwrap_or(Value::Null)
        }

        _ => {
            panic!("pop() hanya menerima array");
        }
    }
}

pub fn insert(array: Value, index: Value, value: Value) -> Value {
    match (array, index) {
        (Value::Array(values), Value::Number(i)) => {
            let mut values = values.borrow_mut();
            let index = i as usize;

            if index > values.len() {
                panic!("insert(): index di luar batas");
            }

            values.insert(index, value);

            Value::Null
        }

        _ => {
            panic!("insert() membutuhkan (array, number, value)");
        }
    }
}

pub fn remove(array: Value, index: Value) -> Value {
    match (array, index) {
        (Value::Array(values), Value::Number(i)) => {
            let mut values = values.borrow_mut();
            let index = i as usize;

            if index >= values.len() {
                panic!("remove(): index di luar batas");
            }

            values.remove(index)
        }

        _ => {
            panic!("remove() membutuhkan (array, number)");
        }
    }
}

pub fn clear(array: Value) -> Value {
    match array {
        Value::Array(values) => {
            values.borrow_mut().clear();

            Value::Null
        }

        _ => {
            panic!("clear() hanya menerima array");
        }
    }
}

pub fn contains(array: Value, target: Value) -> Value {
    match array {
        Value::Array(values) => {
            for item in values.borrow().iter() {
                if equals(item, &target) {
                    return Value::Boolean(true);
                }
            }

            Value::Boolean(false)
        }

        _ => {
            panic!("contains() hanya menerima array");
        }
    }
}

pub fn reverse(array: Value) -> Value {
    match array {
        Value::Array(values) => {
            values.borrow_mut().reverse();

            Value::Null
        }

        _ => {
            panic!("reverse() hanya menerima array");
        }
    }
}

pub fn first(array: Value) -> Value {
    match array {
        Value::Array(values) => {
            values
                .borrow()
                .first()
                .cloned()
                .unwrap_or(Value::Null)
        }

        _ => {
            panic!("first() hanya menerima array");
        }
    }
}

pub fn last(array: Value) -> Value {
    match array {
        Value::Array(values) => {
            values
                .borrow()
                .last()
                .cloned()
                .unwrap_or(Value::Null)
        }

        _ => {
            panic!("last() hanya menerima array");
        }
    }
}

pub fn is_empty(array: Value) -> Value {
    match array {
        Value::Array(values) => {
            Value::Boolean(values.borrow().is_empty())
        }

        _ => {
            panic!("is_empty() hanya menerima array");
        }
    }
}
