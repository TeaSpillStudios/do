pub struct Request {
    operation: Operation,
    data: Option<String>,
}

pub enum Operation {
    Add,
    Remove,
}

pub enum Requests<'a> {
    Send(&'a str),
    Recieve,
}

