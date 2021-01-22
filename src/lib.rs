use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub name: String,
}

impl Request {
    pub fn new(name: String) -> Request {
        Request { name }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub number: u32,
    pub message: String,
}

impl Response {
    pub fn new(number: u32, message: String) -> Response {
        Response { number, message }
    }
}
