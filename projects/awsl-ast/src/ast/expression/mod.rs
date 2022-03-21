mod methods;
use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Expression {
    pub nodes: Vec<ASTNode>,
    pub eos: bool,
}
