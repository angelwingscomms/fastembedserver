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
pub struct Embed {
    i: i32,
    v: Vec<f32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ParsedRow {
    pub i: i32,
    pub b: i32,
    pub c: i32,
    pub v: i32,
    pub t: String,
}

// type Books = Vec<String>;

pub fn process_json_file(file_path: &str) -> Result<Vec<ParsedRow>, Error> {
    let json_data: JsonData = serde_json::from_str(&std::fs::read_to_string(file_path)?)?;
    // let books: Books = serde_json::from_str(&std::fs::read_to_string("books.json")?)?;

    let mut rows: Vec<ParsedRow> = Vec::new();

    for row in json_data.resultset.row {
        if row.field.len() >= 4 {
            rows.push(ParsedRow {
                i: if let FieldValue::I32(l) = row.field[0] {
                    l
                } else {
                    panic!("i not i32")
                },
                b: if let FieldValue::I32(l) = row.field[1] {
                    l
                } else {
                    panic!("b not i32")
                },
                c: if let FieldValue::I32(l) = row.field[2] {
                    l
                } else {
                    panic!("c not i32")
                },
                v: if let FieldValue::I32(l) = row.field[3] {
                    l
                } else {
                    panic!("v not i32")
                },
                t: if let FieldValue::Str(l) = &row.field[4] {
                    l.to_string()
                } else {
                    panic!("t not string")
                },
            });
        }
    }
    Ok(rows)
}

pub async fn embed_verses() {
    // let mut rows_embed: Vec<Embed> = vec![];
    for row in process_json_file("t_ylt.json").unwrap() {
        // rows_embed.push(Embed {
        //     i: row.i,
        //     v: embed(&row.t).unwrap(),
        // });
        reqwest::Client::new().put(format!("{}/collections/i/points", std::env::var("QDRANT_URL").unwrap())).bearer_auth(std::env::var("QDRANT_KEY").unwrap()).body(format!(r#"{{points: [{{"id":"{}", "payload": {{"b": {}, "c": {}, "v": {}}}, "vector": {}, }}]}}"#, row.i, row.b, row.c, row.v, serde_json::to_string(&embed(&row.t).unwrap()).unwrap())).send().await.unwrap();
        println!("{}", row.i)
    }
    // std::fs::write(
    //     "verses_embed.json",
    //     serde_json::to_string_pretty(&rows_embed).unwrap(),
    // )
    // .unwrap();
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
