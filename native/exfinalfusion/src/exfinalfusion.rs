use crate::error::ExFinalFusionError;
use std::fs::File;
use std::io::BufReader;

use finalfusion::prelude::*;
//pub struct ExFinalFusionRef(BufReader<File>);

//#[rustler::nif]
pub fn from_file(path: &str) -> Result<Embeddings<VocabWrap, StorageWrap>, ExFinalFusionError> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let embeddings = Embeddings::<VocabWrap, StorageWrap>::read_embeddings(&mut reader)?;
    Ok(embeddings)
}
pub fn get_embeddings(embeds: Embeddings<VocabWrap, StorageWrap>, string: &str) -> String {
    let embeddings = embeds.embedding(string).unwrap();
    println!("{:?}", &embeddings);
    embeddings.to_string()
}

//rustler::init!("Elixir.ExFinalFusion", [from_file]);
