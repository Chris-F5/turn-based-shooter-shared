use serde::{Deserialize, Serialize};

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
    pub fn add_vec(&mut self, vec: &TileVec) -> TilePos {
        TilePos::new(
            (self.x as i32 + vec.x) as u32,
            (self.y as i32 + vec.y) as u32,
        )
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
}
