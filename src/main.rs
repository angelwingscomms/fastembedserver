use fastembedserver::{embed, embed_verses, process_json_file};
use serde::{Deserialize, Serialize};
use warp::{reply::WithStatus, Filter, Reply};

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

async fn embed_verses_handler() -> impl Reply {
    for row in process_json_file("t_ylt.json").unwrap() {
        reqwest::Client::new().put(format!("{}/collections/i/points", std::env::var("QDRANT_URL").unwrap())).bearer_auth(std::env::var("QDRANT_KEY").unwrap()).body(format!(r#"{{points: [{{"id":"{}", "payload": {{"b": {}, "c": {}, "v": {}}}, "vector": {}, }}]}}"#, row.i, row.b, row.c, row.v, serde_json::to_string(&embed(&row.t).unwrap()).unwrap())).send().await.unwrap();
        println!("{}", row.i)
    }
    warp::reply::with_status("", warp::http::StatusCode::OK)
}

#[shuttle_runtime::main]
async fn warp() -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    let embed_verses_route = warp::get()
        .and(warp::path!("embed_verses"))
        .then(embed_verses_handler);
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

    let json_plain_route = warp::path::end()
        .and(warp::post())
        .and(warp::body::json())
        .map(|input: Input| warp::reply::json(&embed(&input.input).unwrap_or_default()));

    let query_route =
        warp::get()
            .and(warp::query::query::<QueryInput>())
            .map(|query_input: QueryInput| {
                warp::reply::json(&embed(&query_input.q).unwrap_or_default())
            });

    let query_buffer_route = warp::get()
        .and(warp::header("b"))
        .and(warp::query::query::<QueryInput>())
        .map(|_: String, query_input: QueryInput| {
            warp::reply::json(&embed(&query_input.q).unwrap_or_default())
        });

    Ok(json_route
        .or(embed_verses_route)
        .or(json_buffer_route)
        .or(json_plain_route)
        .or(query_route)
        .or(query_buffer_route)
        .boxed()
        .into())
}
