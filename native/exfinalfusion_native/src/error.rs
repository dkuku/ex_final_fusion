use finalfusion::error::Error as FFError;
use rustler::{Encoder, Env, Term};
use serde_rustler::Error as SRError;
use std::io;
use thiserror::Error;

rustler::atoms! {
    ok,
    error
}

#[derive(Error, Debug)]
pub enum ExFinalFusionError {
    #[error("FF Error")]
    FF(#[from] FFError),
    #[error("SR Error")]
    SR(#[from] SRError),
    #[error("IO Error")]
    Io(#[from] io::Error),
    #[error("Internal Error: {0}")]
    Internal(String),
    #[error("Other error: {0}")]
    Other(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl Encoder for ExFinalFusionError {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        format!("{self:?}").encode(env)
    }
}
