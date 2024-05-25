use juniper::{FieldResult, RootNode};

use crate::utils::{date::date_time_now,context::Context};

use super::structs::{
    app_info::ApiInfo,
    user::{User, UserQueryInput},
    video::{Video, VideoQueryInput},
};

pub struct Query;
pub struct Mutation;
pub struct Subscription;
#[juniper::graphql_object]
#[graphql(ctx = Context)]
impl Query {
    fn api_version() -> FieldResult<ApiInfo> {
        Ok(ApiInfo {
            name: String::from("Ice-lake"),
            version: String::from("0.1v"),
        })
    }
    async fn get_video(_ctx: &Context, video_input: VideoQueryInput) -> FieldResult<Video> {
        let video = Video {
            _id: String::from("video_id_123"),
            title: video_input.title,
            description: Some(String::from("This is an example video description.")),
            uploader_id: video_input.uploader_id,
            upload_date: date_time_now(),
            thumbnail_url: Some(String::from("https://example.com/thumbnail.jpg")),
            views: Some(100),
            likes: Some(10),
            dislikes: Some(1),
            video_thread_id: Some(String::from("thread_id_456")),
            tags: video_input.tags.into(),
            duration: 3600,
            resolution: String::from("1080p"),
            categories: vec![String::from("Category1"), String::from("Category2")].into(),
            status: video_input.status,
        };
        Ok(video)
    }
}

// mutation
#[juniper::graphql_object]
#[graphql(ctx = Context)]
impl Mutation {
    async fn create_user(user_input: UserQueryInput) -> FieldResult<User> {
        let new_user = User::new(user_input.username, user_input.email, user_input.password);
        Ok(new_user)
    }
}

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
