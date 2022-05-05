use super::*;

mod methods;

/// ## For In Loop
///
/// ```awsl
/// for $pattern in $terms if $guard {
///     $block
/// }
/// else {
///     $for_else
/// }
/// ```
///
///
///
/// ```awsl
/// let items = $terms.filter($guard)
/// if items is empty {
///     $for_else
/// }
/// else {
///     for $pattern in items {
///         $block
///     }
/// }
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ForInLoop {
    pub pattern: ASTNode,
    pub terms: ASTNode,
    pub guard: Option<ASTNode>,
    pub block: ASTNode,
    pub for_else: Option<ASTNode>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ControlKeyword {
    Return,
    Break,
    Continue,
    Yield,
}

/// ## For In Loop
///
/// ```awsl
/// for $pattern in $terms if $guard {
///     $block
/// }
/// else {
///     $for_else
/// }
/// ```
///
///
///
/// ```awsl
/// let items = $terms.filter($guard)
/// if items is empty {
///     $for_else
/// }
/// else {
///     for $pattern in items {
///         $block
///     }
/// }
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IfElseChain {
    pub pairs: Vec<(ASTNode, ASTNode)>,
    pub cover: Option<ASTNode>,
}
