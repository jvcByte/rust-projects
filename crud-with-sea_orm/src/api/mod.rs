//! API root module — wires feature routes under a common `/api` prefix.
//!
//! This module exposes a single `routes` function that your `main` can pass to
//! `App::configure(...)`. It delegates to feature modules (e.g. `users`) which
//! should each provide their own `pub fn routes(cfg: &mut web::ServiceConfig)`.

use actix_web::web;

pub mod users;

/// Mount all API routes under `/api`.
///
/// Example usage from `main.rs`:
///     .configure(crud_with_sea_orm::api::routes)
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Keep the API surface stable by grouping feature scopes under `/api`.
            // Each feature module (e.g. `users`) should expose `routes`.
            .configure(users::routes),
    );
}
