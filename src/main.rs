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

    let query_route =
        warp::get()
            .and(warp::query::query::<QueryInput>())
            .map(|query_input: QueryInput| {
                let embedding = embed(&query_input.q).unwrap_or_default();
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

    Ok(json_route.or(query_route).boxed().into())
}
// pub fn main() { let all = process_json_file("t_ylt.json").unwrap(); println!("{:#?}", all)
// }
