use juniper::{GraphQLObject, ID};
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};



#[derive(GraphQLObject,Serialize,Deserialize)]
#[graphql(description = "Video Detail")]
pub struct Video {
    pub id:ID,
    pub title: String,
    pub description: Option<String>,
    pub channel_id: ID,
    pub changed_at: String, 
    pub tags: Option<Vec<String>>,
    pub status: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Video input")]
pub struct VideoQueryInput {
    pub title: String,
    pub channel_id: String,
    pub tags: Option<Vec<String>>,
    pub status: String,
}
#[derive(GraphQLInputObject)]
#[graphql(description = "Video input")]
pub struct VideoMutationInput {
    pub title: String,
    pub channel_id: ID,
    pub tags: Option<Vec<String>>,
    pub status: Option<String>,
    pub description:String

}
#[derive(Debug, serde::Deserialize, Serialize)]
pub struct NewVideo{
    pub title: String,
    pub channel_id: Thing,
    pub tags: Option<Vec<String>>,
    pub status: Option<String>,
    pub description:String

}
#[derive(Debug, Deserialize,Clone)]

pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
    pub title: String,
    pub description: Option<String>,
    pub channel_id: Thing,
    pub changed_at: String, 
    pub tags: Option<Vec<String>>,
    pub status: String,
}
impl Video {
    pub fn new(new_video:Record)->Self{
        Video {
            id: ID::new(new_video.id.id.clone().to_string()),
            changed_at: new_video.changed_at.clone().to_string(),
            channel_id: ID::new(new_video.channel_id.id.clone().to_string()),
            status: new_video.status.clone().to_string(),
            title: new_video.title.clone().to_string(),
            description:new_video.description.clone(),
            tags:new_video.tags.clone()
        }
    }
}

