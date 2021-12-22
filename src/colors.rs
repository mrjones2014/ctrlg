use ansi_term::Color;
use std::num::ParseIntError;

pub fn parse_rgb_triple(input: &String) -> Option<(u8, u8, u8)> {
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

pub fn parse_color(input: &String, default_color: Color) -> Color {
    match input.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "purple" => Color::Purple,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        _ => {
            // check for an integer-specified xterm-256 color
            // see: https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg
            let is_xterm_256_color = input.parse::<u8>();
            if let Ok(color_int) = is_xterm_256_color {
                Color::Fixed(color_int)
            } else if let Some(rgb_triple) = parse_rgb_triple(input) {
                Color::RGB(rgb_triple.0, rgb_triple.1, rgb_triple.2)
            } else {
                default_color
            }
        }
    }
}
