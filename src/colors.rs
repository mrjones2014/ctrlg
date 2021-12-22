use ansi_term::Color;
use std::num::ParseIntError;

pub fn parse_rgb_triple(input: &str) -> Option<(u8, u8, u8)> {
    let values = input
        .split(',')
        .map(|value| value.trim())
        .collect::<Vec<&str>>();
    if values.len() != 3 {
        return None;
    }

    let values: Result<Vec<u8>, ParseIntError> =
        values.iter().map(|value| value.parse::<u8>()).collect();

    if let Ok(values) = values {
        return Some((values[0], values[1], values[2]));
    }

    None
}

pub fn parse_color(input: &str) -> Color {
    match input.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "purple" => Color::Purple,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        input => {
            // check for an integer-specified xterm-256 color
            // see: https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg
            let is_xterm_256_color = input.parse::<u8>();
            if let Ok(color_int) = is_xterm_256_color {
                Color::Fixed(color_int)
            } else if let Some(rgb_triple) = parse_rgb_triple(input) {
                Color::RGB(rgb_triple.0, rgb_triple.1, rgb_triple.2)
            } else {
                eprintln!("Invalid color definition found in config file: '{}'", input);
                Color::White
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ansi_term::Color;

    macro_rules! color_test {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    let output = parse_color(input);
                    assert_eq!(expected, output);
                }
            )*
        }
    }

    color_test! {
        black: ("black", Color::Black),
        red: ("red", Color::Red),
        green: ("green", Color::Green),
        yellow: ("yellow", Color::Yellow),
        blue: ("blue", Color::Blue),
        purple: ("purple", Color::Purple),
        cyan: ("cyan", Color::Cyan),
        white: ("white", Color::White),
        invalid_named_color: ("invalid named color", Color::White),
        fixed_color: ("255", Color::Fixed(255)),
        rgb: ("255,255,255", Color::RGB(255, 255, 255)),
        decimal_fixed: ("17.25", Color::White), // decimal fixed colors are not valid
        decimal_rgb: ("17.25, 17.25, 17.25", Color::White), // decimal RGB colors are not valid
    }
}
