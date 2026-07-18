use crate::runtime::Value;

pub fn keys(value: Value) -> Value {
    use std::cell::RefCell;
    use std::rc::Rc;

    match value {
        Value::Dictionary(map) => {
            let mut keys = Vec::new();

            for key in map.borrow().keys() {
                keys.push(Value::String(key.clone()));
            }

            Value::Array(Rc::new(RefCell::new(keys)))
        }

        _ => panic!("keys() expects a dictionary"),
    }
}

pub fn values(value: Value) -> Value {
    use std::cell::RefCell;
    use std::rc::Rc;

    match value {
        Value::Dictionary(map) => {
            let mut values = Vec::new();

            for value in map.borrow().values() {
                values.push(value.clone());
            }

            Value::Array(Rc::new(RefCell::new(values)))
        }

        _ => panic!("values() expects a dictionary"),
    }
}

pub fn has_key(dictionary: Value, key: Value) -> Value {
    match (dictionary, key) {
        (Value::Dictionary(map), Value::String(key)) => {
            Value::Boolean(map.borrow().contains_key(&key))
        }

        (Value::Dictionary(_), _) => {
            panic!("has_key() expects the second argument to be a string")
        }

        _ => {
            panic!("has_key() expects the first argument to be a dictionary")
        }
    }
}

pub fn remove(dictionary: Value, key: Value) -> Value {
    match (dictionary, key) {
        (Value::Dictionary(map), Value::String(key)) => {
            let mut dictionary = map.borrow_mut();

            dictionary.remove(&key).unwrap_or(Value::Null)
        }

        (Value::Dictionary(_), _) => {
            panic!("remove() expects the key to be a string")
        }

        _ => {
            panic!("remove() expects a dictionary as the first argument")
        }
    }
}

pub fn clear(dictionary: Value) -> Value {
    match dictionary {
        Value::Dictionary(map) => {
            map.borrow_mut().clear();
            Value::Null
        }

        _ => panic!("clear() expects a dictionary"),
    }
}


