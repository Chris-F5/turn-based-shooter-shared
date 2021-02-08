use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
}
impl TilePos {
    pub fn new(x: i32, y: i32) -> TilePos {
        TilePos { x, y }
    }
    pub fn world_pos(&self) -> WorldPos {
        WorldPos::new(self.x as f64, self.y as f64)
    }
    pub fn add_vec(&self, vec: &TileVec) -> TilePos {
        TilePos::new(self.x + vec.x, self.y + vec.y)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TileVec {
    pub x: i32,
    pub y: i32,
}
impl TileVec {
    pub fn new(x: i32, y: i32) -> TileVec {
        TileVec { x, y }
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
    pub fn tile_pos(&self) -> TilePos {
        TilePos::new(self.x.floor() as i32, self.y.floor() as i32)
    }
}
