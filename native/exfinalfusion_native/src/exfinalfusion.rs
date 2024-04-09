use crate::error::ExFinalFusionError;
use std::fs::File;
use std::io::BufReader;

use finalfusion::prelude::*;

pub fn from_file(path: &str) -> Result<Embeddings<VocabWrap, StorageWrap>, ExFinalFusionError> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let embeddings = Embeddings::<VocabWrap, StorageWrap>::read_embeddings(&mut reader)?;
    Ok(embeddings)
}
