use ansi_colors_macro::ansi_string;

#[test]
fn test_plain_string() {
    assert_eq!("escape $", ansi_string!("escape $$"));
}
