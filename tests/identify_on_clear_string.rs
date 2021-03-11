use ansi_colors_macro::ansi_string;

#[test]
fn test_plain_string() {
    assert_eq!("Clear string", ansi_string!("Clear string"));
}
