use proc_macro::{Literal, TokenStream, TokenTree};

const STYLE_RESET_TOKEN: &str = "\u{001B}[0m";

/// Color string literals using ANSI colors.
///
/// ```rust
/// use ansi_colors_macro::ansi_string;
///
/// const DONE_MESSAGE: &str = ansi_string!("{green Done}");
/// ```
///
/// # Argument
///
/// A string literal.
#[proc_macro]
pub fn ansi_string(args: TokenStream) -> TokenStream {
    // Only accept string literals. Other input is invalid.
    let mut args: Vec<TokenTree> = args.into_iter().collect();
    if let Some(literal) = args.pop() {
        match literal {
            TokenTree::Literal(literal) => {
                let token: String = literal.to_string();

                match index(&token, 0) {
                    b'"' => {
                        // Ignore the first and last position, as those are ".
                        let mut current_index = 1;
                        let end_index = token.len() - 2;

                        // The resulting string that includes ANSI escape codes.
                        let mut result = Vec::with_capacity(token.len() - 2);

                        while current_index <= end_index {
                            let current_byte = index(&token, current_index);
                            match current_byte {
                                b'{' => {
                                    // The current character starts a color sequence. The next whole word will be the color to render the rest of the content.
                                    let mut color = Vec::with_capacity(10);
                                    let mut temp_index = current_index + 1;

                                    // Just a safety measure to ensure that the loop ends at some point.
                                    while temp_index < end_index {
                                        let next_char = index(&token, temp_index);
                                        if next_char == b' ' {
                                            current_index = current_index + 1;
                                            break;
                                        }

                                        color.push(next_char);
                                        current_index = current_index + 1;
                                        temp_index = temp_index + 1;
                                    }

                                    let color_name =
                                        unsafe { std::str::from_utf8_unchecked(color.as_ref()) };
                                    if let Some(color) = get_ansi_code_for_color_string(color_name)
                                    {
                                        color.as_bytes().into_iter().for_each(|b| result.push(*b));
                                    }
                                }
                                b'$' => {
                                    // The escape key \ has been found. Check the next character to determine the behavior.
                                    match index(&token, current_index + 1) {
                                        b'$' => result.push(b'$'),
                                        b'{' => result.push(b'{'),
                                        _ => panic!("{}", "Found escaping character $ without being followed by either $ or {")
                                    }
                                    current_index = current_index + 1;
                                }
                                b'}' => {
                                    // Color content closed. Push the escape key to reset styles onto the string.
                                    STYLE_RESET_TOKEN
                                        .as_bytes()
                                        .iter()
                                        .for_each(|c| result.push(*c));
                                }
                                _ => result.push(current_byte),
                            }

                            current_index = current_index + 1;
                        }

                        // Construct a new literal from the resulting bytes.
                        // Safety: All bytes passed into result are UTF-8 valid.
                        let literal_to_include =
                            Literal::string(unsafe { std::str::from_utf8_unchecked(&result) });
                        return TokenTree::Literal(literal_to_include).into();
                    }
                    _ => panic_only_string(),
                }
            }
            _ => panic_only_string(),
        }
    } else {
        panic_only_string();
    }

    panic!();
}

fn index(s: impl AsRef<[u8]>, index: usize) -> u8 {
    let s = s.as_ref();
    if index < s.len() {
        s[index]
    } else {
        0
    }
}

fn get_ansi_code_for_color_string(s: &str) -> Option<String> {
    // Multiple styles are separated by a dot.
    let fragments = s.split(".");
    let mut result = String::from("\u{001B}[");

    let mut last_loop_set = false;
    for (index, fragment) in fragments.enumerate() {
        if last_loop_set {
            result.push(';');
            last_loop_set = false;
        }

        let number_code = match fragment {
            "bold" => "1",
            "dim" => "2",
            "italic" => "3",
            "underline" => "4",
            "inverse" => "7",
            "hidden" => "8",
            "strikethrough" => "9",
            "black" => "30",
            "red" => "31",
            "green" => "32",
            "yellow" => "33",
            "blue" => "34",
            "magenta" => "35",
            "cyan" => "36",
            "white" => "37",
            "gray" => "90",
            "bright_red" => "91",
            "bright_green" => "92",
            "bright_yellow" => "93",
            "bright_blue" => "94",
            "bright_magenta" => "95",
            "bright_cyan" => "96",
            "bright_white" => "97",
            "bg_black" => "40",
            "bg_red" => "41",
            "bg_green" => "42",
            "bg_yellow" => "43",
            "bg_blue" => "44",
            "bg_magenta" => "45",
            "bg_cyan" => "46",
            "bg_white" => "47",
            "bg_gray" => "100",
            "bg_bright_red" => "101",
            "bg_bright_green" => "102",
            "bg_bright_yellow" => "103",
            "bg_bright_blue" => "104",
            "bg_bright_magenta" => "105",
            "bg_bright_cyan" => "106",
            "bg_bright_white" => "107",

            _ => panic!("Unknown style type: {}", fragment),
        };

        result.push_str(number_code);
        last_loop_set = true;
    }

    result.push('m');
    Some(result)
}

fn panic_only_string() {
    panic!("ansi_string requires a string literal as its only argument");
}
