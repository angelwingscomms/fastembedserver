// use std::collections::HashMap;
use warp::{Filter, Reply};
use serde::Deserialize;

use fastembedserver::embed;
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

#[shuttle_runtime::main]
async fn warp() -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    let json_route = warp::post()
        .and(warp::path!("embeddings"))
        .and(warp::body::json())
        .map(|input: Input| {
            let embedding = embed(&input.input).unwrap_or_default();
            println!("{}", embedding.len());
            warp::reply::json(&embedding)
        });
    let query_route = warp::get()
        .and(warp::query::query::<QueryInput>())
        .map(|query_input: QueryInput| {
            let embedding = embed(&query_input.q).unwrap_or_default();
            println!("{}", embedding.len());
            warp::reply::json(&embedding)
        });
    Ok(json_route.or(query_route).boxed().into())
}

// pub fn main() {
//     let all = process_json_file("t_ylt.json").unwrap();
//     println!("{:#?}", all)
// }