use env_logger::Env;
use rust_cli_api::handlers::{shortener, state};

use std::{collections::HashMap, sync::Mutex};

use actix_web::{middleware::Logger, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(state::AppState {
        counter: Mutex::new(0),
        map: Mutex::new(HashMap::new()),
    });

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(state.clone())
            .service(shortener::handler)
            .service(shortener::redirect)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
