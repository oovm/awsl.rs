use awsl_error::{Failure, Success};
use awsl_pest::{
    pest::{iterators::Pair, Parser},
    AwslParser, Rule,
};

use crate::ast::{Symbol, SymbolPath};
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
                Rule::expression => self.parse_expression(pair)?,
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

impl ParserConfig {
    fn parse_expression(&self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let r = self.get_position(&pairs);
        let mut terms = ASTNode::default();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::expr => self.parse_expr(pair)?,
                _ => debug_cases!(pair),
            };
        }
        todo!()
    }
    fn parse_expr(&self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let r = self.get_position(&pairs);
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::term => self.parse_term(pair),
                // Rule::pattern | Rule::pattern_bare => pattern = self.parse_pattern(pair),
                // Rule::expr => terms = self.parse_expr(pair),
                // Rule::block => block = self.parse_block(pair),
                // Rule::for_if => guard = Some(self.parse_expr(pair.into_inner().nth(0).unwrap())),
                // Rule::for_else => for_else = Some(self.parse_block(pair.into_inner().nth(0).unwrap())),
                _ => debug_cases!(pair),
            };
        }
        todo!()
    }
    fn parse_term(&self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let r = self.get_position(&pairs);
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::node => {
                    self.parse_node(pair)?;
                }
                // Rule::pattern | Rule::pattern_bare => pattern = self.parse_pattern(pair),
                // Rule::expr => terms = self.parse_expr(pair),
                // Rule::block => block = self.parse_block(pair),
                // Rule::for_if => guard = Some(self.parse_expr(pair.into_inner().nth(0).unwrap())),
                // Rule::for_else => for_else = Some(self.parse_block(pair.into_inner().nth(0).unwrap())),
                _ => debug_cases!(pair),
            };
        }
        todo!()
    }
    fn parse_node(&self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let r = self.get_position(&pairs);
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::data => {
                    self.parse_data(pair)?;
                }
                // Rule::pattern | Rule::pattern_bare => pattern = self.parse_pattern(pair),
                // Rule::expr => terms = self.parse_expr(pair),
                // Rule::block => block = self.parse_block(pair),
                // Rule::for_if => guard = Some(self.parse_expr(pair.into_inner().nth(0).unwrap())),
                // Rule::for_else => for_else = Some(self.parse_block(pair.into_inner().nth(0).unwrap())),
                _ => debug_cases!(pair),
            };
        }
        todo!()
    }
    fn parse_data(&self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let pair = pairs.into_inner().next()?;
        match pair.as_rule() {
            Rule::list => self.parse_list(pair),
            Rule::SYMBOL_PATH => self.parse_symbol_path(pair),
            _ => debug_cases!(pair),
        }
    }
    fn parse_list(&self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let r = self.get_position(&pairs);
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::expr => self.parse_expr(pair)?,
                // Rule::pattern | Rule::pattern_bare => pattern = self.parse_pattern(pair),
                // Rule::expr => terms = self.parse_expr(pair),
                // Rule::block => block = self.parse_block(pair),
                // Rule::for_if => guard = Some(self.parse_expr(pair.into_inner().nth(0).unwrap())),
                // Rule::for_else => for_else = Some(self.parse_block(pair.into_inner().nth(0).unwrap())),
                _ => debug_cases!(pair),
            };
        }
        todo!()
    }
    fn parse_symbol_path(&self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let r = self.get_position(&pairs);
        let mut symbol = SymbolPath::empty(4);
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::SYMBOL => symbol.push(self.parse_symbol(pair)),
                _ => debug_cases!(pair),
            };
        }
        ASTNode::symbol_path(symbol, r)
    }
    fn parse_symbol(&self, pairs: Pair<Rule>) -> Symbol {
        Symbol::new(pairs.to_string(), self.get_position(&pairs))
    }
}
