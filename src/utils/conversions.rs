use crate::map::Tile;

pub fn bytes_to_string(bytes: u64) -> String {
    let mut bytes = bytes as f64;
    let suffixes = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    for suffix in suffixes.iter() {
        if bytes < 1024.0 {
            return format!("{:.2} {}", bytes, suffix);
        }
        bytes /= 1024.0;
    }
    format!("{} {}", bytes, "YB")
}

pub fn split_tiles(tiles: Vec<Tile>, size: i32) -> Vec<Vec<Tile>> {
    let mut tiles = tiles;
    let mut tile_rows: Vec<Vec<Tile>> = Vec::new();
    for _ in 0..size {
        let mut tile_row: Vec<Tile> = Vec::new();
        for _ in 0..size {
            tile_row.push(tiles.remove(0));
        }
        tile_rows.push(tile_row);
    }
    tile_rows
}