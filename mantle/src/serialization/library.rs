use serde::{Deserialize, Serialize};

use crate::error::MantleResultError;

pub fn encode<T>(object: &T) -> Vec<u8>
where
    T: Serialize,
{
    serde_json::to_vec(object).unwrap()
}

pub fn decode<T>(bytes: Vec<u8>) -> T
where
    T: for<'a> Deserialize<'a>,
{
    serde_json::from_slice(&bytes[..]).unwrap()
}

pub fn encode_result<T>(
    result: Result<T, Box<dyn MantleResultError>>,
) -> Result<Vec<u8>, Box<dyn MantleResultError>>
where
    T: Serialize,
{
    result.map(|suc| encode(&suc))
}
