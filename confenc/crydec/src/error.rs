pub type CrydecResult<T> = Result<T, CrydecError>;

/// Error, which can occur while encrypting or decrypting data.
#[derive(Debug, thiserror::Error)]
pub enum CrydecError {
    #[error(
        "Error while encrypting data:\n\t\
            the buffer has insufficient capacity to store the resulting ciphertext message"
    )]
    EncryptionError(#[source] aes_gcm::aead::Error),

    #[error(
        "Error while decrypting data:\n\t\
            provided authentication tag does not match the given ciphertext"
    )]
    DecryptionError(#[source] aes_gcm::aead::Error),

    #[error("Error converting data to UTF-8: {0}")]
    Utf8Error(
        #[from]
        #[source]
        std::string::FromUtf8Error,
    ),
}
