use actix_web::{route, web, HttpResponse, Responder};
use juniper::http::GraphQLRequest;

use crate::{schemas::schema::Schema, utils::context::Context};

#[route("/api", method = "GET", method = "POST")]
async fn graphql(
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
    context: web::Data<Context>,
) -> impl Responder {
    let user = data.execute(&schema,&context).await;
    HttpResponse::Ok().json(user)
}
