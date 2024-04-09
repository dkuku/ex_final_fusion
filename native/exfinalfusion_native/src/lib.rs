pub mod error;
pub mod exfinalfusion;

use crate::error::ExFinalFusionError;
use finalfusion::prelude::*;

use rustler::{Encoder, Env, ResourceArc, Term};
rustler::atoms! { error, ok, }

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
pub fn get_embeddings(reference: ExEmbeddings, string: &str) -> String {
    let embeds = &reference.resource.0;
    let embeddings = embeds.embedding(string).unwrap();
    println!("{:?}", &embeddings);
    embeddings.to_string()
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