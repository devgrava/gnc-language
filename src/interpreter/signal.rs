use crate::runtime::Value;

#[derive(Debug)]
pub enum RuntimeSignal {
    Break,
    Continue,
    Return(Value),
}
