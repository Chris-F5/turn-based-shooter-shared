use serde::{de::DeserializeOwned, Deserialize, Serialize};

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

pub enum ClientPacket {
    Test(TestRequest),
}
pub enum ServerPacket {
    Test(TestResponse),
}
