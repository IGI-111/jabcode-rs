use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("Decoding JABCode failed")]
    Jab,
}
