use actix_web::{get, web, Responder, Result};
use serde::Serialize;
#[derive(Serialize)]
struct ApiInfo {
    name: String,
    version: String,
}

#[get("/")]
pub async fn no() -> Result<impl Responder> {
    Ok(web::Json(ApiInfo {
        name: String::from("Icelake"),
        version: String::from("0.1v"),
    }))
}
