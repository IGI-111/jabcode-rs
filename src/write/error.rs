use thiserror::Error;

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("Creating encode failed")]
    Encode,
    #[error("Creating jab code failed with code: {0}")]
    Jab(i32),
}
