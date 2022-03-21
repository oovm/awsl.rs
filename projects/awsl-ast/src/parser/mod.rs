use awsl_error::{Failure, Success};
use awsl_pest::{
    pest::{iterators::Pair, Parser},
    AwslParser, Rule,
};

pub use crate::Result;
use crate::{ASTKind, ASTNode};

pub use self::config::ParserConfig;

mod config;
// mod regroup;

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
        let input = input
            .as_ref()
            // forbidden CRLF
            .replace("\r\n", "\n")
            .replace("\\\n", "")
            .replace("\t", &" ".repeat(self.tab_size));
        let root = AwslParser::parse(Rule::program, &input)?;
        let range = self.get_position_root(&root);
        let mut codes = vec![];
        for pair in root {
            if let Rule::statement = pair.as_rule() {
                codes.push(self.parse_statement(pair)?);
            };
        }
        ASTNode::program(codes, range)
    }
    fn parse_statement(&self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let range = self.get_position(&pairs);
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            let code = match pair.as_rule() {
                Rule::WHITESPACE => continue,
                // Rule::expression => self.parse_expression(pair),
                // Rule::if_statement => self.parse_if_else(pair),
                Rule::for_statement => self.parse_for_in(pair)?,
                // Rule::assign_statement => self.parse_assign(pair),
                _ => debug_cases!(pair),
            };
            codes.push(code);
        }
        ASTNode::statement(codes, range)
    }

    fn parse_for_in(&self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let r = self.get_position(&pairs);
        let mut guard = None;
        let mut for_else = None;
        let (mut pattern, mut terms, mut block) = Default::default();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                // Rule::pattern | Rule::pattern_bare => pattern = self.parse_pattern(pair),
                // Rule::expr => terms = self.parse_expr(pair),
                // Rule::block => block = self.parse_block(pair),
                // Rule::for_if => guard = Some(self.parse_expr(pair.into_inner().nth(0).unwrap())),
                // Rule::for_else => for_else = Some(self.parse_block(pair.into_inner().nth(0).unwrap())),
                // _ => debug_cases!(pair),
                _ => unreachable!(),
            };
        }
        ASTNode::for_in_loop(pattern, terms, block, guard, for_else, r)
    }
}
