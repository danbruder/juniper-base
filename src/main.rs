#![feature(decl_macro, proc_macro_hygiene, custom_attribute)]
#![allow(proc_macro_derive_resolution_fallback)]

extern crate serde;
extern crate serde_json;
extern crate dotenv;
extern crate juniper_rocket;
extern crate rocket_contrib;
extern crate validator;

#[macro_use] extern crate rocket;
#[macro_use] extern crate juniper;
#[macro_use] extern crate juniper_codegen;
#[macro_use] extern crate diesel;
#[macro_use] extern crate validator_derive;

mod schema;
mod error;
mod db;
mod web;
mod graphql;

fn main() {
    web::launch();
}
