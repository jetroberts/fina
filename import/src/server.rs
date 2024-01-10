use std::{
    io::BufRead,
    sync::{Arc, Mutex, RwLock},
};

use axum::{
    extract::Multipart,
    http::StatusCode,
    routing::{get, post},
    Extension, Router,
};

use crate::service::service::Service;

pub struct Server {
    parse_server: Arc<RwLock<Service>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            parse_server: Arc::new(RwLock::new(Service::new())),
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        let app = Router::new()
            .route("/", get("Ok"))
            .route("/upload", post(upload))
            .layer(Extension(self.parse_server.clone()));

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .expect("enable to bind to port 3000");

        println!("Listening on port 3000");
        axum::serve(listener, app)
            .await
            .expect("failed to run server");

        Ok(())
    }
}

async fn upload(
    Extension(parse_service): Extension<Arc<Mutex<Service>>>,
    mut multipart: Multipart,
) -> Result<StatusCode, ()> {
    let mut count = 0;

    while let Some(field) = multipart
        .next_field()
        .await
        .expect("failed to read multipart")
    {
        if let Some(filename) = field.file_name() {
            println!("filename: {}", filename)
        }

        if let Ok(bytes) = field.bytes().await {
            if let Ok(ps) = parse_service.lock() {
                ps.parse(bytes.as_ref());
            }
        }
    }

    Ok(StatusCode::OK)
}
