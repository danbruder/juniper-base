//
// web.rs
//
pub mod cors;

use rocket::response::content;
use rocket::State;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use crate::db::{Db,create_pool};
use crate::graphql::{Query, Mutation,Ctx,Schema};

#[get("/graphql/explorer")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[options("/graphql")]
fn post_graphql_cors_handler() -> content::Plain<String> {
    content::Plain("".to_string())
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    db: State<Db>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    let connection = db.pool.get().unwrap();

    // Create new context
    let context = Ctx{
        pool: db.pool.clone(),
    };

    request.execute(&schema, &context)
}

pub fn launch() {
    rocket::ignite()
        .manage(Db { pool: create_pool()})
        .manage(Schema::new(
                Query , Mutation
        ))
        .mount(
            "/",
            routes![graphiql, post_graphql_handler, post_graphql_cors_handler],
        )
        .attach(cors::CORS())
        .launch();
}
