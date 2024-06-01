use juniper::FieldResult;
use serde::Deserialize;
use surrealdb::sql::Thing;

use crate::{
    schemas::{
        schema::Mutation,
        structs::{
            user::{User, UserQueryInput},
            video::{NewVideo, Record, Video, VideoMutationInput},
        },
    },
    utils::context::Context,
};


#[juniper::graphql_object]
#[graphql(ctx = Context)]
impl Mutation {
    async fn create_video(ctx: &Context, video_input: VideoMutationInput) -> FieldResult<Option<Video>> {
        let new_video: Vec<Record> = ctx
            .db
            .create("videos")
            .content(NewVideo{
                title:video_input.title,
                channel_id:Thing {
                    id: surrealdb::sql::Id::String(video_input.channel_id.to_string()),
                    tb: "channel".to_string(),
                },
                tags:video_input.tags,
                status:video_input.status,
                description:video_input.description,
    })
            .await?;
        Ok(Some(Video::new(new_video[0].clone())))
    }
}
