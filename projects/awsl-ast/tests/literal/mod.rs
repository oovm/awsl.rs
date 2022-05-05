use super::*;

macro_rules! run_test {
    ($($F:ident), +,) => {
        $(run_test![$F, stringify!($F)];)+
    };
    ($function_name:ident, $file_name:expr) => {
    #[test]
    fn $function_name() {
        let out = render(include_str!(concat!($file_name, ".awsl"))).unwrap();
        assert_eq!(out, include_str!(concat!($file_name, ".out.awsl")))
    }
    };
}

run_test![
    value,
    // comment,
    // number,
    // string, string_escape,
    // template_escape,
];
