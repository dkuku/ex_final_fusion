pub mod error;
use finalfusion::compat::floret::ReadFloretText;

use crate::error::ExFinalFusionError;
use finalfusion::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::ops::Deref;

use rustler::{Atom, Encoder, Env, NifTuple, NifUnitEnum, ResourceArc, Term};
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
type EmbeddingWrap = Embeddings<VocabWrap, StorageWrap>;
pub struct ExFinalFusionRef(EmbeddingWrap);
impl Encoder for ExFinalFusionRef {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        (ok(), self).encode(env)
    }
}
impl From<EmbeddingWrap> for ExEmbeddings {
    fn from(data: EmbeddingWrap) -> Self {
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
#[derive(NifUnitEnum, Debug)]
pub enum FileType {
    FloretText,
    Embeddings,
    MmapEmbeddings,
    Fasttext,
    FasttextLossy,
    Text,
    TextLossy,
    TextDims,
    TextDimsLossy,
    Word2vecBinary,
    Word2vecBinaryLossy,
    //aliases for the most popular ones
    Fifu,
    Word2vec,
    Floret,
}

#[rustler::nif]
pub fn read(path: &str, filetype: FileType) -> Result<ExEmbeddings, ExFinalFusionError> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let embeddings = match filetype {
        FileType::Fifu => Embeddings::read_embeddings(&mut reader)?,
        FileType::Embeddings => Embeddings::read_embeddings(&mut reader)?,
        FileType::MmapEmbeddings => Embeddings::mmap_embeddings(&mut reader)?,
        FileType::Word2vec => Embeddings::read_word2vec_binary(&mut reader)?.into(),
        FileType::Floret => Embeddings::read_floret_text(&mut reader)?.into(),
        FileType::FloretText => Embeddings::read_floret_text(&mut reader)?.into(),
        FileType::Fasttext => Embeddings::read_fasttext(&mut reader)?.into(),
        FileType::FasttextLossy => Embeddings::read_fasttext_lossy(&mut reader)?.into(),
        FileType::Text => Embeddings::read_text(&mut reader)?.into(),
        FileType::TextLossy => Embeddings::read_text_lossy(&mut reader)?.into(),
        FileType::TextDims => Embeddings::read_text_dims(&mut reader)?.into(),
        FileType::TextDimsLossy => Embeddings::read_text_dims_lossy(&mut reader)?.into(),
        FileType::Word2vecBinary => Embeddings::read_word2vec_binary(&mut reader)?.into(),
        FileType::Word2vecBinaryLossy => {
            Embeddings::read_word2vec_binary_lossy(&mut reader)?.into()
        }
    };
    Ok(embeddings.into())
}
#[rustler::nif]
pub fn embedding_batch<'a>(
    env: Env<'a>,
    reference: ExEmbeddings,
    words: Vec<&str>,
) -> Result<Term<'a>, ExFinalFusionError> {
    let (embeddings, _rest) = &reference.resource.0.embedding_batch(&words);
    match serde_rustler::to_term(env, embeddings.clone().into_raw_vec()) {
        Ok(term) => Ok(term),
        Err(e) => Err(ExFinalFusionError::SR(e)),
    }
}
#[rustler::nif]
pub fn embedding<'a>(
    env: Env<'a>,
    reference: ExEmbeddings,
    string: &str,
) -> Result<Term<'a>, ExFinalFusionError> {
    let embeddings = &reference.resource.0.embedding(string).unwrap();
    let vec = &embeddings.iter().collect::<Vec<&f32>>();
    match serde_rustler::to_term(env, vec) {
        Ok(term) => Ok(term),
        Err(e) => Err(ExFinalFusionError::SR(e)),
    }
}
#[rustler::nif]
pub fn metadata(env: Env, reference: ExEmbeddings) -> Option<Term> {
    let embeds = &reference.resource.0;
    match embeds.metadata() {
        None => None,
        Some(metadata) => match serde_rustler::to_term(env, metadata.deref()) {
            Ok(term) => Some(term),
            Err(_e) => None,
        },
    }
}
#[rustler::nif]
pub fn len(reference: ExEmbeddings) -> usize {
    reference.resource.0.len()
}
#[rustler::nif]
pub fn dims(reference: ExEmbeddings) -> usize {
    reference.resource.0.dims()
}
fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(ExFinalFusionRef, env);
    true
}
rustler::init!(
    "Elixir.ExFinalFusion.Native",
    [read, embedding, embedding_batch, len, metadata, dims],
    load = load
);
//vec![
//    "new",
//    "into_parts",
//    "metadata",
//    "norms",
//    "set_metadata",
//    "storage",
//    "vocab",
//    "embedding",
//    "embedding_into",
//    "embedding_batch",
//    "embedding_with_norm",

//    "analogy_masked",
//    "analogy",
//    "fmt",
//    "embedding_similarity_masked",
//    "embedding_similarity",
//    "quantize_using",
//    "quantize",
//    "word_similarity",
//    "write_embeddings",
//    "write_embeddings_len",
//    "write_fasttext",
//    "write_floret_text",
//    "write_text",
//    "write_text_dims",
//    "write_word2vec_binary",
//    "type_id",
//];
