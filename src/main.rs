use axum::{middleware, Router};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use tokio::net::TcpListener;
use crate::error::Result;
use crate::web::routes_files::handler_convert_file;


mod error;

mod web;


#[tokio::main]
pub async fn main() -> Result<()> {


    let route_test_server = Router::new()
        .route("/test_server", get(handler_answer_test_server));

    let route_convert = Router::new()
        .route("/upload/:output_format", post(handler_convert_file));

    let routes_all = Router::new()
        .merge(route_test_server)
        .merge(route_convert)
        .layer(middleware::map_response(main_response_mapper));

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("-->>LISTENING on {:?}", listener.local_addr().unwrap());

    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("-->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

async fn handler_answer_test_server() -> impl IntoResponse {
    println!("-->> {:<12} - answer_server", "HANDLER");

    Html("TEST DONE")
}




