pub mod expr;
pub mod stmt;

pub use expr::Expr;
pub use stmt::Stmt;

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}
