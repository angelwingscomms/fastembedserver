use anyhow::Error;
use fastembed::{Embedding, EmbeddingModel, InitOptions, TextEmbedding};

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
