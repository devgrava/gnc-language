use crate::runtime::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub fn init() {}

pub fn trim(value: Value) -> Value {
    match value {
        Value::String(text) => {
            Value::String(text.trim().to_string())
        }

        _ => {
            panic!("trim() hanya menerima string");
        }
    }
}

pub fn upper(value: Value) -> Value {
    match value {
        Value::String(text) => {
            Value::String(text.to_uppercase())
        }

        _ => {
            panic!("upper() hanya menerima string");
        }
    }
}

pub fn lower(value: Value) -> Value {
    match value {
        Value::String(text) => {
            Value::String(text.to_lowercase())
        }

        _ => {
            panic!("lower() hanya menerima string");
        }
    }
}

pub fn contains(text: Value, pattern: Value) -> Value {
    match (text, pattern) {
        (Value::String(text), Value::String(pattern)) => {
            Value::Boolean(text.contains(&pattern))
        }

        _ => {
            panic!("contains() hanya menerima dua string");
        }
    }
}

pub fn starts_with(text: Value, prefix: Value) -> Value {
    match (text, prefix) {
        (Value::String(text), Value::String(prefix)) => {
            Value::Boolean(text.starts_with(&prefix))
        }

        _ => {
            panic!("starts_with() hanya menerima dua string");
        }
    }
}

pub fn ends_with(text: Value, suffix: Value) -> Value {
    match (text, suffix) {
        (Value::String(text), Value::String(suffix)) => {
            Value::Boolean(text.ends_with(&suffix))
        }

        _ => {
            panic!("ends_with() hanya menerima dua string");
        }
    }
}

pub fn replace(text: Value, from: Value, to: Value) -> Value {
    match (text, from, to) {
        (
            Value::String(text),
            Value::String(from),
            Value::String(to),
        ) => {
            Value::String(text.replace(&from, &to))
        }

        _ => {
            panic!("replace() hanya menerima string");
        }
    }
}

pub fn split(text: Value, separator: Value) -> Value {
    match (text, separator) {
        (
            Value::String(text),
            Value::String(separator),
        ) => {

            let values = text
                .split(&separator)
                .map(|s| Value::String(s.to_string()))
                .collect::<Vec<Value>>();

            Value::Array(Rc::new(RefCell::new(values)))
        }

        _ => {
            panic!("split() hanya menerima dua string");
        }
    }
}
