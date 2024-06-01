use crate::schemas::schema::Query;
use crate::schemas::structs::video::Record;
use crate::schemas::structs::{
    app_info::ApiInfo,
    video::Video,
};
use juniper::FieldResult;

use crate::utils::context::Context;

#[juniper::graphql_object]
#[graphql(ctx = Context)]
impl Query {
    fn api_version() -> FieldResult<ApiInfo> {
        Ok(ApiInfo {
            name: String::from("Ice-lake"),
            version: String::from("0.1v"),
        })
    }
    async fn get_video(ctx: &Context) -> FieldResult<Option<Vec<Video>>> {
        let videos:Vec<Record> = ctx.db.select("videos").await?;
        Ok(Some(videos.into_iter().map(Video::new).collect()))
    }
}
