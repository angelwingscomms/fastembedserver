use std::collections::HashMap;

use fastembedserver::embed;
use warp::{Filter, Reply};

#[shuttle_runtime::main]
async fn warp() -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    let route = warp::any()
        .and(warp::query::query())
        .map(|query: HashMap<String, String>| {
            let default_query = "".to_string();
            let q = query.get("q").unwrap_or(&default_query);

            let embedding = embed(&q.clone()).unwrap_or_default();
            warp::reply::json(&embedding)
        });

    Ok(route.boxed().into())
}