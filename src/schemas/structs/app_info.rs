use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[graphql(description = "ApiInfo")]
pub struct ApiInfo {
    pub name: String,
    pub version: String,
}