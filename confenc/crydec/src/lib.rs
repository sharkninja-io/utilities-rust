//! Utility library for encrypting and decrypting data using AES-GCM.

pub mod error;

use aes_gcm::aead::{Aead, OsRng, Payload};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use error::{CrydecError, CrydecResult};
use rand::Rng;

/// Encrypted data + nonce + key.
///
/// Used to pass encrypted data to the decryption scope at runtime.
pub struct Encrypted {
    pub data: Vec<u8>,
    pub nonce: [u8; 12],
    pub key: [u8; 32],
}

/// Encrypts data using AES-GCM with a 256-bit key and 96-bit nonce.
///
/// Used by the `confenc!` macro.
pub fn encrypt<'msg, 'aad>(data: impl Into<Payload<'msg, 'aad>>) -> CrydecResult<Encrypted> {
    let mut rng = rand::thread_rng();
    let nonce = Nonce::from(rng.gen::<[u8; 12]>());
    let key = Aes256Gcm::generate_key(&mut OsRng);
    let cipher = Aes256Gcm::new(&key);
    let encrypted = cipher
        .encrypt(&nonce, data)
        .map_err(CrydecError::EncryptionError)?;
    #[cfg(test)]
    {
        dbg!(&nonce);
        dbg!(&encrypted);
    }
    Ok(Encrypted {
        data: encrypted,
        nonce: nonce.into(),
        key: key.into(),
    })
}

/// Decrypts data using AES-GCM with a 256-bit key and 96-bit nonce.
///
/// Used by the `confenc!` macro.
pub fn decrypt(key: &[u8; 32], nonce: &[u8; 12], data: &[u8]) -> CrydecResult<String> {
    #[cfg(test)]
    {
        dbg!(nonce);
        dbg!(data);
    }
    let cipher = Aes256Gcm::new(key.into());
    let decrypted = cipher
        .decrypt(Nonce::from_slice(nonce), data)
        .map_err(CrydecError::DecryptionError)?;
    Ok(String::from_utf8(decrypted)?)
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn decrypted_data_should_be_the_same_as_the_original_data(data in ".*") {
            let encrypted = encrypt(data.as_bytes()).unwrap();
            let decrypted = decrypt(&encrypted.key, &encrypted.nonce, &encrypted.data).unwrap();
            prop_assert_eq!(data, decrypted);
        }

        #[test]
        fn encrypted_data_should_be_different_from_the_original_data(data in ".*") {
            let encrypted = encrypt(data.as_bytes()).unwrap();
            prop_assert_ne!(data.as_bytes(), encrypted.data);
        }
    }
}
