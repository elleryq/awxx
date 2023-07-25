// use clap::Parser;
use clap::{arg, command, value_parser, ArgAction, Command, Arg};

use http_body_util::Empty;
use hyper::Request;
use hyper::body::Bytes;
use tokio::net::TcpStream;

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("conf_host").long("conf.host"))
        .arg(Arg::new("conf_token").long("conf.token"))
        .arg(Arg::new("conf_username").long("conf.username"))
        .arg(Arg::new("conf_password").long("conf.password"))
        .arg(Arg::new("conf_insecure").short('k').long("conf.insecure"))
        .arg(Arg::new("conf_color").long("conf.color"))
        .subcommand(Command::new("login"))
        .subcommand(Command::new("config"))
        .subcommand(Command::new("import"))
        .subcommand(Command::new("export"))
        .subcommand(Command::new("ping"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(std::path::PathBuf)),
        )
        .arg(arg!(
            -v --verbose "print debug-level logs, including requests made"
        ))
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .get_matches();
    //let args = Args::parse();
    dbg!(matches.get_one::<String>("conf_host").map(String::as_str));
}
