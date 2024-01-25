use std::sync::{Arc, RwLock};

use axum::{
    extract::Multipart,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Router,
};

use crate::{
    database::redis::Redis,
    service::{
        parse::{Config, Service},
        transaction::TransactionService,
    },
};

pub struct Server {
    parse_service: Arc<RwLock<Service>>,
    transactions_service: Arc<RwLock<TransactionService<Redis>>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            parse_service: Arc::new(RwLock::new(Service::new())),
            transactions_service: Arc::new(RwLock::new(TransactionService::new(Redis::new()))),
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        let app = Router::new()
            .route("/", get("Ok"))
            .route("/upload", post(upload))
            .layer(Extension(self.parse_service.clone()))
            .layer(Extension(self.transactions_service.clone()));

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

enum UploadError {
    MultipartError(String),
    ParseError(String),
    ServiceError(String),
}

impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        match self {
            UploadError::MultipartError(_) => StatusCode::BAD_REQUEST.into_response(),
            UploadError::ParseError(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            UploadError::ServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

async fn upload(
    Extension(parse_service): Extension<Arc<RwLock<Service>>>,
    mut multipart: Multipart,
) -> Result<StatusCode, UploadError> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| UploadError::MultipartError(e.to_string()))?
    {
        if let Some(filename) = field.file_name() {
            println!("filename: {}", filename)
        }

        // need to handle errors here and return responses to client
        let data = field
            .text()
            .await
            .map_err(|e| UploadError::MultipartError(e.to_string()))?;

        let ps = parse_service
            .write()
            .map_err(|e| UploadError::ServiceError(e.to_string()))?;

        let config = Config {
            name: "Amex".to_string(),
            date_position: 0,
            amount_position: 4,
            description_position: 1,
        };

        ps.parse_data(config, data)
            .map_err(|e| UploadError::ParseError(e.to_string()))?;
    }

    Ok(StatusCode::OK)
}
