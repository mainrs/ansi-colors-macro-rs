use ansi_colors_macro::ansi_string;

#[test]
fn test_red_string() {
    assert_eq!("\u{1b}[31mred\u{1b}[0m", ansi_string!("{red red}"));
}
