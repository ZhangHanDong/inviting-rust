use std::net::SocketAddr;
use bytes::Bytes;
use axumlike02::{
    handler::get, Router, response::IntoResponse, 
    extract::{Query, TypedHeader},
    http::StatusCode
};
use tower::{BoxError, ServiceBuilder};
use tower_http::set_header::SetRequestHeaderLayer;
use std::time::Duration;
use hyper::Body;
use http::{Request, Response, HeaderValue, header::USER_AGENT};
use color_eyre::Report;
use tracing::info;
use tracing_subscriber::EnvFilter;
use std::convert::Infallible;

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    info!("Axumlike init ...");
    // build our application with a route
    let app = 
        Router::new()
            .route("/", get(handler))
            // curl http://127.0.0.1:3000/page?page=2&per_page=30
            .route("/page", get(page_handler))
            .layer(SetRequestHeaderLayer::<_, Body>::overriding(
                USER_AGENT,
                HeaderValue::from_static("tower-http demo")
            ));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axumlike02::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn handler(user_agent: Option<TypedHeader<headers::UserAgent>>) -> impl IntoResponse {
    let url = "localhost";
    if let Some(TypedHeader(user_agent)) = user_agent {
        info!(%url, content_type = ?user_agent.as_str(), "Got a connection!");
    }
    
    let res = "<h1>Hello, World!</h1>".into_response();
    info!(%url, content_type = ?res.headers().get(USER_AGENT), "Got a response!");
    res

}

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Pagination {
    page: usize,
    per_page: usize,
}

async fn page_handler(pagination: Query<Pagination>) -> &'static str {
    let url = "localhost";
    let pagination: Pagination = pagination.0;

    info!(?pagination,  "Got a connection!");
    
    "<h1>Hello, World!</h1>"
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
