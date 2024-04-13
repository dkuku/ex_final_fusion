pub mod error;
use crate::error::ExFinalFusionError;
use finalfusion::compat::floret::ReadFloretText;
use finalfusion::prelude::*;
use finalfusion::similarity::{Analogy, WordSimilarity, WordSimilarityResult};
use finalfusion::vocab::{
    Vocab,
    WordIndex::{Subword, Word},
};
use ndarray::Axis;
use std::fs::File;
use std::io::BufReader;
use std::ops::Deref;

use rustler::{Atom, Encoder, Env, NifTaggedEnum, NifTuple, NifUnitEnum, ResourceArc, Term};
mod atoms {
    rustler::atoms! {
        ok,
        error,
        word,
        subword,
    }
}
#[derive(NifTaggedEnum, Clone, Debug)]
pub enum SimilarityType {
    AngularSimilarity,
    CosineSimilarity,
    EuclideanSimilarity,
    EuclideanDistance,
}
#[derive(NifTaggedEnum, Debug)]
pub enum SearchOptionPub {
    Limit(usize),
    BatchSize(usize),
    SimilarityType(SimilarityType),
}
#[derive(Debug)]
struct SearchOption {
    limit: usize,
    batch_size: Option<usize>,
    similarity_type: SimilarityType,
}

#[derive(NifTuple)]
struct ResponseTerm<'a> {
    status: Atom,
    message: Term<'a>,
}
type EmbeddingWrap = Embeddings<VocabWrap, StorageViewWrap>;
pub struct ExFinalFusionRef(EmbeddingWrap);
impl Encoder for ExFinalFusionRef {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        (atoms::ok(), self).encode(env)
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
    strings: Vec<&str>,
) -> Result<Term<'a>, ExFinalFusionError> {
    let (embeddings, _rest) = &reference.resource.0.embedding_batch(&strings);
    let embeddings_array = embeddings
        .axis_iter(Axis(0))
        .map(|x| x.iter().cloned().collect::<Vec<f32>>())
        .collect::<Vec<_>>();
    Ok(serde_rustler::to_term(env, &embeddings_array)?)
}
#[rustler::nif]
pub fn embedding<'a>(
    env: Env<'a>,
    reference: ExEmbeddings,
    string: &str,
) -> Result<Term<'a>, ExFinalFusionError> {
    match &reference.resource.0.embedding(string) {
        Some(embeddings) => {
            let vec = &embeddings.iter().collect::<Vec<&f32>>();
            Ok(serde_rustler::to_term(env, vec)?)
        }
        None => Err(ExFinalFusionError::Internal(
            "embedding not found".to_string(),
        )),
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
pub fn words(env: Env<'_>, reference: ExEmbeddings) -> Result<Term<'_>, ExFinalFusionError> {
    let vocab_words = reference
        .resource
        .0
        .vocab()
        .words()
        .iter()
        .map(|word| word.to_string())
        .collect::<Vec<String>>();
    Ok(serde_rustler::to_term(env, vocab_words)?)
}
#[rustler::nif]
pub fn idx<'a>(env: Env<'a>, reference: ExEmbeddings, string: &str) -> Option<Term<'a>> {
    match reference.resource.0.vocab().idx(string) {
        Some(Word(index)) => Some((atoms::word(), vec![index]).encode(env)),
        Some(Subword(indexes)) => Some((atoms::subword(), indexes).encode(env)),
        None => None,
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
#[rustler::nif]
pub fn vocab_len(reference: ExEmbeddings) -> usize {
    reference.resource.0.vocab().vocab_len()
}
#[rustler::nif]
pub fn words_len(reference: ExEmbeddings) -> usize {
    reference.resource.0.vocab().words_len()
}

#[rustler::nif]
pub fn analogy(
    reference: ExEmbeddings,
    w1: &str,
    w2: &str,
    w3: &str,
    options: Vec<SearchOptionPub>,
) -> Result<Vec<(String, f32)>, ExFinalFusionError> {
    analogy_wrapper(reference, [w1, w2, w3], [true; 3], options)
}
#[rustler::nif]
pub fn analogy_masked(
    reference: ExEmbeddings,
    w1: &str,
    w1_mask: bool,
    w2: &str,
    w2_mask: bool,
    w3: &str,
    w3_mask: bool,
    options: Vec<SearchOptionPub>,
) -> Result<Vec<(String, f32)>, ExFinalFusionError> {
    analogy_wrapper(
        reference,
        [w1, w2, w3],
        [w1_mask, w2_mask, w3_mask],
        options,
    )
}
#[rustler::nif]
fn word_similarity(
    reference: ExEmbeddings,
    string: &str,
    options: Vec<SearchOptionPub>,
) -> Result<Vec<(String, f32)>, ExFinalFusionError> {
    let opts = get_options(options);
    let result = reference
        .resource
        .0
        .word_similarity(string, opts.limit, opts.batch_size)
        .expect("Similarities not found");
    let data = convert_result(result, &opts);
    Ok(data)
}

fn analogy_wrapper(
    reference: ExEmbeddings,
    strings: [&str; 3],
    mask: [bool; 3],
    options: Vec<SearchOptionPub>,
) -> Result<Vec<(String, f32)>, ExFinalFusionError> {
    // Default values
    let opts = get_options(options);
    let result = reference
        .resource
        .0
        .analogy_masked(strings, mask, opts.limit, opts.batch_size)
        .map_err(|_e| ExFinalFusionError::Other("Mask problem".to_string()))?;
    Ok(convert_result(result, &opts))
}
fn convert_result(result: Vec<WordSimilarityResult>, options: &SearchOption) -> Vec<(String, f32)> {
    result
        .iter()
        .map(|similarity| {
            (
                similarity.word().to_string(),
                get_similarity_value(similarity, &options.similarity_type),
            )
        })
        .collect::<Vec<_>>()
}
fn get_similarity_value(
    similarity: &WordSimilarityResult,
    similarity_type: &SimilarityType,
) -> f32 {
    let similarity_value = match similarity_type {
        SimilarityType::AngularSimilarity => similarity.angular_similarity(),
        SimilarityType::CosineSimilarity => similarity.cosine_similarity(),
        SimilarityType::EuclideanSimilarity => similarity.euclidean_similarity(),
        SimilarityType::EuclideanDistance => similarity.euclidean_distance(),
    };
    if f32::is_nan(similarity_value) {
        0.0
    } else {
        similarity_value
    }
}
fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(ExFinalFusionRef, env);
    true
}
fn get_options(options: Vec<SearchOptionPub>) -> SearchOption {
    let mut opts = SearchOption {
        limit: 1,
        batch_size: None,
        similarity_type: SimilarityType::CosineSimilarity,
    };

    options.iter().for_each(|option| match option {
        SearchOptionPub::Limit(val) => opts.limit = *val,
        SearchOptionPub::BatchSize(val) => opts.batch_size = Some(*val),
        SearchOptionPub::SimilarityType(val) => opts.similarity_type = val.clone(),
    });
    opts
}
rustler::init!(
    "Elixir.ExFinalFusion.Native",
    [
        analogy,
        analogy_masked,
        dims,
        embedding,
        embedding_batch,
        idx,
        len,
        metadata,
        read,
        vocab_len,
        word_similarity,
        words,
        words_len,
    ],
    load = load
);
//vec![
//    "into_parts",
//    "norms",

//    "embedding_into",
//    "embedding_with_norm",

//    "fmt",
//    "embedding_similarity_masked",
//    "embedding_similarity",
//    "quantize_using",
//    "quantize",
//    "type_id",
//];
