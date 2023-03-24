use std::collections::HashMap;

use image::{ImageBuffer, Rgb, RgbImage, Rgba, RgbaImage};
use imageproc::drawing::{draw_text, draw_text_mut, Canvas};
use noise::{Clamp, NoiseFn, OpenSimplex};
use rusttype::{Font, Scale};

use crate::config::get_config;
use crate::map;

const TILE_SIZE: i32 = 150;
const BORDER_SIZE: i32 = 10;
pub const VIEW_DISTANCE: i32 = 10;
const PIXEL_CLUMPING: i32 = 30;

const TEXT_SCALE: f32 = 40.0;

pub async fn draw_map(grid: &Vec<Vec<map::Tile>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // draw a 2d grid of tiles based off the vector of vectors given
    // draw each tile as TILE_SIZE * TILE_SIZE pixels with a BORDER_SIZE pixel border
    // if the tile is occupied, draw the border as red, otherwise draw it as black
    // Each pixel has to be drawn individually, so this is going to be a bit of a pain
    // The size of the image is based off the size of the grid

    let mut image = RgbaImage::new(
        ((grid.len() as i32 * (TILE_SIZE + BORDER_SIZE)) + BORDER_SIZE) as u32,
        ((grid[0].len() as i32 * (TILE_SIZE + BORDER_SIZE)) + BORDER_SIZE) as u32,
    );
    let perlin = OpenSimplex::new(get_config().perlin_seed); // Noise so we can texture the tiles

    let scale = Scale {
        x: TEXT_SCALE,
        y: TEXT_SCALE,
    };
    let font = Vec::from(include_bytes!("font.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let clamp = Clamp::new(perlin).set_bounds(0.0, 1.0);
    for x in 0..grid.len() as i32 {
        for y in 0..grid[x as usize].len() as i32 {
            // Draw the tile first
            for tile_x in 0..TILE_SIZE {
                for tile_y in 0..TILE_SIZE {
                    let pixel_x = (x * (TILE_SIZE + BORDER_SIZE)) + tile_x + BORDER_SIZE;
                    let pixel_y = (y * (TILE_SIZE + BORDER_SIZE)) + tile_y + BORDER_SIZE;
                    let rounded_pixel_x = PIXEL_CLUMPING * (pixel_x / PIXEL_CLUMPING);
                    let rounded_pixel_y = PIXEL_CLUMPING * (pixel_y / PIXEL_CLUMPING);
                    let alpha_initial = clamp.get([rounded_pixel_x as f64, rounded_pixel_y as f64]);
                    let alpha = (alpha_initial * 100.0) as u8 + 155;
                    // We round each pixel to the nearest PIXEL_CLUMPING pixels to make the image
                    // This way groups of pixels will be the same color, making the image look more
                    // like a texture and less like random noise

                    image.draw_pixel(pixel_x as u32, pixel_y as u32, Rgba([175, 255, 175, alpha]));
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
                Rgba([255, 0, 0, 255])
            } else {
                Rgba([0, 0, 0, 255])
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

    // now we need to figure out what to display as the axis labels
    let mut min_x = 10 ^ 10;
    let mut max_x = 0;
    let mut min_y = 10 ^ 10;
    let mut max_y = 0;
    for row in grid {
        for tile in row {
            if tile.x < min_x {
                min_x = tile.x;
            }
            if tile.x > max_x {
                max_x = tile.x;
            }
            if tile.y < min_y {
                min_y = tile.y;
            }
            if tile.y > max_y {
                max_y = tile.y;
            }
        }
    }
    // now go from the min to the max and draw the labels on the relevant sides
    let mut tile_counter = 0;
    loop {
        if tile_counter == VIEW_DISTANCE {
            break;
        }
        let x = min_x + tile_counter;
        let y = min_y - 1;
        let pixel_x = (x * (TILE_SIZE + BORDER_SIZE)) + TILE_SIZE / 2 + BORDER_SIZE;
        let pixel_y = (y * (TILE_SIZE + BORDER_SIZE)) + TILE_SIZE / 2 + BORDER_SIZE;
        let text = format!("{}", x);

        image = draw_text(
            &mut image,
            Rgba([0, 0, 0, 255]),
            pixel_x,
            pixel_y,
            scale,
            &font,
            &text,
        );
        tile_counter += 1;
    }
    tile_counter = 0;
    loop {
        if min_y + tile_counter > max_y {
            break;
        }
        let x = min_x - 1;
        let y = min_y + tile_counter;
        let pixel_x = (x * (TILE_SIZE + BORDER_SIZE)) + TILE_SIZE / 2 + BORDER_SIZE;
        let pixel_y = (y * (TILE_SIZE + BORDER_SIZE)) + TILE_SIZE / 2 + BORDER_SIZE;
        let text = format!("{}", y);

        draw_text_mut(
            &mut image,
            Rgba([0, 0, 0, 255]),
            pixel_x,
            pixel_y,
            scale,
            &font,
            &text,
        );
        tile_counter += 1;
    }
    image
}
