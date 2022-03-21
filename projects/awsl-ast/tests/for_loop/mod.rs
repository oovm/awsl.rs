use super::*;

macro_rules! run_test {
    ($($F:ident), +,) => {
        $(run_test![$F, stringify!($F)];)+
    };
    ($function_name:ident, $file_name:expr) => {
    #[test]
    fn $function_name() {
        let out = render(include_str!(concat!($file_name, ".sdl"))).unwrap();
        assert_eq!(out, include_str!(concat!($file_name, ".out.sdl")))
    }
    };
}

run_test![
    for_i_in_list,
    for_i_in_string,
    for_if_guard,
    for_else_guard,
    for_if_else_guard,
];

#[test]
fn error_for_in_decimal() {
    let e = render("for i in 0.0 {}").unwrap_err();
    assert_eq!(e.to_string(), "IteratorError: Type `Decimal` is not an iterable element
--> 1:10")
}