use image::{Rgba, RgbaImage};
use imageproc::{
    drawing::{draw_filled_rect_mut, draw_line_segment_mut, draw_text_mut},
    rect::Rect,
};
use rusttype::{Font, Scale};
use screenshots::Screen;
use std::error::Error;

pub fn get_screen_size() -> Result<(u32, u32), &'static str> {
    match Screen::all() {
        Ok(screens) => match screens.get(0) {
            Some(screen) => {
                let width = screen.display_info.width;
                let height = screen.display_info.height;
                Ok((width, height))
            }
            None => Err("No screens found"),
        },
        Err(_) => Err("Failed to get screen information"),
    }
}

pub fn capture_screen_with_cursor(file_path: &str) -> Result<(), String> {
    let screens = Screen::all().map_err(|e| format!("Failed to get screens: {}", e))?;

    let screen = screens.get(0).ok_or("No screens found")?;
    let screenshot = screen
        .capture()
        .map_err(|e| format!("Failed to capture screen: {}", e))?;

    screenshot
        .save(file_path)
        .map_err(|e| format!("Failed to save screenshot: {}", e))
}

pub fn add_grid_to_image(
    original_image_path: &str,
    new_image_path: &str,
    grid_interval: i32,
) -> Result<(), Box<dyn Error>> {
    let image = image::open(original_image_path)?;

    let image = image.into_rgba8();
    let (width, height) = image.dimensions();
    let mut draw = image;

    let font_size = (grid_interval / 10) as u32;
    let bg_width = (font_size as f32 * 6.5) as u32;
    let bg_height = (font_size as f32 * 1.2) as u32;

    // Draw vertical lines and labels
    for x in (grid_interval as u32..width).step_by(grid_interval as usize) {
        for y in (grid_interval as u32..height).step_by(grid_interval as usize) {
            draw_line_segment_mut(
                &mut draw,
                (x as f32, 0.0),
                (x as f32, height as f32),
                Rgba([0, 0, 255, 255]),
            );

            let x_percent = ((x as f32 / width as f32) * 100.0).round() as u32;
            let y_percent = ((y as f32 / height as f32) * 100.0).round() as u32;
            draw_label_with_background(
                (x - bg_width / 2, y - bg_height / 2),
                &format!("X={}%,Y={}%", x_percent, y_percent),
                &mut draw,
                bg_width,
                bg_height,
                font_size,
            )?;
        }
    }

    // Draw horizontal lines
    for y in (grid_interval as u32..height).step_by(grid_interval as usize) {
        draw_line_segment_mut(
            &mut draw,
            (0.0, y as f32),
            (width as f32, y as f32),
            Rgba([0, 0, 255, 255]),
        );
    }

    // Save the image
    draw.save(new_image_path)?;

    Ok(())
}

fn draw_label_with_background(
    position: (u32, u32),
    text: &str,
    draw: &mut RgbaImage,
    bg_width: u32,
    bg_height: u32,
    font_size: u32,
) -> Result<(), String> {
    let scale = Scale {
        x: font_size as f32,
        y: font_size as f32,
    };

    let font_data = include_bytes!("font/DejaVuSans.ttf") as &[u8];
    let font =
        Font::try_from_vec(font_data.to_vec()).ok_or_else(|| "Failed to load font".to_string())?;

    let rect = Rect::at(position.0 as i32, position.1 as i32).of_size(bg_width, bg_height);
    draw_filled_rect_mut(draw, rect, Rgba([255, 255, 255, 255]));

    let x_position = position.0;
    let y_position = position.1 + bg_height / 8;
    draw_text_mut(
        draw,
        Rgba([0, 0, 0, 255]),
        x_position as i32,
        y_position as i32,
        scale,
        &font,
        text,
    );

    Ok(())
}
