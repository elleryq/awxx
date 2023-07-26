// use clap::Parser;
use clap::{arg, command, value_parser, Command, Arg, ArgAction};


// fn ping(url: &str) -> Result<(), Box<dyn std::error::Error>> {
fn ping(url: &str) {
    let resp = reqwest::blocking::get(url);
    dbg!("ping");
    println!("hello");
    println!("{:?}", resp);
    // Ok(())
}

fn main() {
    println!("Hello world");
    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("conf_host").long("conf.host").default_value("https://localhost"))
        .arg(Arg::new("conf_token").long("conf.token"))
        .arg(Arg::new("conf_username").long("conf.username"))
        .arg(Arg::new("conf_password").long("conf.password"))
        .arg(Arg::new("conf_insecure").short('k').long("conf.insecure").action(ArgAction::SetTrue))
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

    let conf_host = if let Some(conf_host) = matches.get_one::<String>("conf_host").map(String::as_str) { conf_host } else { "" };
    // println!("conf_host = {}", conf_host);
    if let Some(matches) = matches.subcommand_matches("ping") {
        let conf_host = conf_host;
        ping("https://ansible.cloudns.pro");
        println!("conf_host = {}", conf_host);
    }
}
