use std::sync::Arc;

use axum::{
    extract::{MatchedPath, Multipart, Path},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Extension, Json, Router,
};
use serde_json::{json, Value};
use tokio::{signal, sync::RwLock};
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::{
    database::{base::DatabaseInit, postgres::Postgres},
    service::{
        parse::{Config, Service},
        transaction::TransactionService,
    },
};

pub struct Server {
    parse_service: Arc<RwLock<Service>>,
    transactions_service: Arc<RwLock<TransactionService<Postgres>>>,
}

impl Server {
    pub async fn new() -> Self {
        let pg_connection_string = env!("DATABASE_URL");
        println!("Using connection string: {}", pg_connection_string);

        let mut new_pg_service = Postgres::new(pg_connection_string);
        match new_pg_service.connect().await {
            Ok(_) => (),
            Err(e) => {
                // FIX ME - this should be a proper error
                panic!("Failed to connect to database: {}", e);
            }
        }
        let t_service = Arc::new(RwLock::new(TransactionService::new(new_pg_service)));

        Self {
            parse_service: Arc::new(RwLock::new(Service::new(t_service.clone()))),
            transactions_service: t_service,
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        tracing_subscriber::registry()
            .with(EnvFilter::new("info"))
            .with(tracing_subscriber::fmt::layer())
            .init();

        let app = Router::new()
            .route("/", get("Ok"))
            .route("/upload", post(upload))
            .route("/transactions/:id", get(get_transaction))
            .route("/transactions", get(get_transactions))
            .route("/transactions/:id", delete(delete_transaction))
            .layer(Extension(self.parse_service.clone()))
            .layer(Extension(self.transactions_service.clone()))
            .layer(
                TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                    let matched_path = request.extensions().get().map(MatchedPath::as_str);
                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                    )
                }),
            );

        let listener = tokio::net::TcpListener::bind("0.0.0.0:50051")
            .await
            .expect("enable to bind to port 3000");

        tracing::debug!("Listening on {}", listener.local_addr().unwrap());

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown())
            .await
            .expect("failed to run server");

        Ok(())
    }
}

async fn shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to listen for ctrl_c event");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

enum ServerError {
    MultipartError(String),
    ParseError(String),
    ServiceError(String),
    NoValue(String),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::MultipartError(c) => (StatusCode::BAD_REQUEST, c).into_response(),
            ServerError::ParseError(c) => (StatusCode::INTERNAL_SERVER_ERROR, c).into_response(),
            ServerError::ServiceError(c) => (StatusCode::INTERNAL_SERVER_ERROR, c).into_response(),
            ServerError::NoValue(c) => (StatusCode::BAD_REQUEST, c).into_response(),
        }
    }
}

async fn upload(
    Extension(parse_service): Extension<Arc<RwLock<Service>>>,
    mut multipart: Multipart,
) -> Result<StatusCode, ServerError> {
    let ps = parse_service.write().await;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ServerError::MultipartError(e.to_string()))?
    {
        let data = field
            .text()
            .await
            .map_err(|e| ServerError::MultipartError(e.to_string()))?;

        let config = Config {
            name: "Amex".to_string(),
            date_position: 0,
            amount_position: 4,
            description_position: 1,
        };

        ps.parse_data(config, data)
            .await
            .map_err(|e| ServerError::ParseError(e.to_string()))?;
    }

    Ok(StatusCode::OK)
}

async fn get_transaction(
    Path(id): Path<String>,
    Extension(transaction_service): Extension<Arc<RwLock<TransactionService<Postgres>>>>,
) -> Result<Json<Value>, ServerError> {
    let ts = transaction_service.read().await;

    let possible_transaction = ts
        .find_transaction(&id)
        .await
        .map_err(|e| ServerError::NoValue(e.to_string()))?;

    match possible_transaction {
        Some(t) => {
            return Ok(Json(json!(t)));
        }
        None => {
            println!("Unable to find transaction for ID: {}", id);
            return Err(ServerError::NoValue(format!(
                "Unable to find transaction for ID: {}",
                id
            )));
        }
    };
}

async fn get_transactions(
    Extension(transaction_service): Extension<Arc<RwLock<TransactionService<Postgres>>>>,
) -> Result<Json<Value>, ServerError> {
    let ts = transaction_service.read().await;

    let transactions = ts
        .find_transactions()
        .await
        .map_err(|e| ServerError::ServiceError(e.to_string()))?;

    return Ok(Json(json!(transactions)));
}

async fn delete_transaction(
    Path(id): Path<String>,
    Extension(transaction_service): Extension<Arc<RwLock<TransactionService<Postgres>>>>,
) -> Result<StatusCode, ServerError> {
    let ts = transaction_service.read().await;

    let has_deleted = ts
        .delete_transaction(&id)
        .await
        .map_err(|e| ServerError::ServiceError(e.to_string()))?;

    if !has_deleted {
        return Err(ServerError::ServiceError(format!(
            "Failed to delete record with ID: {}",
            id
        )));
    }

    return Ok(StatusCode::OK);
}
