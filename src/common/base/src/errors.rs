use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RobustMQError {
    #[error("io error")]
    IOJsonError(#[from] io::Error),
    #[error("{0}")]
    CommonError(String),
}
