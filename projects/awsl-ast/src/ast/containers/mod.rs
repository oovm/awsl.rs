use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct List {
    inner: VecDeque<Expression>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Dict {}

impl From<List> for Expression {
    fn from(v: List) -> Self {
        Self::List(box v)
    }
}
