use clap::{App as args, Arg, ArgMatches};
use failure::Error;
use std::string::String;

// general configuration across all the app.
#[derive(Clone, Debug)]
pub struct Configuration {
    pub redis_address: String,
    pub server: Server,
}

// configuration for the underlying http(s) server.
#[derive(Clone, Debug)]
pub struct Server {
    pub address: String,
    pub port: String,
    pub folder: String,
}

pub fn configuration() -> Result<Configuration, Error> {
    let cli_app = cli();
    from_cli(cli_app)
}

fn from_cli(matches: ArgMatches<'static>) -> Result<Configuration, Error> {
    let cache = matches
        .value_of("redis.url")
        .map(|url| url.to_string())
        .unwrap_or("redis://127.0.0.1:6379".to_string());

    let port = matches
        .value_of("server.port")
        .map(|port| port.to_string())
        .unwrap_or("8000".to_string());

    let address = matches
        .value_of("server.address")
        .map(|address| address.to_string())
        .unwrap_or("127.0.0.1".to_string());

    let folder = matches
        .value_of("server.files")
        .map(|path| path.to_string())
        .unwrap_or("static".to_string());

    Ok(Configuration {
        redis_address: cache,
        server: Server {
            address,
            port,
            folder,
        },
    })
}

fn cli() -> ArgMatches<'static> {
    args::new("hashing")
        .version("0.1")
        .about("a redis http toy server")
        .arg(
            Arg::with_name("server.address")
                .short("a")
                .help("address, the server will listen")
                .required(true)
                .default_value("127.0.0.1"),
        )
        .arg(
            Arg::with_name("server.port")
                .short("p")
                .help("port, the server will listen")
                .required(true)
                .default_value("8000"),
        )
        .arg(
            Arg::with_name("server.files")
                .short("f")
                .help("(relative/absolute) path to static files")
                .required(true)
                .default_value("static"),
        )
        .arg(
            Arg::with_name("redis.url")
                .short("r")
                .help("address to redis")
                .required(true)
                .default_value("redis://127.0.0.1:6379"),
        )
        .get_matches()
}
