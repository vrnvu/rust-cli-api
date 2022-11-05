mod handlers;
use handlers::shortener;

use std::{collections::HashMap, sync::Mutex};

use actix_web::{middleware::Logger, web, App, HttpServer};

#[macro_use]
extern crate log;
use env_logger::Env;

struct AppState {
    counter: Mutex<u32>,
    map: Mutex<HashMap<String, String>>,
}

impl AppState {
    async fn insert(self: &Self, k: String, v: String) {
        let mut map = self.map.lock().unwrap();
        map.insert(k, v);
    }

    async fn get(self: &Self, k: &String) -> Option<String> {
        let map = self.map.lock().unwrap();
        map.get(k).map_or_else(|| None, |v| Some(v.to_owned()))
    }

    async fn inc(self: &Self) {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
        info!("{:?}", counter);
    }

    async fn dec(self: &Self) {
        let mut counter = self.counter.lock().unwrap();
        *counter -= 1;
        info!("{:?}", counter);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
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
