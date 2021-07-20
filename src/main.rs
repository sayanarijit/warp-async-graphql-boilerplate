use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_warp::Response;
use std::convert::Infallible;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Response as HttpResponse, Filter};

use store::graph;

#[tokio::main]
async fn main() {
    std::env::var("RUST_LOG")
        .map(|filter| {
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .with_span_events(FmtSpan::CLOSE)
                .init();
        })
        .unwrap_or_default();

    let graphql = async_graphql_warp::graphql(graph::build()).and_then(
        |(schema, request): (graph::Schema, async_graphql::Request)| async move {
            Ok::<_, Infallible>(Response::from(schema.execute(request).await))
        },
    );

    let playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    println!("Playground: http://localhost:8080");

    let routes = playground.or(graphql).with(warp::trace::request());

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
