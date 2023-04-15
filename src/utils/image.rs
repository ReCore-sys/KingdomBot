use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, Canvas};
use noise::{Clamp, NoiseFn, OpenSimplex};
use rusttype::{Font, Scale};

use crate::config::get_config;
use crate::types;

const TILE_SIZE: i32 = 150;
const BORDER_SIZE: i32 = 5;
pub const VIEW_DISTANCE: i32 = 10;
const PIXEL_CLUMPING: i32 = 35;

const TEXT_SCALE: f32 = 75.0;
const LETTER_WIDTH: i32 = 30;

pub async fn draw_map(
    grid: &Vec<Vec<types::map::Tile>>,
    faction: String,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    // draw a 2d grid of tiles based off the vector of vectors given
    // draw each tile as TILE_SIZE * TILE_SIZE pixels with a BORDER_SIZE pixel border
    // if the tile is occupied, draw the border as red, otherwise draw it as black
    // Each pixel has to be drawn individually, so this is going to be a bit of a pain
    // The size of the image is based off the size of the grid

    // TODO: Optimise this to use imageproc stuff and also only take 1 pass

    let mut image = RgbImage::new(
        ((grid.len() as i32 * (TILE_SIZE + BORDER_SIZE)) + BORDER_SIZE) as u32,
        ((grid[0].len() as i32 * (TILE_SIZE + BORDER_SIZE)) + BORDER_SIZE) as u32,
    );
    let perlin = OpenSimplex::new(get_config().perlin_seed); // Noise so we can texture the tiles

    let scale = Scale {
        x: TEXT_SCALE,
        y: TEXT_SCALE,
    };
    let font = Vec::from(include_bytes!("../font.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let clamp = Clamp::new(perlin).set_bounds(0.1, 0.4);
    for x in 1..grid.len() as i32 {
        for y in 0..(grid[x as usize].len() - 1) as i32 {
            // Draw the tile first
            for tile_x in 0..TILE_SIZE {
                for tile_y in 0..TILE_SIZE {
                    let pixel_x = (x * (TILE_SIZE + BORDER_SIZE)) + tile_x + BORDER_SIZE;
                    let pixel_y = (y * (TILE_SIZE + BORDER_SIZE)) + tile_y + BORDER_SIZE;
                    let rounded_pixel_x = PIXEL_CLUMPING * (pixel_x / PIXEL_CLUMPING);
                    let rounded_pixel_y = PIXEL_CLUMPING * (pixel_y / PIXEL_CLUMPING);
                    let value = clamp.get([rounded_pixel_x as f64, rounded_pixel_y as f64]) + 0.3;
                    // We round each pixel to the nearest PIXEL_CLUMPING pixels to make the image
                    // This way groups of pixels will be the same color, making the image look more
                    // like a texture and less like random noise
                    let background_color = hsv_to_rgb(120.0, 31.4, value as f32);
                    image.draw_pixel(
                        pixel_x as u32,
                        pixel_y as u32,
                        Rgb([background_color.0, background_color.1, background_color.2]),
                    );
                }
            }
        }
    }
    // now loop through the grid again and draw the borders
    // if the tile is occupied, draw the border as red, otherwise draw it as black
    // as long as we are at least 2 border widths away from the edge of the image, draw borders
    // twice as thick
    for x in 0..grid.len() as i32 {
        for y in 0..grid[x as usize].len() as i32 {
            let color = if grid[x as usize][y as usize].occupied {
                if faction == grid[x as usize][y as usize].faction {
                    Rgb([135, 206, 235])
                } else {
                    Rgb([255, 0, 0])
                }
            } else {
                Rgb([0, 0, 0])
            };
            // Draw the top  and bottom border
            for border_x in 0..TILE_SIZE + BORDER_SIZE {
                for border_y in 0..BORDER_SIZE {
                    image.draw_pixel(
                        ((x * (TILE_SIZE + BORDER_SIZE)) + border_x + BORDER_SIZE) as u32,
                        ((y * (TILE_SIZE + BORDER_SIZE)) + border_y + BORDER_SIZE) as u32,
                        color,
                    );
                    image.draw_pixel(
                        ((x * (TILE_SIZE + BORDER_SIZE)) + border_x + BORDER_SIZE) as u32,
                        ((y * (TILE_SIZE + BORDER_SIZE)) + border_y + TILE_SIZE + BORDER_SIZE)
                            as u32,
                        color,
                    );
                }
            }

            // Draw the side borders
            for border_x in 0..BORDER_SIZE {
                for border_y in 0..TILE_SIZE {
                    image.draw_pixel(
                        ((x * (TILE_SIZE + BORDER_SIZE)) + border_x + BORDER_SIZE) as u32,
                        ((y * (TILE_SIZE + BORDER_SIZE)) + border_y + BORDER_SIZE) as u32,
                        color,
                    );
                    image.draw_pixel(
                        ((x * (TILE_SIZE + BORDER_SIZE)) + border_x + TILE_SIZE + BORDER_SIZE)
                            as u32,
                        ((y * (TILE_SIZE + BORDER_SIZE)) + border_y + BORDER_SIZE) as u32,
                        color,
                    );
                }
            }
        }
    }
    // Now draw the text
    // the text is drawn in the center of the tile and only on the first column and last row
    // of the grid
    let text_color = Rgb([255, 255, 255]);
    for x_counter in 1..grid.len() {
        let text = format!("{}", grid[x_counter][0].x);
        let text_width = text.len() as f32 * (LETTER_WIDTH as f32);

        draw_text_mut(
            &mut image,
            text_color,
            ((TILE_SIZE + BORDER_SIZE) * x_counter as i32) + (TILE_SIZE / 2)
                - (text_width as i32 / 2),
            (grid.len() as i32) * TILE_SIZE - (BORDER_SIZE * grid.len() as i32),
            scale,
            &font,
            text.as_str(),
        );
    }
    for y_counter in 0..grid[0].len() - 1 {
        let text = format!("{}", grid[0][y_counter].y);
        let text_width = text.len() as f32 * (LETTER_WIDTH as f32);
        draw_text_mut(
            &mut image,
            text_color,
            TILE_SIZE / 2 - (text_width as i32 / 2),
            ((TILE_SIZE + BORDER_SIZE) * y_counter as i32) + (BORDER_SIZE * grid.len() as i32),
            scale,
            &font,
            text.as_str(),
        );
    }

    image
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    // Convert the HSV values to RGB
    // The h parameter should be in the range 0-360
    // The s parameter should be in the range 0-100
    // The v parameter should be in the range 0-1
    // All output values should be in the range 0-255

    // convert s to 0-1
    let c = v * (s / 100.0);
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = (r + m) * 255.0;
    let g = (g + m) * 255.0;
    let b = (b + m) * 255.0;

    (r as u8, g as u8, b as u8)
}