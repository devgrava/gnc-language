use crate::runtime::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn keys(value: Value) -> Value {
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

pub fn set(dictionary: Value, key: Value, value: Value) -> Value {
    match (dictionary, key) {
        (Value::Dictionary(map), Value::String(key)) => {
            map.borrow_mut().insert(key, value);
            Value::Null
        }

        (Value::Dictionary(_), _) => {
            panic!("set() expects the key to be a string")
        }

        _ => {
            panic!("set() expects a dictionary as the first argument")
        }
    }
}

pub fn get(dictionary: Value, key: Value, default: Option<Value>) -> Value {
    match (dictionary, key) {
        (Value::Dictionary(map), Value::String(key)) => {
            if let Some(value) = map.borrow().get(&key) {
                value.clone()
            } else {
                default.unwrap_or(Value::Null)
            }
        }

        (Value::Dictionary(_), _) => {
            panic!("get() expects the key to be a string")
        }

        _ => {
            panic!("get() expects a dictionary as the first argument")
        }
    }
}

pub fn merge(target: Value, source: Value) -> Value {
    match (target, source) {
        (Value::Dictionary(target_map), Value::Dictionary(source_map)) => {
            for (key, value) in source_map.borrow().iter() {
                target_map
                    .borrow_mut()
                    .insert(key.clone(), value.clone());
            }

            Value::Null
        }

        (Value::Dictionary(_), _) => {
            panic!("merge() expects the second argument to be a dictionary")
        }

        _ => {
            panic!("merge() expects the first argument to be a dictionary")
        }
    }
}

pub fn clone(dictionary: Value) -> Value {
    match dictionary {
        Value::Dictionary(map) => {
            let copied: HashMap<String, Value> =
                map.borrow().iter().map(|(k, v)| (k.clone(), v.clone())).collect();

            Value::Dictionary(Rc::new(RefCell::new(copied)))
        }

        _ => {
            panic!("clone() expects a dictionary")
        }
    }
}
