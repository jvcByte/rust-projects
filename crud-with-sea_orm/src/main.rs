use actix_web::{App, HttpResponse, HttpServer, Responder, middleware::Logger, web};
use dotenvy::dotenv;
use env_logger::Env;
use log::{error, info};
use std::env;

mod api;
mod postgres;
mod shared;

use crate::shared::AppState;

/// Simple health-check endpoint so you can verify the server is running.
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env (if present) and initialize logging.
    dotenv().ok();
    let env = Env::default().filter_or("RUST_LOG", "info");
    env_logger::Builder::from_env(env).init();

    // Initialize DB connection via the postgres module. This requires the
    // `DATABASE_URL` environment variable to be set. No secrets are hardcoded here.
    let db = match postgres::init_db().await {
        Ok(db) => db,
        Err(e) => {
            error!("failed to initialize database: {}", e);
            // Exit with non-zero status so orchestrators/CI notice startup failure.
            std::process::exit(1);
        }
    };

    // Build application state and start server.
    let state = web::Data::new(AppState::new(db));

    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    info!("starting server at http://{}", &bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            // basic health route on root service, API routes mounted under /api
            .service(web::resource("/health").route(web::get().to(health)))
            .configure(api::routes)
    })
    .bind(bind_addr)?
    .run()
    .await
}
