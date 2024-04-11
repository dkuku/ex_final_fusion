pub mod error;

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
impl<'a> Encoder for ExFinalFusionRef {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
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
    Fifu,
    Word2vec,
    Floret,
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
}

#[rustler::nif]
pub fn read(path: &str, filetype: FileType) -> Result<ExEmbeddings, ExFinalFusionError> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let embeddings = match filetype {
        FileType::Fifu => Embeddings::read_embeddings(&mut reader)?,
        FileType::Embeddings => Embeddings::read_embeddings(&mut reader)?,
        FileType::MmapEmbeddings => Embeddings::mmap_embeddings(&mut reader)?,

        //FileType::Word2vec => Embeddings::read_word2vec_binary(&mut reader)?,
        // FileType::Floret => Embeddings::read_floret_text(&mut reader)?,
        // FileType::FloretText => Embeddings::read_floret_text(&mut reader)?,
        // FileType::Fasttext => Embeddings::read_fasttext(&mut reader)?,
        // FileType::FasttextLossy => Embeddings::read_fasttext_lossy(&mut reader)?,
        // FileType::Text => Embeddings::read_text(&mut reader)?,
        // FileType::TextLossy => Embeddings::read_text_lossy(&mut reader)?,
        // FileType::TextDims => Embeddings::read_text_dims(&mut reader)?,
        // FileType::TextDimsLossy => Embeddings::read_text_dims_lossy(&mut reader)?,
        // FileType::Word2vecBinary => Embeddings::read_word2vec_binary(&mut reader)?,
        // FileType::Word2vecBinaryLossy => Embeddings::read_word2vec_binary_lossy(&mut reader)?,
        _ => todo!(),
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
//    "metadata_mut",
//    "norms",
//    "set_metadata",
//    "storage",
//    "vocab",
//    "dims",
//    "embedding",
//    "embedding_into",
//    "embedding_batch",
//    "embedding_batch_into",
//    "embedding_with_norm",
//    "iter",
//    "iter_with_norms",
//    "len",
//    "to_explicit",
//    "try_to_explicit",
//    "analogy_masked",
//    "analogy",
//    "clone",
//    "clone_from",
//    "fmt",
//    "embedding_similarity_masked",
//    "embedding_similarity",
//    "into_iter",
//    "mmap_embeddings",
//    "quantize_using",
//    "quantize",
//    "read_embeddings",
//    "read_fasttext",
//    "read_fasttext_lossy",
//    "read_floret_text",
//    "read_text",
//    "read_text_lossy",
//    "read_text_dims",
//    "read_text_dims_lossy",
//    "read_word2vec_binary",
//    "read_word2vec_binary_lossy",
//    "word_similarity",
//    "write_embeddings",
//    "write_embeddings_len",
//    "write_fasttext",
//    "write_floret_text",
//    "write_text",
//    "write_text_dims",
//    "write_word2vec_binary",
//    "type_id",
//    "borrow",
//    "borrow_mut",
//    "from",
//    "into",
//    "init",
//    "deref",
//    "deref_mut",
//    "drop",
//    "to_owned",
//    "clone_into",
//    "try_from",
//    "try_into",
//    "vzip",
//];
