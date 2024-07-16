use anyhow::Error;
use fastembed::{Embedding, EmbeddingModel, InitOptions, TextEmbedding};

use serde::Deserialize;

#[derive(Deserialize)]
struct JsonData {
    resultset: ResultSet,
}

#[derive(Deserialize)]
struct ResultSet {
    row: Vec<Row>,
}

#[derive(Deserialize)]
struct Row {
    field: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct MyStruct {
    i: usize,
    b: usize,
    c: usize,
    v: usize,
}

type Books = Vec<String>;


pub fn process_json_file(file_path: &str) -> Result<Vec<MyStruct>, Error> {
    let json_data: JsonData = serde_json::from_str(&std::fs::read_to_string(file_path)?)?;
    let books: Books = serde_json::from_str(&std::fs::read_to_string("books.json")?)?;

    let mut structs: Vec<MyStruct> = Vec::new();

    for row in json_data.resultset.row {
        if row.field.len() >= 4 {
            structs.push(MyStruct {
                i: row.field[0],
                b: row.field[1],
                c: row.field[2],
                v: row.field[3],
            });
        }
    }

    Ok(structs)
}

pub fn embed(text: &str) -> Result<Embedding, Error> {
    // With default InitOptions
    // let model = TextEmbedding::try_new(Default::default())?;

    // With custom InitOptions
    let model = TextEmbedding::try_new(InitOptions {
        model_name: EmbeddingModel::MxbaiEmbedLargeV1Q,
        show_download_progress: true,
        ..Default::default()
    })?;

    let documents = vec!["query: {}", text];

    // Generate embeddings with the default batch size, 256
    let embeddings = model.embed(documents, None)?;
    Ok(embeddings[0].clone())
}


//  println!("Embeddings length: {}", embeddings.len()); // -> Embeddings length: 4
//  println!("Embedding dimension: {}", embeddings[0].len()); // -> Embedding dimension: 384
