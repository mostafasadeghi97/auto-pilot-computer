use crate::{parsers::convert_percent_to_decimal, screen::get_screen_size};
use enigo::{Enigo, Key, KeyboardControllable, MouseButton, MouseControllable};

pub fn keyboard_type(text: &str) -> String {
    let mut enigo = Enigo::new();
    for c in text.chars() {
        match c {
            '/' => {
                enigo.key_sequence("/");
            }
            _ => {
                enigo.key_click(Key::Layout(c));
            }
        }
    }

    enigo.key_down(Key::Return);

    format!("Type: {}", text)
}

pub fn search(text: &str) -> String {
    let mut enigo = Enigo::new();
    // open the search in MacOS
    enigo.key_down(Key::Meta);
    enigo.key_click(Key::Layout(' '));
    enigo.key_up(Key::Meta);

    std::thread::sleep(std::time::Duration::from_secs(1));

    for c in text.chars() {
        enigo.key_click(Key::Layout(c));
    }
    enigo.key_down(Key::Return);
    return format!("Open program: {}", text);
}

pub fn click_at_percentage(x_percentage: &str, y_percentage: &str) -> String {
    let x_decimal = match convert_percent_to_decimal(x_percentage) {
        Ok(x_decimal) => x_decimal,
        Err(_) => 0.0,
    };

    let y_decimal = match convert_percent_to_decimal(y_percentage) {
        Ok(y_decimal) => y_decimal,
        Err(_) => 0.0,
    };

    let (screen_width, screen_height) = match get_screen_size() {
        Ok((screen_width, screen_height)) => (screen_width, screen_height),
        Err(_) => (0, 0),
    };

    let x_pixel = (x_decimal * screen_width as f32).round() as i32;
    let y_pixel = (y_decimal * screen_height as f32).round() as i32;

    let mut enigo = Enigo::new();
    enigo.mouse_move_to(x_pixel, y_pixel);
    enigo.mouse_click(MouseButton::Left);

    format!("Click: x: {}, y: {}", x_pixel, y_pixel)
}

pub fn mouse_click(click_detail: &serde_json::Value) -> String {
    match (click_detail["x"].as_str(), click_detail["y"].as_str()) {
        (Some(x), Some(y)) if !x.is_empty() && !y.is_empty() => {
            click_at_percentage(x, y);
            format!(
                "Click: x: {}, y: {}, description: {}, reason: {}",
                x, y, click_detail["description"], click_detail["reason"]
            )
        }
        _ => "We failed to click".to_string(),
    }
}
