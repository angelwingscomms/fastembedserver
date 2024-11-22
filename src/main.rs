use fastembedserver::{embed, embed_verses, process_json_file};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::task;
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

async fn embed_verses_f() -> anyhow::Result<()> {
    for row in process_json_file("t_ylt.json")? {
        let res = reqwest::Client::new().put(format!("{}/collections/i/points?wait=true", std::env::var("QDRANT_URL")?)).bearer_auth(std::env::var("QDRANT_KEY")?).body(format!(r#"{{points: [{{"id":"{}", "payload": {{"b": {}, "c": {}, "v": {}}}, "vector": {}, }}]}}"#, row.i, row.b, row.c, row.v, serde_json::to_string(&embed(&row.t)?)?)).send().await?;
        println!("{} res: {:#?}", row.i, res);
    }
    Ok(())
}

async fn embed_verses_handler() -> impl Reply {
    task::spawn(async {
        embed_verses_f().await.unwrap();
    });
    warp::reply::with_status("", warp::http::StatusCode::OK)
}

#[shuttle_runtime::main]
async fn warp() -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    dotenv::dotenv().ok();
    let embed_verses_route = warp::get()
        .and(warp::path!("embed_verses"))
        .then(embed_verses_handler);

    // maybe buffer routes

    let json_route = warp::post()
        .and(warp::path!("embeddings"))
        .and(warp::body::json())
        .map(|i: Input| match embed(&i.input) {
            Ok(embedding) => {
                let response = ApiResponse {
                    object: "list",
                    data: vec![VecEmbedding {
                        object: "embedding",
                        embedding,
                        index: 0,
                    }],
                    model: "mxbai-embed-large-v1",
                };
                warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::OK)
            }
            Err(e) => warp::reply::with_status(
                warp::reply::json(&e.to_string()),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
        });

    let json_plain_route = warp::path::end()
        .and(warp::post())
        .and(warp::body::json())
        .map(|i: Input| match embed(&i.input) {
            Ok(e) => warp::reply::with_status(warp::reply::json(&e), warp::http::StatusCode::OK),
            Err(e) => warp::reply::with_status(
                warp::reply::json(&e.to_string()),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
        });

    let query_route = warp::get()
        .and(warp::query::query::<QueryInput>())
        .map(|q: QueryInput| match embed(&q.q) {
            Ok(e) => warp::reply::with_status(warp::reply::json(&e), warp::http::StatusCode::OK),
            Err(e) => warp::reply::with_status(
                warp::reply::json(&e.to_string()),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
        });

    Ok(json_route
        .or(embed_verses_route)
        .or(json_plain_route)
        .or(query_route)
        .boxed()
        .into())
}
