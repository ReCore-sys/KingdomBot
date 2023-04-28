use std::thread;

use atomic_counter::{AtomicCounter, RelaxedCounter};
use dashmap::DashMap;
use image::{GenericImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;
use noise::{Clamp, NoiseFn, OpenSimplex};

use crate::config::get_config;
use crate::types;

const TILE_SIZE: i32 = 100;
const BORDER_SIZE: i32 = 5;
const INSET_SIZE: i32 = 5;
const IN_TILE_SIZE: i32 = TILE_SIZE - (INSET_SIZE * 2);
pub const VIEW_DISTANCE: i32 = 10;

// const TEXT_SCALE: f32 = 75.0;
// const LETTER_WIDTH: i32 = 30; todo

const C_1_CENTER: (i32, i32) = (
    (INSET_SIZE + (IN_TILE_SIZE / 4)),
    (INSET_SIZE + (IN_TILE_SIZE / 4)),
);
const C_2_CENTER: (i32, i32) = (
    (INSET_SIZE + ((IN_TILE_SIZE / 4) * 3)),
    (INSET_SIZE + (IN_TILE_SIZE / 4)),
);
const C_3_CENTER: (i32, i32) = (
    (INSET_SIZE + (IN_TILE_SIZE / 4)),
    (INSET_SIZE + ((IN_TILE_SIZE / 4) * 3)),
);
const C_4_CENTER: (i32, i32) = (
    (INSET_SIZE + ((IN_TILE_SIZE / 4) * 3)),
    (INSET_SIZE + ((IN_TILE_SIZE / 4) * 3)),
);

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
    let img_width = (grid.clone().len() as i32 * TILE_SIZE) as u32;
    let img_height = (grid.clone()[0].len() as i32 * TILE_SIZE) as u32;

    let mut image = RgbImage::new(img_width, img_height);
    draw_filled_rect_mut(
        &mut image,
        Rect::at(0, 0).of_size(img_width, img_height),
        Rgb([255, 255, 255]),
    );
    let perlin = OpenSimplex::new(get_config().perlin_seed); // Noise so we can texture the tiles
    let clamped = Clamp::new(perlin).set_bounds(0.1, 0.5);

    /*
    let scale = Scale {
    x: TEXT_SCALE,
    y: TEXT_SCALE,
    }; todo
    */
    let completed_tiles: DashMap<(i32, i32), ImageBuffer<Rgb<u8>, Vec<u8>>> = DashMap::new();
    // let font_bytes = Vec::from(include_bytes!("../font.ttf") as &[u8]);
    // let font = Font::try_from_vec(font_bytes).unwrap(); todo
    let flatten_grid = Vec::from_iter(grid.iter().flatten().cloned());
    // Since everything is based on the X,Y of the tile, no point in setting up another loop
    let final_amount = flatten_grid.len();

    let center_tile = flatten_grid[flatten_grid.len() / 2].clone();
    let completed = RelaxedCounter::new(0);
    thread::scope(|s| {
        // imma be honest, idk what a scope is but it makes my IDE stop screaming at me so we'll use it
        for tile in flatten_grid.clone() {
            let completed_tiles = &completed_tiles;
            let clamped = &clamped;
            let completed = &completed;
            let faction = &faction;
            s.spawn(move || {
                let mut tile_image = RgbImage::new((TILE_SIZE) as u32, (TILE_SIZE) as u32);
                for x in 0..TILE_SIZE {
                    for y in 0..TILE_SIZE {
                        let abs_x = (tile.x * TILE_SIZE) + x;
                        let abs_y = (tile.y * TILE_SIZE) + y;
                        let noise = clamped.get([abs_x as f64 / 100.0, abs_y as f64 / 100.0]);
                        let hsv = hsv_to_rgb(143.0, 96.0, noise as f32);
                        tile_image.put_pixel(x as u32, y as u32, Rgb([hsv.0, hsv.1, hsv.2]));
                        if tile.x == center_tile.x && tile.y == center_tile.y {
                            let offset_start = TILE_SIZE / 4;
                            let seg_size = (TILE_SIZE / 2) as u32;

                            draw_filled_rect_mut(
                                &mut tile_image,
                                Rect::at(0, offset_start).of_size(BORDER_SIZE as u32, seg_size),
                                Rgb([255, 0, 0]),
                            );
                            draw_filled_rect_mut(
                                &mut tile_image,
                                Rect::at(offset_start, 0).of_size(seg_size, BORDER_SIZE as u32),
                                Rgb([255, 0, 0]),
                            );
                            draw_filled_rect_mut(
                                &mut tile_image,
                                Rect::at(TILE_SIZE - BORDER_SIZE, offset_start)
                                    .of_size(BORDER_SIZE as u32, seg_size),
                                Rgb([255, 0, 0]),
                            );
                            draw_filled_rect_mut(
                                &mut tile_image,
                                Rect::at(offset_start, TILE_SIZE - BORDER_SIZE)
                                    .of_size(seg_size, BORDER_SIZE as u32),
                                Rgb([255, 0, 0]),
                            );
                        }

                        // Now draw the actual tile

                        let color = if tile.occupied {
                            if tile.faction == *faction {
                                Rgb([46, 130, 1])
                            } else {
                                Rgb([28, 172, 255])
                            }
                        } else {
                            Rgb([28, 119, 68])
                        };

                        draw_filled_circle_mut(
                            &mut tile_image,
                            C_2_CENTER,
                            (IN_TILE_SIZE / 4) as i32,
                            color,
                        );
                        draw_filled_circle_mut(
                            &mut tile_image,
                            C_3_CENTER,
                            (IN_TILE_SIZE / 4) as i32,
                            color,
                        );
                        draw_filled_circle_mut(
                            &mut tile_image,
                            C_4_CENTER,
                            (IN_TILE_SIZE / 4) as i32,
                            color,
                        );
                        draw_filled_circle_mut(
                            &mut tile_image,
                            C_1_CENTER,
                            (IN_TILE_SIZE / 4) as i32,
                            color,
                        );

                        draw_filled_rect_mut(
                            &mut tile_image,
                            Rect::at(INSET_SIZE + (IN_TILE_SIZE / 4), INSET_SIZE)
                                .of_size((IN_TILE_SIZE / 2) as u32, IN_TILE_SIZE as u32),
                            color,
                        );

                        draw_filled_rect_mut(
                            &mut tile_image,
                            Rect::at(INSET_SIZE, INSET_SIZE + (IN_TILE_SIZE / 4))
                                .of_size(IN_TILE_SIZE as u32, (IN_TILE_SIZE / 2) as u32),
                            color,
                        );
                    }
                }
                completed_tiles.insert((tile.x, tile.y), tile_image);
                completed.inc();
            });
        }
    });
    loop {
        // we block until every thread is complete. There is probably a better way to do this
        if completed.get() >= final_amount {
            // It can't actually go over but if i mess up and it does, don't want it to hang
            break;
        }
    }

    let mut x_sorted = flatten_grid.iter().map(|tile| tile.x).collect::<Vec<i32>>();
    x_sorted.sort();
    let mut y_sorted = flatten_grid.iter().map(|tile| tile.y).collect::<Vec<i32>>();
    y_sorted.sort();

    let min_x = x_sorted[0];
    let min_y = y_sorted[0];

    for tile in flatten_grid {
        let tile_image = completed_tiles.get(&(tile.x, tile.y)).unwrap().clone();
        let rel_x = tile.x - min_x;
        let rel_y = tile.y - min_y;
        image
            .copy_from(
                &tile_image
                    .view(0, 0, TILE_SIZE as u32, TILE_SIZE as u32)
                    .to_image(),
                (rel_x as i32 * (TILE_SIZE)) as u32,
                (rel_y as i32 * (TILE_SIZE)) as u32,
            )
            .expect("Failed to copy tile to final image");
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