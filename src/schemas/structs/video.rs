use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "Video Detail")]
pub struct Video {
    pub _id: String,
    pub title: String,
    pub description: Option<String>,
    pub uploader_id: String,
    pub upload_date: String, // ISO 8601 formatted date-time string
    pub thumbnail_url: Option<String>,
    pub views: Option<i32>,
    pub likes: Option<i32>,
    pub dislikes: Option<i32>,
    pub video_thread_id: Option<String>,
    pub tags: Option<Vec<String>>,
    pub duration: i32,
    pub resolution: String,
    pub categories: Option<Vec<String>>,
    pub status: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Video input")]
pub struct VideoQueryInput {
    pub title: String,
    pub uploader_id: String,
    pub tags: Option<Vec<String>>,
    pub status: String,
}
