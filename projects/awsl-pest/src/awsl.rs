pub struct AwslParser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program,
    statement,
    emptyStatement,
    eos,
    comma_or_semi,
    block_or_stmt,
    importStatement,
    use_alias,
    use_module_select,
    module_block,
    module_tuple,
    ModuleSplit,
    controlFlow,
    block,
    if_statement,
    if_single,
    if_nested,
    if_single_else,
    if_nested_else,
    else_if_block,
    if_else_block,
    condition,
    for_statement,
    for_in_loop,
    re_control,
    Return,
    Yield,
    Break,
    Pass,
    Type,
    classStatement,
    short_block,
    short_statement,
    short_annotation,
    extendStatement,
    assignStatement,
    assign_terms,
    assign_name,
    assign_pair,
    defineStatement,
    define_terms,
    define_parameter,
    define_pair,
    annotation,
    annotation_call,
    apply,
    apply_kv,
    function_name,
    function_module,
    expression,
    expr,
    term,
    node,
    tuple,
    bracket_call,
    bracket_apply,
    dot_call,
    template,
    EmptyTemplate,
    Fragment,
    OpenClose,
    SelfClose,
    text_mode,
    text_char,
    HTMLEscape,
    data,
    dict,
    list,
    slice,
    index,
    key_value,
    key_valid,
    index_range,
    index_step,
    SpecialValue,
    Byte,
    Byte_BIN,
    Byte_OCT,
    Byte_HEX,
    Number,
    Decimal,
    DecimalBad,
    Integer,
    Complex,
    String,
    StringLiteral,
    StringNormal,
    StringEmpty,
    StringLiteralText,
    StringText,
    StringStart,
    StringEnd,
    WHITESPACE,
    COMMENT,
    LineCommentSimple,
    LineCommentTodo,
    LineCommentFixme,
    LineCommentWarning,
    MultiLineComment,
    Symbol,
    namespace,
    SYMBOL,
    Modifier,
    Prefix,
    Suffix,
    Infix,
    Set,
    Or,
    LazyOr,
    Star,
    Slash,
    Solidus,
    Proportion,
    Dot,
    Comma,
    Semicolon,
    Colon,
    Question,
    Underline,
    Load,
    Save,
    LeftShift,
    RightShift,
    LessEqual,
    GraterEqual,
    Equivalent,
    NotEquivalent,
    Equal,
    NotEqual,
    Plus,
    Minus,
    Power,
    Surd,
    Increase,
    Decrease,
    To,
    Elvis,
    Quote,
    Acute,
    Apostrophe,
    Quotation,
    LogicOr,
    LogicAnd,
    LogicNot,
    Ellipsis,
    Concat,
    Destruct,
    Bang,
    Sharp,
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for AwslParser {
    fn parse<'i>(rule: Rule, input: &'i str) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic { state.sequence(|state| state.repeat(|state| super::visible::WHITESPACE(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::visible::COMMENT(state).and_then(|state| state.repeat(|state| super::visible::WHITESPACE(state))))))) } else { Ok(state) }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::SOI(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::statement(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::statement(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::statement, |state| self::emptyStatement(state).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::importStatement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::classStatement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::extendStatement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::controlFlow(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::assignStatement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::defineStatement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::annotation(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| self::expression(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn emptyStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::emptyStatement, |state| self::eos(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn eos(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::eos, |state| self::Semicolon(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn comma_or_semi(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Comma(state).or_else(|state| self::Semicolon(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn block_or_stmt(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| self::block(state)).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::Set(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::statement(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn importStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::importStatement, |state| state.sequence(|state| state.match_string("import").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::Dot(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Dot(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.restore_on_err(|state| self::use_alias(state)).or_else(|state| state.restore_on_err(|state| self::use_module_select(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn use_alias(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::use_alias, |state| state.restore_on_err(|state| state.sequence(|state| self::String(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("as")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))).or_else(|state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::ModuleSplit(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::ModuleSplit(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("as")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn use_module_select(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::use_module_select, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::ModuleSplit(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::ModuleSplit(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.sequence(|state| self::ModuleSplit(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.restore_on_err(|state| self::module_block(state)).or_else(|state| self::Star(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn module_block(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::module_block, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| self::module_tuple(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| self::comma_or_semi(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::module_tuple(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| self::comma_or_semi(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::module_tuple(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::comma_or_semi(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn module_tuple(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::module_tuple, |state| state.restore_on_err(|state| self::use_alias(state)).or_else(|state| state.restore_on_err(|state| self::use_module_select(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ModuleSplit(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Dot(state).or_else(|state| self::Proportion(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn controlFlow(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| self::re_control(state)).or_else(|state| state.restore_on_err(|state| self::if_statement(state))).or_else(|state| state.restore_on_err(|state| self::for_statement(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn block(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::block, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| self::statement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::statement(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::statement(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::if_statement, |state| state.sequence(|state| state.match_string("if").and_then(|state| super::hidden::skip(state)).and_then(|state| self::condition(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("else")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_single(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.match_string("if").and_then(|state| super::hidden::skip(state)).and_then(|state| self::condition(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_nested(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.match_string("if").and_then(|state| super::hidden::skip(state)).and_then(|state| self::condition(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::else_if_block(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::else_if_block(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::else_if_block(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_single_else(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.match_string("if").and_then(|state| super::hidden::skip(state)).and_then(|state| self::condition(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::if_else_block(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_nested_else(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.match_string("if").and_then(|state| super::hidden::skip(state)).and_then(|state| self::condition(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| self::else_if_block(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::else_if_block(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::else_if_block(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::if_else_block(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn else_if_block(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.sequence(|state| state.match_string("else").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("if"))).or_else(|state| state.match_string("ef")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::condition(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_else_block(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.match_string("else").and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn condition(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")")))).or_else(|state| state.restore_on_err(|state| self::expr(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn for_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::for_statement, |state| state.sequence(|state| state.match_string("for").and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("in")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn for_in_loop(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::for_in_loop, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("in")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn re_control(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::re_control, |state| state.restore_on_err(|state| state.sequence(|state| self::Return(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))).or_else(|state| self::Break(state)).or_else(|state| self::Pass(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Return(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Return, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("return")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Yield(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Yield, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("yield")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Break(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Break, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("break")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Pass(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Pass, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("pass")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Type(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Type, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("type")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn classStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::classStatement, |state| state.sequence(|state| state.match_string("class").and_then(|state| super::hidden::skip(state)).and_then(|state| self::assign_pair(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::short_block(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn short_block(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::short_block, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::short_statement(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::short_statement(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn short_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::short_statement, |state| self::emptyStatement(state).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| state.match_string("def")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::define_terms(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| state.match_string("let")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::assign_terms(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::short_annotation(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn short_annotation(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::short_annotation, |state| state.sequence(|state| state.sequence(|state| self::annotation_call(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::annotation_call(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::annotation_call(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::short_statement(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn extendStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::extendStatement, |state| state.sequence(|state| state.match_string("extend").and_then(|state| super::hidden::skip(state)).and_then(|state| self::Symbol(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.match_string("with").or_else(|state| self::Colon(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::short_block(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn assignStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::assignStatement, |state| state.sequence(|state| state.match_string("let").and_then(|state| super::hidden::skip(state)).and_then(|state| self::assign_terms(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn assign_terms(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| self::assign_name(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")")).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::block_or_stmt(state))))).or_else(|state| state.sequence(|state| self::assign_name(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::block_or_stmt(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn assign_name(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::assign_pair(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::assign_pair(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::assign_pair(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn assign_pair(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::assign_pair, |state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::Modifier(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Modifier(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Symbol(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn defineStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::defineStatement, |state| state.sequence(|state| state.match_string("def").and_then(|state| super::hidden::skip(state)).and_then(|state| self::define_terms(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn define_terms(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::assign_pair(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::define_parameter(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::block_or_stmt(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn define_parameter(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::define_parameter, |state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::define_pair(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::define_pair(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::define_pair(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")")))).or_else(|state| state.match_string(")")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn define_pair(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::define_pair, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Set(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn annotation(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::annotation, |state| state.sequence(|state| state.sequence(|state| self::annotation_call(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::annotation_call(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::annotation_call(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::statement(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn annotation_call(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::annotation_call, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("@").and_then(|state| state.restore_on_err(|state| self::list(state)).or_else(|state| state.restore_on_err(|state| self::apply(state))).or_else(|state| self::Symbol(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn apply(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::apply, |state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::apply_kv(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::apply_kv(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::apply_kv(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn apply_kv(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::apply_kv, |state| state.restore_on_err(|state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Colon(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))).or_else(|state| state.restore_on_err(|state| self::expr(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn function_name(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::function_name, |state| self::SYMBOL(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn function_module(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::function_module, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::namespace(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Dot(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Dot(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Dot(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn expression(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::expression, |state| state.sequence(|state| self::expr(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn expr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::expr, |state| state.sequence(|state| self::term(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Infix(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::term(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Infix(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::term(state))))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn term(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::term, |state| state.sequence(|state| state.sequence(|state| state.optional(|state| self::Prefix(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Prefix(state))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::node(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::Suffix(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Suffix(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn node(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::node, |state| state.restore_on_err(|state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")")))).or_else(|state| state.restore_on_err(|state| self::tuple(state))).or_else(|state| state.restore_on_err(|state| self::bracket_call(state))).or_else(|state| state.restore_on_err(|state| self::data(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn tuple(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::tuple, |state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn bracket_call(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::bracket_call, |state| state.sequence(|state| self::data(state).and_then(|state| state.sequence(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::WHITESPACE(state)))).and_then(|state| state.restore_on_err(|state| self::slice(state)).or_else(|state| state.restore_on_err(|state| self::apply(state)))))).and_then(|state| state.repeat(|state| state.sequence(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::WHITESPACE(state)))).and_then(|state| state.restore_on_err(|state| self::slice(state)).or_else(|state| state.restore_on_err(|state| self::apply(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn bracket_apply(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::bracket_apply, |state| state.sequence(|state| self::Symbol(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::dict(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn dot_call(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::dot_call, |state| state.sequence(|state| self::term(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Dot(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Integer(state).or_else(|state| self::Symbol(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn template(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::template, |state| self::EmptyTemplate(state).or_else(|state| state.restore_on_err(|state| self::Fragment(state))).or_else(|state| state.restore_on_err(|state| self::OpenClose(state))).or_else(|state| state.restore_on_err(|state| self::SelfClose(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EmptyTemplate(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EmptyTemplate, |state| state.match_string("</>"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Fragment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Fragment, |state| state.sequence(|state| state.match_string("<>").and_then(|state| super::hidden::skip(state)).and_then(|state| self::text_mode(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("</>"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn OpenClose(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::OpenClose, |state| state.sequence(|state| state.match_string("<").and_then(|state| super::hidden::skip(state)).and_then(|state| state.stack_push(|state| self::Symbol(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(">")).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| self::text_mode(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::text_mode(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::text_mode(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("</")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::POP(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(">"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SelfClose(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SelfClose, |state| state.sequence(|state| state.match_string("<").and_then(|state| super::hidden::skip(state)).and_then(|state| state.stack_push(|state| self::Symbol(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| self::text_mode(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::text_mode(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::text_mode(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::POP(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("/>"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn text_mode(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::text_mode, |state| state.sequence(|state| state.match_string("{").and_then(|state| state.repeat(|state| state.restore_on_err(|state| self::statement(state)))).and_then(|state| state.match_string("}"))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::template(state).and_then(|state| state.repeat(|state| state.restore_on_err(|state| self::template(state))))))).or_else(|state| state.sequence(|state| self::HTMLEscape(state).or_else(|state| self::text_char(state)).and_then(|state| state.repeat(|state| self::HTMLEscape(state).or_else(|state| self::text_char(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn text_char(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::text_char, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.lookahead(false, |state| state.match_string("{").or_else(|state| state.match_string("}")).or_else(|state| state.match_string("<")).or_else(|state| state.match_string(">"))).and_then(|state| self::ANY(state))).or_else(|state| state.match_string("{{")).or_else(|state| state.match_string("}}")).or_else(|state| state.match_string("<<")).or_else(|state| state.match_string(">>"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn HTMLEscape(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::HTMLEscape, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("&").and_then(|state| self::ASCII_ALPHA_LOWER(state)).and_then(|state| state.match_string(";"))).or_else(|state| state.sequence(|state| state.match_string("&#").and_then(|state| self::Integer(state)).and_then(|state| state.match_string(";"))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn data(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::data, |state| state.restore_on_err(|state| self::template(state)).or_else(|state| state.restore_on_err(|state| self::dict(state))).or_else(|state| state.restore_on_err(|state| self::list(state))).or_else(|state| self::SpecialValue(state)).or_else(|state| self::Byte(state)).or_else(|state| self::Number(state)).or_else(|state| state.restore_on_err(|state| self::String(state))).or_else(|state| self::Symbol(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn dict(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::dict, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::key_value(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::key_value(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::key_value(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn list(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::list, |state| state.sequence(|state| state.match_string("[").and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::expr(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn slice(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::slice, |state| state.sequence(|state| state.match_string("[").and_then(|state| super::hidden::skip(state)).and_then(|state| self::index(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::index(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::index(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn index(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::index, |state| state.restore_on_err(|state| self::index_step(state)).or_else(|state| state.restore_on_err(|state| self::index_range(state))).or_else(|state| state.restore_on_err(|state| self::expr(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn key_value(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::key_value, |state| state.sequence(|state| self::key_valid(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Colon(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn key_valid(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::key_valid, |state| self::Integer(state).or_else(|state| self::SYMBOL(state)).or_else(|state| state.restore_on_err(|state| self::String(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn index_range(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::index_range, |state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::expr(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Colon(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::expr(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn index_step(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::index_step, |state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::expr(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Colon(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::expr(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Colon(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::expr(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SpecialValue(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SpecialValue, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("true").or_else(|state| state.match_string("false")).or_else(|state| state.match_string("null"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Byte(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Byte, |state| self::Byte_BIN(state).or_else(|state| self::Byte_OCT(state)).or_else(|state| self::Byte_HEX(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Byte_BIN(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Byte_BIN, |state| state.sequence(|state| state.match_string("0").and_then(|state| state.match_string("b").or_else(|state| state.match_string("B"))).and_then(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_BIN_DIGIT(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_BIN_DIGIT(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Byte_OCT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Byte_OCT, |state| state.sequence(|state| state.match_string("0").and_then(|state| state.match_string("o").or_else(|state| state.match_string("O"))).and_then(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_OCT_DIGIT(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_OCT_DIGIT(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Byte_HEX(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Byte_HEX, |state| state.sequence(|state| state.match_string("0").and_then(|state| state.match_string("x").or_else(|state| state.match_string("X"))).and_then(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_HEX_DIGIT(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_HEX_DIGIT(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Number(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Number, |state| self::Complex(state).or_else(|state| self::Decimal(state)).or_else(|state| self::DecimalBad(state)).or_else(|state| self::Integer(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Decimal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Decimal, |state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state)).and_then(|state| self::ASCII_DIGIT(state)).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DecimalBad(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::DecimalBad, |state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state))).or_else(|state| state.sequence(|state| self::Dot(state).and_then(|state| self::ASCII_DIGIT(state)).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Integer(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Integer, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("0").or_else(|state| state.sequence(|state| self::ASCII_NONZERO_DIGIT(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_DIGIT(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Complex(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Complex, |state| state.sequence(|state| self::Decimal(state).or_else(|state| self::Integer(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn String(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::String, |state| state.sequence(|state| state.optional(|state| self::SYMBOL(state)).and_then(|state| self::StringNormal(state).or_else(|state| state.restore_on_err(|state| self::StringLiteral(state))).or_else(|state| self::StringEmpty(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringLiteral(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringLiteral, |state| state.sequence(|state| self::StringStart(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::StringLiteralText(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::StringEnd(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringNormal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringNormal, |state| state.sequence(|state| self::Quotation(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::StringText(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Quotation(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringEmpty(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringEmpty, |state| state.sequence(|state| self::Quotation(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Quotation(state))).or_else(|state| state.sequence(|state| self::Apostrophe(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Apostrophe(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringLiteralText(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringLiteralText, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Apostrophe(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Apostrophe(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringText(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringText, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Solidus(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Solidus(state).or_else(|state| self::Quotation(state)))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| self::Quotation(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Solidus(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Solidus(state).or_else(|state| self::Quotation(state)))).or_else(|state| state.sequence(|state| state.lookahead(false, |state| self::Quotation(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state)))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringStart(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringStart, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Apostrophe(state).and_then(|state| state.stack_push(|state| state.repeat(|state| self::Apostrophe(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringEnd(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringEnd, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::POP(state).and_then(|state| self::Apostrophe(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::WHITESPACE, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::NEWLINE(state).or_else(|state| self::WHITE_SPACE(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::COMMENT, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::MultiLineComment(state).or_else(|state| self::LineCommentTodo(state)).or_else(|state| self::LineCommentFixme(state)).or_else(|state| self::LineCommentWarning(state)).or_else(|state| self::LineCommentSimple(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LineCommentSimple(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::LineCommentSimple, |state| state.sequence(|state| state.match_string("//").and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LineCommentTodo(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::LineCommentTodo, |state| state.sequence(|state| state.match_string("//?").and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LineCommentFixme(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::LineCommentFixme, |state| state.sequence(|state| state.match_string("//!").and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LineCommentWarning(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::LineCommentWarning, |state| state.sequence(|state| state.match_string("//*").and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MultiLineComment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::MultiLineComment, |state| state.sequence(|state| state.match_string("%%%").and_then(|state| state.repeat(|state| self::MultiLineComment(state).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("%%%")).and_then(|state| self::ANY(state)))))).and_then(|state| state.match_string("%%%")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Symbol(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Symbol, |state| self::namespace(state).or_else(|state| self::SYMBOL(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn namespace(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::namespace, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| state.sequence(|state| self::Proportion(state).and_then(|state| self::SYMBOL(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| self::Proportion(state).and_then(|state| self::SYMBOL(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SYMBOL, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::XID_START(state).and_then(|state| state.repeat(|state| self::XID_CONTINUE(state)))).or_else(|state| state.sequence(|state| self::Underline(state).and_then(|state| self::XID_CONTINUE(state)).and_then(|state| state.repeat(|state| self::XID_CONTINUE(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Modifier(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::Modifier, |state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Set(state).or_else(|state| self::Colon(state)).or_else(|state| self::Semicolon(state)).or_else(|state| state.match_string("{")).or_else(|state| state.match_string("}")).or_else(|state| state.match_string("(")).or_else(|state| state.match_string(")")).or_else(|state| state.match_string("<")).or_else(|state| state.match_string(">"))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Prefix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Prefix, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Bang(state).or_else(|state| self::Plus(state)).or_else(|state| self::Minus(state)).or_else(|state| self::Star(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Suffix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Suffix, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Bang(state).or_else(|state| self::Question(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Infix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Set(state).or_else(|state| self::Plus(state)).or_else(|state| self::Minus(state)).or_else(|state| self::Power(state)).or_else(|state| self::Dot(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Set(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Set, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Or(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Or, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("|")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LazyOr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LazyOr, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("||")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Star(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Star, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("*")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Slash(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Slash, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("/")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Solidus(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Solidus, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\\")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Proportion(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Proportion, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("::").or_else(|state| state.match_string("∷"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Dot(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Dot, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(".")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Comma(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Comma, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(",")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Semicolon(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Semicolon, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(";")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Colon(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Colon, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(":").or_else(|state| state.match_string("："))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Question(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Question, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("?")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Underline(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Underline, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("_")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Load(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Load, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("<<<").or_else(|state| state.match_string("⋘"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Save(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Save, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">>>").or_else(|state| state.match_string("⋙"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LeftShift(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LeftShift, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("<<").or_else(|state| state.match_string("≪"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RightShift(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::RightShift, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">>").or_else(|state| state.match_string("≫"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LessEqual(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LessEqual, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("<=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn GraterEqual(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::GraterEqual, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Equivalent(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Equivalent, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("===")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NotEquivalent(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NotEquivalent, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("=!=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Equal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Equal, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("==")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NotEqual(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NotEqual, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("!=").or_else(|state| state.match_string("≠"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Plus(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Plus, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("+")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Minus(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Minus, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("-")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Power(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Power, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("^")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Surd(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Surd, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("√")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Increase(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Increase, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("++")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Decrease(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Decrease, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("--")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn To(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::To, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("->")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Elvis(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Elvis, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(":?")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Quote(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Quote, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("`")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Acute(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Acute, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("´")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Apostrophe(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Apostrophe, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("'")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Quotation(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Quotation, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\"")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LogicOr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LogicOr, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("||").or_else(|state| state.match_string("∧"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LogicAnd(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LogicAnd, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("&&").or_else(|state| state.match_string("∨"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LogicNot(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LogicNot, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("¬")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Ellipsis(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Ellipsis, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("...").or_else(|state| state.match_string("…"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Concat(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Concat, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("~~")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Destruct(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Destruct, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("~=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Bang(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Bang, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("!")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Sharp(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Sharp, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("#")))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn PEEK(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_peek()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn POP(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_pop()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_NONZERO_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('1'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_BIN_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'1')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_OCT_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'7')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_HEX_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9').or_else(|state| state.match_range('a'..'f')).or_else(|state| state.match_range('A'..'F'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_ALPHA_LOWER(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('a'..'z')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn NEWLINE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\n").or_else(|state| state.match_string("\r\n")).or_else(|state| state.match_string("\r"))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn WHITE_SPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::WHITE_SPACE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_CONTINUE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_CONTINUE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_START(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_START)
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::program => rules::program(state),
            Rule::statement => rules::statement(state),
            Rule::emptyStatement => rules::emptyStatement(state),
            Rule::eos => rules::eos(state),
            Rule::comma_or_semi => rules::comma_or_semi(state),
            Rule::block_or_stmt => rules::block_or_stmt(state),
            Rule::importStatement => rules::importStatement(state),
            Rule::use_alias => rules::use_alias(state),
            Rule::use_module_select => rules::use_module_select(state),
            Rule::module_block => rules::module_block(state),
            Rule::module_tuple => rules::module_tuple(state),
            Rule::ModuleSplit => rules::ModuleSplit(state),
            Rule::controlFlow => rules::controlFlow(state),
            Rule::block => rules::block(state),
            Rule::if_statement => rules::if_statement(state),
            Rule::if_single => rules::if_single(state),
            Rule::if_nested => rules::if_nested(state),
            Rule::if_single_else => rules::if_single_else(state),
            Rule::if_nested_else => rules::if_nested_else(state),
            Rule::else_if_block => rules::else_if_block(state),
            Rule::if_else_block => rules::if_else_block(state),
            Rule::condition => rules::condition(state),
            Rule::for_statement => rules::for_statement(state),
            Rule::for_in_loop => rules::for_in_loop(state),
            Rule::re_control => rules::re_control(state),
            Rule::Return => rules::Return(state),
            Rule::Yield => rules::Yield(state),
            Rule::Break => rules::Break(state),
            Rule::Pass => rules::Pass(state),
            Rule::Type => rules::Type(state),
            Rule::classStatement => rules::classStatement(state),
            Rule::short_block => rules::short_block(state),
            Rule::short_statement => rules::short_statement(state),
            Rule::short_annotation => rules::short_annotation(state),
            Rule::extendStatement => rules::extendStatement(state),
            Rule::assignStatement => rules::assignStatement(state),
            Rule::assign_terms => rules::assign_terms(state),
            Rule::assign_name => rules::assign_name(state),
            Rule::assign_pair => rules::assign_pair(state),
            Rule::defineStatement => rules::defineStatement(state),
            Rule::define_terms => rules::define_terms(state),
            Rule::define_parameter => rules::define_parameter(state),
            Rule::define_pair => rules::define_pair(state),
            Rule::annotation => rules::annotation(state),
            Rule::annotation_call => rules::annotation_call(state),
            Rule::apply => rules::apply(state),
            Rule::apply_kv => rules::apply_kv(state),
            Rule::function_name => rules::function_name(state),
            Rule::function_module => rules::function_module(state),
            Rule::expression => rules::expression(state),
            Rule::expr => rules::expr(state),
            Rule::term => rules::term(state),
            Rule::node => rules::node(state),
            Rule::tuple => rules::tuple(state),
            Rule::bracket_call => rules::bracket_call(state),
            Rule::bracket_apply => rules::bracket_apply(state),
            Rule::dot_call => rules::dot_call(state),
            Rule::template => rules::template(state),
            Rule::EmptyTemplate => rules::EmptyTemplate(state),
            Rule::Fragment => rules::Fragment(state),
            Rule::OpenClose => rules::OpenClose(state),
            Rule::SelfClose => rules::SelfClose(state),
            Rule::text_mode => rules::text_mode(state),
            Rule::text_char => rules::text_char(state),
            Rule::HTMLEscape => rules::HTMLEscape(state),
            Rule::data => rules::data(state),
            Rule::dict => rules::dict(state),
            Rule::list => rules::list(state),
            Rule::slice => rules::slice(state),
            Rule::index => rules::index(state),
            Rule::key_value => rules::key_value(state),
            Rule::key_valid => rules::key_valid(state),
            Rule::index_range => rules::index_range(state),
            Rule::index_step => rules::index_step(state),
            Rule::SpecialValue => rules::SpecialValue(state),
            Rule::Byte => rules::Byte(state),
            Rule::Byte_BIN => rules::Byte_BIN(state),
            Rule::Byte_OCT => rules::Byte_OCT(state),
            Rule::Byte_HEX => rules::Byte_HEX(state),
            Rule::Number => rules::Number(state),
            Rule::Decimal => rules::Decimal(state),
            Rule::DecimalBad => rules::DecimalBad(state),
            Rule::Integer => rules::Integer(state),
            Rule::Complex => rules::Complex(state),
            Rule::String => rules::String(state),
            Rule::StringLiteral => rules::StringLiteral(state),
            Rule::StringNormal => rules::StringNormal(state),
            Rule::StringEmpty => rules::StringEmpty(state),
            Rule::StringLiteralText => rules::StringLiteralText(state),
            Rule::StringText => rules::StringText(state),
            Rule::StringStart => rules::StringStart(state),
            Rule::StringEnd => rules::StringEnd(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
            Rule::COMMENT => rules::COMMENT(state),
            Rule::LineCommentSimple => rules::LineCommentSimple(state),
            Rule::LineCommentTodo => rules::LineCommentTodo(state),
            Rule::LineCommentFixme => rules::LineCommentFixme(state),
            Rule::LineCommentWarning => rules::LineCommentWarning(state),
            Rule::MultiLineComment => rules::MultiLineComment(state),
            Rule::Symbol => rules::Symbol(state),
            Rule::namespace => rules::namespace(state),
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::Modifier => rules::Modifier(state),
            Rule::Prefix => rules::Prefix(state),
            Rule::Suffix => rules::Suffix(state),
            Rule::Infix => rules::Infix(state),
            Rule::Set => rules::Set(state),
            Rule::Or => rules::Or(state),
            Rule::LazyOr => rules::LazyOr(state),
            Rule::Star => rules::Star(state),
            Rule::Slash => rules::Slash(state),
            Rule::Solidus => rules::Solidus(state),
            Rule::Proportion => rules::Proportion(state),
            Rule::Dot => rules::Dot(state),
            Rule::Comma => rules::Comma(state),
            Rule::Semicolon => rules::Semicolon(state),
            Rule::Colon => rules::Colon(state),
            Rule::Question => rules::Question(state),
            Rule::Underline => rules::Underline(state),
            Rule::Load => rules::Load(state),
            Rule::Save => rules::Save(state),
            Rule::LeftShift => rules::LeftShift(state),
            Rule::RightShift => rules::RightShift(state),
            Rule::LessEqual => rules::LessEqual(state),
            Rule::GraterEqual => rules::GraterEqual(state),
            Rule::Equivalent => rules::Equivalent(state),
            Rule::NotEquivalent => rules::NotEquivalent(state),
            Rule::Equal => rules::Equal(state),
            Rule::NotEqual => rules::NotEqual(state),
            Rule::Plus => rules::Plus(state),
            Rule::Minus => rules::Minus(state),
            Rule::Power => rules::Power(state),
            Rule::Surd => rules::Surd(state),
            Rule::Increase => rules::Increase(state),
            Rule::Decrease => rules::Decrease(state),
            Rule::To => rules::To(state),
            Rule::Elvis => rules::Elvis(state),
            Rule::Quote => rules::Quote(state),
            Rule::Acute => rules::Acute(state),
            Rule::Apostrophe => rules::Apostrophe(state),
            Rule::Quotation => rules::Quotation(state),
            Rule::LogicOr => rules::LogicOr(state),
            Rule::LogicAnd => rules::LogicAnd(state),
            Rule::LogicNot => rules::LogicNot(state),
            Rule::Ellipsis => rules::Ellipsis(state),
            Rule::Concat => rules::Concat(state),
            Rule::Destruct => rules::Destruct(state),
            Rule::Bang => rules::Bang(state),
            Rule::Sharp => rules::Sharp(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
