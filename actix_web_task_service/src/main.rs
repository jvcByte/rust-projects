mod api;
mod repo;
use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use api::task::get_task;
use repo::ddb::DDBRepository;

#[actix_web::main]
async fn main() -> Result<()> {
    let env = env_logger::Env::default().filter_or("RUST_LOG", "debug");
    env_logger::Builder::from_env(env).init();
    let config = aws_config::load_from_env().await;

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).app_data().service(get_task)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
