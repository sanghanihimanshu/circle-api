use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use routes::playground::graphql_playground;

// local
mod database;
mod routes;
mod utils;
mod types;
mod schemas;

//gql
use crate::routes::gql::graphql;
use crate::routes::version::no;
use crate::database::connection::connect;
use crate::schemas::schema::create_schema;
use crate::types::apperror::{AppError,Result};
use crate::utils::context::Context;

//server function
#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let port= std::env::var("HTTP_PORT").expect("HTTP_PORT must be set.").parse::<u16>().unwrap();
    let circle_db = connect().await?;
    circle_db.use_ns("circle").use_db("circle").await?;
    let context = Context { db: circle_db };
    log::info!("DataBase is running 🚀");
    log::info!("Starting on Port: http://localhost:{}", port);
    log::info!("Graphql running on: http://localhost:{}/api", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(context.clone()))
            .app_data(web::Data::from(
                std::sync::Arc::new(create_schema()).clone(),
            ))
            .service(graphql)
            .service(graphql_playground)
            .service(no)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .bind(("localhost", port))?
    .run()
    .await
    .map_err(AppError::from)
}
