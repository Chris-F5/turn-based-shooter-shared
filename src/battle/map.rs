use super::TilePos;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum TileAppearance {
    TestWhite,
    TestBlack,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tile {
    appearance: TileAppearance,
}
impl Tile {
    pub fn new_white() -> Tile {
        Tile {
            appearance: TileAppearance::TestWhite,
        }
    }
    pub fn new_black() -> Tile {
        Tile {
            appearance: TileAppearance::TestBlack,
        }
    }
    pub fn appearance(&self) -> &TileAppearance {
        &self.appearance
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    tiles: Vec<Tile>,
    x_size: usize,
    y_size: usize,
}

impl Map {
    pub fn new(x_size: usize, y_size: usize) -> Map {
        let mut tiles = Vec::with_capacity(x_size * y_size);
        for _ in 0..y_size {
            for _ in 0..x_size {
                tiles.push(Tile::new_white())
            }
        }
        Map {
            x_size,
            y_size,
            tiles,
        }
    }
    pub fn get_tile(&self, pos: &TilePos) -> &Tile {
        let x = pos.x as usize;
        let y = pos.y as usize;
        &self.tiles[y * self.x_size + x]
    }
    pub fn set_tile(&mut self, pos: &TilePos, tile: Tile) {
        let x = pos.x as usize;
        let y = pos.y as usize;
        self.tiles[y * self.x_size + x] = tile;
    }
    pub fn x_size(&self) -> u32 {
        self.x_size as u32
    }
    pub fn y_size(&self) -> u32 {
        self.y_size as u32
    }
}
