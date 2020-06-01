#[macro_use]
extern crate quote;
extern crate pest_generator;
extern crate proc_macro;

mod pest;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
#[ignore]
fn gen_parser() {
    pest::gen_note_down();
}
