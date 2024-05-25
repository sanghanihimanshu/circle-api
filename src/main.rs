use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};

// local
mod database;
mod routes;
mod utils;
mod types;
use crate::routes::gql::graphql;
use crate::routes::version::no;

//gql
mod schemas;
use crate::database::connection::connect;
use crate::schemas::schema::create_schema;
use crate::types::apperror::{AppError,Result};
use crate::utils::context::Context;

//server function
#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let circle_db = connect().await?;
    circle_db.use_ns("circle").use_db("circle").await?;
    let context = Context { db: circle_db };
    log::info!("DataBase is running 🚀");
    log::info!("Starting on Port: http://localhost:{}", 8080);
    log::info!("Graphql running on: http://localhost:{}/api", 8080);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(context.clone()))
            .app_data(web::Data::from(
                std::sync::Arc::new(create_schema()).clone(),
            ))
            .service(graphql)
            // .service(graphql_playground)
            .service(no)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .bind(("localhost", 8080))?
    .run()
    .await
    .map_err(AppError::from)
}
