use log::info;
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::error::Error;
use std::io;

/// Initialize a `DatabaseConnection` using the `DATABASE_URL` environment variable.
///
/// This function requires `DATABASE_URL` to be set in the environment. It will
/// return an error if the variable is missing. This avoids embedding secrets
/// in source code or falling back to hardcoded credentials.
pub async fn init_db() -> Result<DatabaseConnection, Box<dyn Error + Send + Sync>> {
    let database_url = env::var("DATABASE_URL").map_err(|_| {
        Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "DATABASE_URL environment variable must be set",
        )) as Box<dyn Error + Send + Sync>
    })?;

    info!(
        "Connecting to database: {}",
        redact_url_password(&database_url)
    );
    let db = Database::connect(&database_url)
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
    info!("Database connected");
    Ok(db)
}

/// Redact the password in a database URL for less noisy logging.
/// This is a simple, best-effort helper that hides the password portion if present.
///
/// It does not fully parse every possible DSN format; it's intended to be a safe
/// default for typical `postgresql://user:password@host/...` strings.
fn redact_url_password(url: &str) -> String {
    // Find the `://` then the `@` and replace the user:pass portion between them.
    if let Some(scheme_end) = url.find("://") {
        if let Some(at_pos) = url[scheme_end + 3..].find('@') {
            let start = scheme_end + 3;
            let end = start + at_pos;
            let mut out = String::with_capacity(url.len());
            out.push_str(&url[..start]);
            out.push_str("[REDACTED]");
            out.push_str(&url[end..]);
            return out;
        }
    }
    // Fallback: return the original if we couldn't redact.
    url.to_string()
}
