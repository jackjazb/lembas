use entity::{
    async_graphql::{self, Context},
    prelude, recipe,
    sea_orm::{DatabaseConnection, EntityTrait},
};
use migration::DbErr;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        "Hello GraphQL".to_owned()
    }

    async fn recipes(&self, ctx: &Context<'_>) -> Result<Vec<recipe::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        prelude::Recipe::find().all(db).await
    }
}
