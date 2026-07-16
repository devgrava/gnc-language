use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        value: Expr,
    },

    Assign {
        name: String,
        value: Expr,
    },
    
    Print {
        value: Expr,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Break,
    
    Continue,

    Function {
      name: String,
      params: Vec<String>,
      body: Vec<Stmt>,
    },
    Return {
      value: Expr,
    },
    Expression {
        expression: Expr,
    },
}
