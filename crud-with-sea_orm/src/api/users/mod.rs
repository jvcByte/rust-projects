//! Users feature module: routes, handlers and SeaORM entity definitions.
//!
//! This file defines:
//! - a `routes` function that mounts the users endpoints under a scope (used by the top-level router)
//! - Actix handlers for CRUD operations (list, get, create, update, delete)
//! - a simple SeaORM entity definition for `users`
//!
//! Notes:
//! - Handlers expect a `web::Data<crate::AppState>` with a `db: sea_orm::DatabaseConnection` field.
//! - Error handling is intentionally simple: SeaORM errors are converted to 500 Internal Server Error responses.
//! - This is a compact, single-file example. For a larger project you may want to split entity/service/repo/handlers across files.

use actix_web::{HttpResponse, Result, web};
use serde::{Deserialize, Serialize};

use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::AppState;

/// Re-exported so `api::mod` can call `users::routes`.
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(list_users))
            .route("", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user)),
    );
}

//
// DTOs
//
#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Serialize)]
struct UserResponse {
    id: i32,
    name: String,
    email: String,
}

//
// Handlers
//
async fn list_users(state: web::Data<AppState>) -> Result<HttpResponse> {
    let db: &DatabaseConnection = &state.db;
    let users = User::find()
        .all(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("db error: {}", e)))?;

    // Map to response DTOs
    let resp: Vec<UserResponse> = users
        .into_iter()
        .map(|m| UserResponse {
            id: m.id,
            name: m.name,
            email: m.email,
        })
        .collect();

    Ok(HttpResponse::Ok().json(resp))
}

async fn get_user(path: web::Path<i32>, state: web::Data<AppState>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let db: &DatabaseConnection = &state.db;

    match User::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("db error: {}", e)))?
    {
        Some(user) => {
            let resp = UserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
            };
            Ok(HttpResponse::Ok().json(resp))
        }
        None => Ok(HttpResponse::NotFound().body(format!("user {} not found", id))),
    }
}

async fn create_user(
    body: web::Json<CreateUser>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let db: &DatabaseConnection = &state.db;

    let active = user::ActiveModel {
        // id is auto-increment primary key; leave as NotSet
        name: Set(body.name.clone()),
        email: Set(body.email.clone()),
        // created_at default handled by DB or set to None
        created_at: Set(None),
        ..Default::default()
    };

    let res = User::insert(active)
        .exec(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("db error: {}", e)))?;

    // SeaORM's InsertResult may not return the full model; fetch it back.
    let created = User::find_by_id(res.last_insert_id)
        .one(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("db error: {}", e)))?;

    match created {
        Some(m) => {
            let resp = UserResponse {
                id: m.id,
                name: m.name,
                email: m.email,
            };
            Ok(HttpResponse::Created().json(resp))
        }
        None => Ok(HttpResponse::InternalServerError().body("failed to fetch created user")),
    }
}

async fn update_user(
    path: web::Path<i32>,
    body: web::Json<UpdateUser>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let db: &DatabaseConnection = &state.db;

    // Fetch existing
    let existing = User::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("db error: {}", e)))?;

    if let Some(model) = existing {
        let mut active: user::ActiveModel = model.into();

        if let Some(name) = &body.name {
            active.name = Set(name.clone());
        }
        if let Some(email) = &body.email {
            active.email = Set(email.clone());
        }

        let updated = active
            .update(db)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(format!("db error: {}", e)))?;

        let resp = UserResponse {
            id: updated.id,
            name: updated.name,
            email: updated.email,
        };
        Ok(HttpResponse::Ok().json(resp))
    } else {
        Ok(HttpResponse::NotFound().body(format!("user {} not found", id)))
    }
}

async fn delete_user(path: web::Path<i32>, state: web::Data<AppState>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let db: &DatabaseConnection = &state.db;

    let res = User::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("db error: {}", e)))?;

    if res.rows_affected > 0 {
        Ok(HttpResponse::Ok().body(format!("deleted user {}", id)))
    } else {
        Ok(HttpResponse::NotFound().body(format!("user {} not found", id)))
    }
}

//
// SeaORM entity definition (users table)
//
pub mod user {
    use sea_orm::entity::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub name: String,
        pub email: String,
        /// Optional created_at field. The actual DB column type should match your DB (e.g. timestamptz).
        pub created_at: Option<sea_orm::prelude::DateTimeWithTimeZone>,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {}

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            panic!("No Relations")
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

/// Convenience re-export so top-level code can reference `users::User` if needed.
pub use user::Entity as User;
