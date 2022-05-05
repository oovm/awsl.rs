// mod arithmetic;
// mod call_chain;
// mod declare;
// mod for_loop;
mod literal;
// mod if_condition;
//
// use sdl_ast::SDLContext;
use awsl_ast::{ParserConfig, Result};

#[test]
fn ready() {
    println!("it works!")
}

pub fn render(input: &str) -> Result<String> {
    let mut parser = ParserConfig::default();
    let out = parser.parse(input)?;
    let mut ctx = SDLContext::default();
    let out = ctx.evaluate(&out)?;
    let out = ctx.render(&out)?;
    Ok(out)
}
//
// pub fn render_steps(input: &str) -> Result<String> {
//     let mut parser = ParserConfig::default();
//     let out = parser.parse(input)?;
//     println!("{:#?}", out);
//     let mut ctx = SDLContext::default();
//     let out = ctx.evaluate(&out)?;
//     println!("{:?}", out);
//     let out = ctx.render(&out)?;
//     println!("{:?}", out);
//     Ok(out)
// }