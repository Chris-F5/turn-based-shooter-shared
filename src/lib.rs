pub mod battle;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TestRequest {
    pub name: String,
}

impl TestRequest {
    pub fn new(name: String) -> TestRequest {
        TestRequest { name }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TestResponse {
    pub number: u32,
    pub message: String,
}

impl TestResponse {
    pub fn new(number: u32, message: String) -> TestResponse {
        TestResponse { number, message }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ClientPacket {
    Test(TestRequest),
    RequestBattle,
    SelectMove(u32),
}

#[derive(Serialize, Deserialize)]
pub enum ServerPacket {
    Test(TestResponse),
    BattleInfo(battle::BattleInfo),
    BattleInfoUpdate(battle::BattleInfoUpdate),
}
