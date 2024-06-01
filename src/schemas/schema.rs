use juniper::{FieldResult, RootNode};

use crate::utils::context::Context;


pub struct Query;
pub struct Mutation;
pub struct Subscription;

#[juniper::graphql_object]
#[graphql(ctx = Context)]
impl Subscription {
    async fn new() -> FieldResult<String> {
        Ok(String::from(""))
    }
}

pub type Schema = RootNode<'static, Query, Mutation, Subscription>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, Subscription {})
}
