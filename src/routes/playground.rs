use actix_web::{get, Responder};
use actix_web_lab::respond::Html;
use juniper::http::graphiql::graphiql_source;

/// Playground
#[get("/graphiql")]
pub async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/api", None))
}
