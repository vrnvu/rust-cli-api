use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::handlers::state::AppState;

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
pub async fn handler(info: web::Json<Url>, state: Data<AppState>) -> impl Responder {
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
