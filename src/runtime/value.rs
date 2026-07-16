use std::rc::Rc;
use std::cell::RefCell;

use crate::ast::Stmt;
use crate::runtime::Environment;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,

    Array(Rc<RefCell<Vec<Value>>>),

    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
        env: Environment,
    },
}
