use anyhow::Error;
use fastembed::{Embedding, EmbeddingModel, InitOptions, TextEmbedding};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct JsonData {
    resultset: ResultSet,
}

#[derive(Deserialize)]
struct ResultSet {
    row: Vec<Row>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum FieldValue {
    I32(i32),
    Str(String),
}

#[derive(Deserialize)]
struct Row {
    field: Vec<FieldValue>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MyStruct {
    i: String,
    v: Vec<f32>,
}

// type Books = Vec<String>;

pub fn process_json_file(file_path: &str) -> Result<(), Error> {
    let json_data: JsonData = serde_json::from_str(&std::fs::read_to_string(file_path)?)?;
    // let books: Books = serde_json::from_str(&std::fs::read_to_string("books.json")?)?;

    let mut structs: Vec<MyStruct> = Vec::new();

    for row in json_data.resultset.row {
        let text = match &row.field[3] {
            FieldValue::I32(x) => x.to_string(),
            FieldValue::Str(x) => x.to_string(),
        };
        if row.field.len() >= 4 {
            structs.push(MyStruct {
                i: match &row.field[0] {
                    FieldValue::I32(x) => x.to_string(),
                    FieldValue::Str(x) => x.to_string(),
                },
                v: embed(&text).unwrap(),
            });
        }
    }

    let json_string = serde_json::to_string_pretty(&structs)?;
    std::fs::write("output.json", json_string)?;
    Ok(())
}

pub fn embed(text: &str) -> Result<Embedding, Error> {
    // With default InitOptions
    // let model = TextEmbedding::try_new(Default::default())?;

    // With custom InitOptions
    let model = TextEmbedding::try_new(InitOptions {
        model_name: EmbeddingModel::MxbaiEmbedLargeV1,
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
