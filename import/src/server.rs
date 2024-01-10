use axum::{
    extract::Multipart,
    http::StatusCode,
    routing::{get, post},
    Router,
};

use crate::service::service::Service;

pub struct Server {
    parse_server: Service,
}

impl Server {
    pub fn new() -> Self {
        Self {
            parse_server: Service::new(),
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        let app = Router::new()
            .route("/", get("Ok"))
            .route("/upload", post(upload));

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .expect("enable to bind to port 3000");

        axum::serve(listener, app)
            .await
            .expect("failed to run server");

        Ok(())
    }
}

async fn upload(multipart: Multipart) -> Result<StatusCode, ()> {
    Ok(StatusCode::OK)
}
