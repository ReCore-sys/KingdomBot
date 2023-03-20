use noise::{Clamp, NoiseFn, OpenSimplex};
use raster::{Color, Image};

use crate::config::get_config;
use crate::map;

const TILE_SIZE: i32 = 250;
const BORDER_SIZE: i32 = 10;
pub const VIEW_DISTANCE: i32 = 20;
const PIXEL_CLUMPING: i32 = 80;

pub async fn draw_map(grid: &Vec<Vec<map::Tile>>) -> Image {
    // draw a 2d grid of tiles based off the vector of vectors given
    // draw each tile as TILE_SIZE * TILE_SIZE pixels with a BORDER_SIZE pixel border
    // if the tile is occupied, draw the border as red, otherwise draw it as black
    // Each pixel has to be drawn individually, so this is going to be a bit of a pain
    // The size of the image is based off the size of the grid

    let mut image = Image::blank(
        (grid.len() as i32 * (TILE_SIZE + BORDER_SIZE)) + BORDER_SIZE,
        (grid[0].len() as i32 * (TILE_SIZE + BORDER_SIZE)) + BORDER_SIZE,
    );
    let perlin = OpenSimplex::new(get_config().perlin_seed); // Noise so we can texture the tiles

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

                    image
                        .set_pixel(pixel_x, pixel_y, Color::rgba(175, 255, 175, alpha))
                        .expect("Failed to set pixel")
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
                Color::rgb(255, 0, 0)
            } else {
                Color::rgb(0, 0, 0)
            };
            // Draw the top  and bottom border
            for border_x in 0..TILE_SIZE + BORDER_SIZE {
                for border_y in 0..BORDER_SIZE {
                    image
                        .set_pixel(
                            (x * (TILE_SIZE + BORDER_SIZE)) + border_x + BORDER_SIZE,
                            (y * (TILE_SIZE + BORDER_SIZE)) + border_y + BORDER_SIZE,
                            color.clone(),
                        )
                        .expect("Failed to set pixel");
                    image
                        .set_pixel(
                            (x * (TILE_SIZE + BORDER_SIZE)) + border_x + BORDER_SIZE,
                            (y * (TILE_SIZE + BORDER_SIZE)) + border_y + TILE_SIZE + BORDER_SIZE,
                            color.clone(),
                        )
                        .expect("Failed to set pixel");
                }
            }

            // Draw the side borders
            for border_x in 0..BORDER_SIZE {
                for border_y in 0..TILE_SIZE {
                    image
                        .set_pixel(
                            (x * (TILE_SIZE + BORDER_SIZE)) + border_x + BORDER_SIZE,
                            (y * (TILE_SIZE + BORDER_SIZE)) + border_y + BORDER_SIZE,
                            color.clone(),
                        )
                        .expect("Failed to set pixel");
                    image
                        .set_pixel(
                            (x * (TILE_SIZE + BORDER_SIZE)) + border_x + TILE_SIZE + BORDER_SIZE,
                            (y * (TILE_SIZE + BORDER_SIZE)) + border_y + BORDER_SIZE,
                            color.clone(),
                        )
                        .expect("Failed to set pixel");
                }
            }
        }
    }
    image
}