#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod common;

#[test]
fn test_staging_config() {
    common::test_config(rocket::config::Environment::Staging);
}
