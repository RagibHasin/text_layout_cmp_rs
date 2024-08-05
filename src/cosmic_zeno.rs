// SPDX-License-Identifier: MIT OR Apache-2.0

//! Run this example with `cargo run --package terminal`
//! or `cargo run --package terminal -- "my own text"`

use std::fs::File;

use cosmic_text::{Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, SwashCache};
use image::{codecs::png::PngEncoder, Pixel, Rgba, RgbaImage};

pub fn main(text: &str, dark: bool) {
    let text_color;
    let bg_color;
    let suffix;
    if dark {
        text_color = Color::rgb(255, 255, 255);
        bg_color = Rgba(0x000000ffu32.to_be_bytes());
        suffix = "_dark";
    } else {
        text_color = Color::rgb(0, 0, 0);
        bg_color = Rgba(0xffffffffu32.to_be_bytes());
        suffix = "";
    }
    let output_path = format!(
        "output/{}{suffix}.png",
        file!()[4..].trim_end_matches(".rs"),
    );

    // A FontSystem provides access to detected system fonts, create one per application
    let mut font_system = FontSystem::new();
    font_system
        .db_mut()
        .load_font_file(r"assets\iosevka-ahad-regular.ttf")
        .unwrap();

    // A SwashCache stores rasterized glyphs, create one per application
    let mut swash_cache = SwashCache::new();

    // Text metrics indicate the font size and line height of a buffer
    let metrics = Metrics::relative(24.0, 1.3);

    // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
    let mut buffer = Buffer::new(&mut font_system, metrics);
    let mut buffer = buffer.borrow_with(&mut font_system);

    // Set a size for the text buffer, in pixels
    // let width = 320;
    // The height is unbounded
    // buffer.set_size(Some(width as f32), None);

    // Attributes indicate what font to choose
    let attrs = Attrs::new().family(Family::Name("Iosevka Ahad"));

    // Add some text!
    buffer.set_text(text, attrs, Shaping::Advanced);

    // Perform shaping as desired
    buffer.shape_until_scroll(true);

    // Padding around the output image
    let padding = 20;

    // Set up the canvas
    let height = (buffer.metrics().line_height * buffer.layout_runs().count() as f32).ceil() as u32
        + (padding * 2);
    let width = buffer
        .layout_runs()
        .map(|r| r.line_w)
        .max_by(f32::total_cmp)
        .unwrap() as u32
        + (padding * 2);
    let mut img = RgbaImage::from_pixel(width, height, bg_color);

    // Draw to the canvas
    buffer.draw(&mut swash_cache, text_color, |x, y, w, h, color| {
        let x = (x + padding as i32) as u32;
        let y = (y + padding as i32) as u32;
        if color.a() == 0 || x >= width || y >= height || w != 1 || h != 1 {
            // Ignore alphas of 0, or invalid x, y coordinates, or unimplemented sizes
            return;
        }

        img.get_pixel_mut(x, y).blend(&Rgba(color.as_rgba()));
    });

    let output_file = File::create(output_path).unwrap();
    let png_encoder = PngEncoder::new(output_file);
    img.write_with_encoder(png_encoder).unwrap();
}
