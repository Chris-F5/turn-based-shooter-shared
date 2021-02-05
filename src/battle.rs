mod coordinates;
pub mod map;

pub use coordinates::{TilePos, TileVec, WorldPos};
pub use map::Map;

use serde::{Deserialize, Serialize};

pub struct Battle {
    map: Map,
    white_player_pos: TilePos,
    black_player_pos: TilePos,
    team_turn: Team,
}
pub struct ActionError;
impl Battle {
    pub fn new() -> Battle {
        let mut map = Map::new(8, 8);
        map.set_tile(&TilePos::new(2, 6), map::Tile::new_black());
        Battle {
            map,
            white_player_pos: TilePos::new(1, 1),
            black_player_pos: TilePos::new(6, 6),
            team_turn: Team::White,
        }
    }
    pub fn battle_info(&self, team: Team) -> BattleInfo {
        BattleInfo {
            team,
            team_turn: self.team_turn,
            map: self.map.clone(),
            black_player_pos: self.black_player_pos.clone(),
            white_player_pos: self.white_player_pos.clone(),
        }
    }
    pub fn play_action(
        &mut self,
        action: Action,
        team: Team,
    ) -> Result<(BattleInfoUpdate, BattleInfoUpdate), ActionError> {
        if self.team_turn == team {
            return Err(ActionError);
        }
        match action {
            Action::Move(vec) => match team {
                Team::White => {
                    let new_pos = self.white_player_pos.add_vec(&vec);
                    if self.map.get_tile(&new_pos).is_some() {
                        self.white_player_pos = new_pos;
                    } else {
                        return Err(ActionError);
                    }
                }
                Team::Black => {
                    let new_pos = self.black_player_pos.add_vec(&vec);
                    if self.map.get_tile(&new_pos).is_some() {
                        self.black_player_pos = new_pos;
                    } else {
                        return Err(ActionError);
                    }
                }
            },
        };
        let white_info_update = BattleInfoUpdate {
            white_player_pos: self.white_player_pos.clone(),
            black_player_pos: self.black_player_pos.clone(),
        };
        let black_info_update = BattleInfoUpdate {
            white_player_pos: self.white_player_pos.clone(),
            black_player_pos: self.black_player_pos.clone(),
        };
        Ok((white_info_update, black_info_update))
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BattleInfo {
    map: Map,
    team: Team,
    team_turn: Team,
    black_player_pos: TilePos,
    white_player_pos: TilePos,
}

impl BattleInfo {
    pub fn update(&mut self, update: &BattleInfoUpdate) {
        self.black_player_pos = update.black_player_pos.clone();
        self.white_player_pos = update.white_player_pos.clone();
    }
    pub fn get_actions(&self) -> Vec<Action> {
        if self.team == self.team_turn {
            match self.team {
                Team::White => vec![Action::Move(TileVec::new(1, 0))],
                Team::Black => vec![Action::Move(TileVec::new(0, -1))],
            }
        } else {
            panic!("cant get actions when its not my turn");
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BattleInfoUpdate {
    black_player_pos: TilePos,
    white_player_pos: TilePos,
}

#[derive(Deserialize, Serialize, Copy, Clone, PartialEq)]
pub enum Team {
    Black,
    White,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum Action {
    Move(TileVec),
}
