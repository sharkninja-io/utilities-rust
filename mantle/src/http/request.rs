use std::fmt::{self, Display};

#[derive(Default, Clone, Debug)]
pub struct Request {
    pub url: String,
    pub method: Method,
    pub body: Option<Vec<u8>>,
    pub timeout: u8,
    pub headers: Vec<Header>,
}

#[derive(Default, Clone, Debug)]
pub struct Header {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub enum Method {
    GET,
    PUT,
    POST,
    DELETE,
}

impl Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Method {
    fn default() -> Self {
        Self::GET
    }
}
