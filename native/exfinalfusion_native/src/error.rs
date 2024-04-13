use finalfusion::error::Error as FinalFusionError;
use rustler::{Encoder, Env, Term};
use serde_rustler::Error as SerdeRustlerError;
use std::io;
use thiserror::Error;

rustler::atoms! {
    ok,
    error
}

#[derive(Error, Debug)]
pub enum ExFinalFusionError {
    #[error("FinalFusion Error: {0}")]
    FinalFusion(#[from] FinalFusionError),
    #[error("SerdeRustler Error: {0}")]
    SerdeRustler(#[from] SerdeRustlerError),
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
