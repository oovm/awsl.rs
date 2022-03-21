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
    index_of_list,
    index_of_string,
    index_of_nested_list,
];

#[test]
fn error_index_of_null() {
    let e = render("null.1").unwrap_err();
    assert_eq!(e.to_string(), "IndexError: Unable to get index 1 on type `Null`
--> 1:5")
}

#[test]
fn error_index_of_decimal() {
    let e = render("1.0.1").unwrap_err();
    assert_eq!(e.to_string(), "IndexError: Unable to get index 1 on type `Decimal`
--> 1:4")
}
