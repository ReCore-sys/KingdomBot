use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    pub(crate) faction: String,
    pub(crate) unit_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Building {
    pub(crate) faction: String,
    pub(crate) building_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileOccupant {
    pub(crate) faction: String,
    pub(crate) is_unit: bool,
    pub(crate) building: Building,
    pub(crate) unit: Unit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
    pub(crate) occupied: bool,
    pub(crate) occupant: TileOccupant,
    pub(crate) x: i32,
    pub(crate) y: i32,
}

/// Creates a blank tile
///
///
/// # Returns
/// ```Tile```: A blank tile
///

pub async fn blank_tile() -> Tile {
    Tile {
        occupied: false,
        occupant: TileOccupant {
            faction: String::new(),
            is_unit: false,
            building: Building {
                faction: String::new(),
                building_type: String::new(),
            },
            unit: Unit {
                faction: String::new(),
                unit_type: String::new(),
            },
        },
        x: 0,
        y: 0,
    }
}

/// Generates a list of blank tiles between the specified ranges
///
/// # Arguments
///
/// * `x_range` - The range of x values
/// * `y_range` - The range of y values
///
/// # Returns
/// ```Vec<Tile>```: A list of blank tiles with the specified ranges
///

pub async fn blank_tile_range(x_range: (i32, i32), y_range: (i32, i32)) -> Vec<Tile> {
    let mut tiles = Vec::new();
    for x in x_range.0..x_range.1 {
        for y in y_range.0..y_range.1 {
            let mut t = blank_tile().await;
            t.x = x;
            t.y = y;
            tiles.push(t);
        }
    }
    tiles
}