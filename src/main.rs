use fastembedserver::embed;
use serde::{Deserialize, Serialize};
use warp::{Filter, Reply};

#[derive(Deserialize)]
struct Input {
    input: String,
    model: Option<String>,
    format: Option<String>,
    dimensions: Option<usize>,
    params: Option<std::collections::HashMap<String, String>>,
}

#[derive(Deserialize)]
struct QueryInput {
    q: String,
}

#[derive(Serialize)]
struct ApiResponse {
    object: &'static str,
    data: Vec<VecEmbedding>,
    model: &'static str,
}

#[derive(Serialize)]
struct VecEmbedding {
    object: &'static str,
    embedding: Vec<f32>,
    index: usize,
}

#[shuttle_runtime::main]
async fn warp() -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    let json_route = warp::post()
        .and(warp::path!("embeddings"))
        .and(warp::body::json())
        .map(|input: Input| {
            let embedding = embed(&input.input).unwrap_or_default();
            let response = ApiResponse {
                object: "list",
                data: vec![VecEmbedding {
                    object: "embedding",
                    embedding,
                    index: 0,
                }],
                model: "text-embedding-ada-002",
            };
            warp::reply::json(&response)
        });

    let json_buffer_route = warp::post()
        .and(warp::path!("embeddings"))
        .and(warp::header("b"))
        .and(warp::body::json())
        .map(|_: String, input: Input| {
            let embedding = embed(&input.input).unwrap_or_default();
            let response = ApiResponse {
                object: "list",
                data: vec![VecEmbedding {
                    object: "embedding",
                    embedding,
                    index: 0,
                }],
                model: "mxbai-embed-large-v1",
            };
            warp::reply::json(&response)
        });

    let json_plain_route = warp::post()
        .and(warp::path!("embeddings"))
        .and(warp::body::json())
        .map(|input: Input| warp::reply::json(&embed(&input.input).unwrap_or_default()));

    let query_route =
        warp::get()
            .and(warp::query::query::<QueryInput>())
            .map(|query_input: QueryInput| {
                warp::reply::json(&embed(&query_input.q).unwrap_or_default())
            });

    let query_buffer_route =
        warp::get()
            .and(warp::header("b"))
            .and(warp::query::query::<QueryInput>())
            .map(|_: String, query_input: QueryInput| {
                warp::reply::json(&embed(&query_input.q).unwrap_or_default())
            });

    Ok(json_route
        .or(json_buffer_route)
        .or(json_plain_route)
        .or(query_route)
        .or(query_buffer_route)
        .boxed()
        .into())
}
