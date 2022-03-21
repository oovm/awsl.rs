mod config;
// mod regroup;

use awsl_error::{Failure, Success};
use awsl_pest::{AwslParser, Rule};
use awsl_pest::pest::Parser;
use crate::ASTNode;
pub use self::config::ParserConfig;
pub use crate::Result;
// use crate::{parser::regroup::PREC_CLIMBER, Result, SDLError};

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

impl ParserConfig {
    pub fn parse(&mut self, input: impl AsRef<str>) -> Result<ASTNode> {
        let input = input.as_ref().replace("\r\n", "\n").replace("\\\n", "").replace("\t", &" ".repeat(self.tab_size));
        let _ = AwslParser::parse(Rule::program, &input)?;
        Success(ASTNode::default())
    }
}