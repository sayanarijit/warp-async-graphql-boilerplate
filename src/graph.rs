use async_graphql::{Context, Object, Result};
use async_graphql::{EmptySubscription, Schema as GQLSchema};

use crate::db;

pub type Schema = GQLSchema<Query, Mutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    async fn data<'a>(&self, ctx: &Context<'a>) -> Result<&'a String> {
        ctx.data()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn mutate(&self) -> Result<i32> {
        let conn = db::get_connection()?;
        let post = db::create_post(&conn, "title", "body")?;
        Ok(post.id)
    }
}

pub fn build() -> Schema {
    GQLSchema::build(Query, Mutation, EmptySubscription)
        .data(String::from("foo"))
        .limit_complexity(500)
        .limit_depth(500)
        .finish()
}
