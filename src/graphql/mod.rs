//
// graphql.rs
//

use crate::db::{ConnectionPool};
use juniper::{FieldResult,FieldError};

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub struct Ctx {
    pub pool: ConnectionPool,
}

pub struct Query;
pub struct Mutation;

graphql_object!(Query: Ctx |&self| {
    field viewer(&executor) -> FieldResult<String> {
        Ok("World".into())
        //let conn = executor.context().pool.get().unwrap();
        //let current_user = executor.context().user.clone();
        //let lookahead = executor.look_ahead();
        //viewer::current(conn, current_user, lookahead)
            //.map_err(|e| FieldError::from(e))
    }
});

graphql_object!(Mutation: Ctx |&self| {
    field login(&executor, input: String) -> FieldResult<String> {
        Ok("Hi".into())
    }
});
