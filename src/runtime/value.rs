use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::Stmt;
use crate::runtime::Environment;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,

    Array(Rc<RefCell<Vec<Value>>>),
    Dictionary(Rc<RefCell<HashMap<String, Value>>>),

    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
        env: Environment,
    },

    StructDefinition {
        name: String,
        fields: Vec<String>,
        methods: HashMap<String, Value>,
    },

    StructInstance {
        name: String,
        fields: Rc<RefCell<HashMap<String, Value>>>,
    },
}
