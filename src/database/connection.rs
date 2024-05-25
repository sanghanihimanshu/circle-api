use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use crate::types::apperror::Result;

pub async fn connect() -> Result<Surreal<Client>> {
    let data_base = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    data_base.signin(Root {
        username: "root",
        password: "root",
    }).await?;
    Ok(data_base)
}
