extern crate actix;
extern crate actix_web;

extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

extern crate json;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate num_cpus;

extern crate bytes;
extern crate clap;
extern crate futures;
extern crate rusty_ulid;

#[macro_use]
extern crate failure;

mod application;
mod configuration;
mod models;
mod server;
mod store;

use failure::Error;

fn main() -> Result<(), Error> {
    application::run_app()
}
