#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),

    Number(f64),

    String(String),

    Boolean(bool),

    Null,

    Binary {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
    Call {
       callee: Box<Expr>,
       arguments: Vec<Expr>,
    },
    Array {
        elements: Vec<Expr>,
    },

    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
}
