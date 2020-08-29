#[macro_use]
extern crate failure;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

mod application;
mod configuration;
mod models;
mod server;
mod store;

use failure::Error;

fn main() -> Result<(), Error> {
    application::run_app()
}
