use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub operation: Operation,
    pub section: Option<String>,
    pub name: Option<String>,
    pub data: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Operation {
    Add,
    Remove,
}

#[derive(Serialize, Deserialize)]
pub enum Requests {
    Send(Request),
    Recieve,
}

impl Request {
    pub fn new(
        operation: Operation,
        section: Option<String>,
        name: Option<String>,
        data: Option<String>,
    ) -> Request {
        Request {
            operation,
            section,
            name,
            data,
        }
    }
}
