use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use crate::types::apperror::Result;
use std::env::var;
pub async fn connect() -> Result<Surreal<Client>> {
    let data_base = Surreal::new::<Ws>(var("DATABASE_URL").expect("DATABASE_URL must be set.")).await?;
    data_base.signin(Root {
        username: &var("DATABASE_USERNAME").expect("DATABASE_USERNAME must be set."),
        password: &var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set."),
    }).await?;
    Ok(data_base)
}
