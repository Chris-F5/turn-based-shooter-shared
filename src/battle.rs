mod coordinates;
pub mod map;

pub use coordinates::{TilePos, TileVec, WorldPos};
pub use map::Map;

use serde::{Deserialize, Serialize};

pub struct Battle {
    data: BattleData,
    white_info_update: Option<BattleInfoUpdate>,
    black_info_update: Option<BattleInfoUpdate>,
    white_info: BattleInfo,
    black_info: BattleInfo,
}

pub struct BattleData {
    map: Map,
    white_player_pos: TilePos,
    black_player_pos: TilePos,
}
impl BattleData {
    pub fn new(map: Map, black_player_pos: TilePos, white_player_pos: TilePos) -> BattleData {
        BattleData {
            map,
            black_player_pos,
            white_player_pos,
        }
    }
}

impl BattleData {
    fn get_action_set(&self, team: Team) -> ActionSet {
        match team {
            Team::White => vec![Action {
                team,
                action_type: ActionType::Move(TileVec::new(1, 0)),
            }],
            Team::Black => vec![Action {
                team,
                action_type: ActionType::Move(TileVec::new(0, -1)),
            }],
        }
    }
}

impl Battle {
    pub fn new(data: BattleData) -> Battle {
        Battle {
            white_info: BattleInfo::from_battle_data(&data, Team::White),
            black_info: BattleInfo::from_battle_data(&data, Team::Black),
            white_info_update: None,
            black_info_update: None,
            data,
        }
    }
    pub fn play_action(&mut self, action_index: ActionIndex, team: Team) {
        let action = match team {
            Team::White => &self.white_info.action_set[action_index],
            Team::Black => &self.black_info.action_set[action_index],
        };
        match &action.action_type {
            ActionType::Move(vec) => match action.team {
                Team::White => {
                    self.data.white_player_pos.add_vec(&vec);
                }
                Team::Black => {
                    self.data.black_player_pos.add_vec(&vec);
                }
            },
        };
        self.white_info_update = Some(BattleInfoUpdate {
            white_player_pos: self.data.white_player_pos.clone(),
            black_player_pos: self.data.black_player_pos.clone(),
            action_set: None,
        });
        self.black_info_update = Some(BattleInfoUpdate {
            white_player_pos: self.data.white_player_pos.clone(),
            black_player_pos: self.data.black_player_pos.clone(),
            action_set: None,
        });
    }

    pub fn take_white_battle_info_update(&mut self) -> Option<BattleInfoUpdate> {
        let update_info = self.white_info_update.take();
        if let Some(update_info) = update_info {
            self.white_info.update(&update_info);
            Some(update_info)
        } else {
            None
        }
    }
    pub fn take_black_battle_info_update(&mut self) -> Option<BattleInfoUpdate> {
        let update_info = self.black_info_update.take();
        if let Some(update_info) = update_info {
            self.black_info.update(&update_info);
            Some(update_info)
        } else {
            None
        }
    }
    pub fn white_battle_info(&self) -> &BattleInfo {
        &self.white_info
    }
    pub fn black_battle_info(&self) -> &BattleInfo {
        &self.black_info
    }
}

pub type ActionSet = Vec<Action>;
pub type ActionIndex = usize;

#[derive(Deserialize, Serialize, Clone)]
pub enum Team {
    Black,
    White,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Action {
    team: Team,
    action_type: ActionType,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum ActionType {
    Move(TileVec),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BattleInfo {
    map: Map,
    action_set: ActionSet,
    black_player_pos: TilePos,
    white_player_pos: TilePos,
}

impl BattleInfo {
    pub fn from_battle_data(battle_data: &BattleData, team: Team) -> BattleInfo {
        BattleInfo {
            map: battle_data.map.clone(),
            action_set: battle_data.get_action_set(team),
            black_player_pos: battle_data.black_player_pos.clone(),
            white_player_pos: battle_data.white_player_pos.clone(),
        }
    }
    pub fn update(&mut self, update: &BattleInfoUpdate) {
        self.black_player_pos = update.black_player_pos.clone();
        self.white_player_pos = update.white_player_pos.clone();
    }
}

#[derive(Serialize, Deserialize)]
pub struct BattleInfoUpdate {
    black_player_pos: TilePos,
    white_player_pos: TilePos,
    action_set: Option<ActionSet>,
}
