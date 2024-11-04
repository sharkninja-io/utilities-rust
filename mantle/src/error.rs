use serde_json::error::Category;
use std::error::Error;
use std::fmt::{Debug, Display};

pub trait MantleResultError: Error + Display + Debug + Sync + Send {
    fn error_type(&self) -> String;
    fn error_description(&self) -> String;
}

impl MantleResultError for serde_json::Error {
    fn error_type(&self) -> String {
        match self.classify() {
            Category::Io => "io",
            Category::Syntax => "syntax",
            Category::Data => "data",
            Category::Eof => "eof",
        }
        .to_string()
    }

    fn error_description(&self) -> String {
        self.to_string()
    }
}
