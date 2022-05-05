use super::*;

#[test]
fn literal_value() {
    let out = render(include_str!("value.awsl")).unwrap();
    assert_eq!(out, include_str!("value.out.awsl"));
    let out = render(include_str!("value.awsl")).unwrap();
    assert_eq!(out, include_str!("value.out.awsl"));
}
