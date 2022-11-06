use std::{collections::HashMap, sync::Mutex};

use log::info;

use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};

use serde::Deserialize;
use uuid::Uuid;

pub struct AppState {
    pub counter: Mutex<u32>,
    pub map: Mutex<HashMap<String, String>>,
}

impl AppState {
    pub async fn insert(self: &Self, k: String, v: String) {
        let mut map = self.map.lock().unwrap();
        map.insert(k, v);
    }

    pub async fn get(self: &Self, k: &String) -> Option<String> {
        let map = self.map.lock().unwrap();
        map.get(k).map_or_else(|| None, |v| Some(v.to_owned()))
    }

    pub async fn inc(self: &Self) {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
        info!("{:?}", counter);
    }

    pub async fn dec(self: &Self) {
        let mut counter = self.counter.lock().unwrap();
        *counter -= 1;
        info!("{:?}", counter);
    }
}

#[derive(Deserialize)]
pub struct Info {
    uuid: String,
}

#[get("/id/{uuid}")]
async fn redirect(info: web::Path<Info>, state: Data<AppState>) -> impl Responder {
    state.inc().await;

    let url = state.get(&info.uuid).await;
    log::info!("map: {:?}", state.map);

    state.dec().await;
    url.map_or_else(
        || HttpResponse::NotFound().finish(),
        |u| {
            HttpResponse::MovedPermanently()
                .append_header(("location", u))
                .finish()
        },
    )
}

#[derive(Deserialize)]
pub struct Url {
    uri: String,
}

#[post("/")]
pub async fn shortener(info: web::Json<Url>, state: Data<AppState>) -> impl Responder {
    state.inc().await;

    let url = generate_url();
    state
        .insert(info.uri.to_owned(), String::from("https://www.google.com"))
        .await;
    log::info!("{:?}", state.map);

    state.dec().await;

    HttpResponse::Ok().body(url)
}

fn generate_url() -> String {
    let uuid = Uuid::new_v4().to_string();
    uuid
}
