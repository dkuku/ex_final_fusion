pub mod error;
pub mod exfinalfusion;

use crate::error::ExFinalFusionError;
use finalfusion::prelude::*;

use rustler::{Atom, Encoder, Env, NifTuple, ResourceArc, Term};
mod atoms {
    rustler::atoms! {
        ok,
        error,
    }
}
rustler::atoms! { error, ok, }

#[derive(NifTuple)]
struct ResponseTerm<'a> {
    status: Atom,
    message: Term<'a>,
}
type Embeds = Embeddings<VocabWrap, StorageWrap>;
pub struct ExFinalFusionRef(Embeds);
impl<'a> Encoder for ExFinalFusionRef {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        (ok(), self).encode(env)
    }
}

impl From<Embeds> for ExEmbeddings {
    fn from(data: Embeds) -> Self {
        Self {
            resource: ResourceArc::new(ExFinalFusionRef(data)),
        }
    }
}
#[derive(rustler::NifStruct)]
#[module = "ExFinalFusion.Embeddings"]
pub struct ExEmbeddings {
    pub resource: ResourceArc<ExFinalFusionRef>,
}

#[rustler::nif]
pub fn from_file(path: &str) -> Result<ExEmbeddings, ExFinalFusionError> {
    match exfinalfusion::from_file(path) {
        Ok(embeddings) => Ok(embeddings.into()),
        Err(_) => todo!(),
    }
}
#[rustler::nif]
pub fn get_embeddings<'a>(
    env: Env<'a>,
    reference: ExEmbeddings,
    string: &str,
) -> Result<Term<'a>, ExFinalFusionError> {
    let embeds = &reference.resource.0;
    let embeddings = embeds.embedding(string).unwrap();
    let vec = &embeddings.iter().collect::<Vec<&f32>>();
    match serde_rustler::to_term(env, vec) {
        Ok(term) => Ok(term),
        Err(_e) => todo!(),
    }
}
fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(ExFinalFusionRef, env);
    true
}
rustler::init!(
    "Elixir.ExFinalFusion.Native",
    [from_file, get_embeddings],
    load = load
);
