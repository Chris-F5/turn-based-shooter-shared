use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum TileAppearance {
    TestWhite,
}

#[derive(Serialize, Deserialize)]
pub struct Tile {
    appearance: TileAppearance,
}
impl Tile {
    pub fn appearance(&self) -> &TileAppearance {
        &self.appearance
    }
}

#[derive(Serialize, Deserialize)]
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
                tiles.push(Tile {
                    appearance: TileAppearance::TestWhite,
                })
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
    pub fn x_size(&self) -> u32 {
        self.x_size as u32
    }
    pub fn y_size(&self) -> u32 {
        self.y_size as u32
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TilePos {
    pub x: u32,
    pub y: u32,
}
impl TilePos {
    pub fn new(x: u32, y: u32) -> TilePos {
        TilePos { x, y }
    }
    pub fn world_pos(&self) -> WorldPos {
        WorldPos::new(self.x as f64, self.y as f64)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldPos {
    pub x: f64,
    pub y: f64,
}
impl WorldPos {
    pub fn new(x: f64, y: f64) -> WorldPos {
        WorldPos { x, y }
    }
}
