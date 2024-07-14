use std::collections::HashMap;

use fastembedserver::embed;
use serde::{Deserialize, Serialize};
use warp::{Filter, Reply};

// Define a struct to hold the embedding response
#[derive(Serialize, Deserialize)]
struct EmbeddingResponse {
    embedding: Vec<f32>,
}

#[shuttle_runtime::main]
async fn warp() -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    let route = warp::any()
        .and(warp::query::query())
        .map(|query: HashMap<String, String>| {
            let default_query = "".to_string();
            let q = query.get("q").unwrap_or(&default_query);

            let embedding = embed(&q.clone()).unwrap_or_default();
            let response = EmbeddingResponse { embedding };
            warp::reply::json(&response)
        });

    Ok(route.boxed().into())
}
