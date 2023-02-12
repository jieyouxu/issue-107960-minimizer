use std::{convert::Infallible, env, net::SocketAddr};
use std::sync::Arc;

use hyper::{body::Body, Method, Request, Response, StatusCode};
use hyper::body::Bytes;
use hyper::Server;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use tokio::sync::RwLock;
use tokio::net::TcpListener;

use crate::ConfigStruct;

async fn handle_404() -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let mut not_found = Response::new("404 not found");
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok(not_found)
}

async fn handle_req(global_config: &Arc<RwLock<ConfigStruct>>, req: Request<hyper::body::Incoming>)
-> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>  {
    println!("got req {} {}", req.uri(), req.method());
    match (req.method(), req.uri().path()) {
        (_, _) => handle_404().await
    }
}

pub async fn start_web_server(global_config: Arc<RwLock<ConfigStruct>>) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(|r| handle_req(&global_config, r) ))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}