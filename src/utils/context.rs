use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Clone)]
pub struct Context {
    pub db: Surreal<Client>,
}

impl juniper::Context for Context {}