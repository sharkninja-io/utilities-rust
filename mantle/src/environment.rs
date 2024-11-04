#[derive(Debug, Clone)]
pub enum Environment { 
    Dev,
    Prod,
}

impl From<i8> for Environment {
    fn from(value: i8) -> Self {
        match value {
            0 => Environment::Dev,
            _ => Environment::Prod,
        }
    }
}