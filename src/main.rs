use clap::{arg, command, value_parser, ArgAction, Command};
use errors::Error;
use std::path::PathBuf;
mod errors;
mod parsers;
fn main() {
    let matches = command!()
        .arg(
            arg!( -c --config <FILE> "The config file path")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .subcommand(
            Command::new("init")
                .about("start a new proxy server in the specified path")
                // .arg(arg!(-p --path "the rodeo.toml | rodeo.json path").action(ArgAction::SetTrue))
                .arg(arg!([file] "the config file type, default to toml")),
        )
        .get_matches();

    // the init command
    if let Some(matches) = matches.subcommand_matches("init") {
        if let Some(config_type) = matches.get_one::<String>("file") {
            parsers::create_config_file(String::from(config_type)).expect("TODO: panic message");
            std::process::exit(0)
        };
    }

    // let Some(config_path) = matches.get_one::<PathBuf>("config") else {
    //     Error::new("missing config file, please specify path to rodeo.toml or rodeo.json").missing_config();
    //     std::process::exit(0);
    //
    // };

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    // let subcommand = matches.subcommand_matches("init");
    // let config_path = matches.get_one::<PathBuf>("config");
    //
    // if config_path.is_none() {
    //     //     Error::new("missing config file, please specify path to rodeo.toml or rodeo.json").missing_config();
    //     std::process::exit(0);
    // }
}
