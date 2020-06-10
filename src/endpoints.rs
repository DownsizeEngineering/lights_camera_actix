use serde::Deserialize;
use actix_web::{HttpResponse, Responder, web, Result, get};
use super::AppState;

pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello {}!", app_name))
}
// pub async fn index2() -> impl Responder {
//     HttpResponse::Ok().body("Hello Universe!")
// }

#[get("/hi")]
pub async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hi user!")
}

#[derive(Deserialize)]
pub struct Greeting {
    name: String,
    greeting: String
}


pub async fn url_parser(info: web::Path<Greeting>) -> Result<String> {
    Ok(format!("{}, {}!", info.greeting, info.name))
}