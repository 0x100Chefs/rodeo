use std::{ffi::OsStr, path::Path};

use anyhow::Result;
use clap::{arg, command, Command};
mod errors;
mod parsers;
mod rodeo;

use errors::Error as RodeoError;
use rodeo::Rodeo;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let matches = command!()
        .subcommand(
            Command::new("init")
                .about("start a new proxy server in the specified path")
                .arg(arg!([file] "the config file type, default to toml")),
        )
        .subcommand(
            Command::new("run")
                .about("Run the proxy server")
                .arg(arg!([port] "The port on which the proxy should run, default to 10465"))
                .arg(arg!(-c --config <CONFIG> "Optionally sets a config file to use")),
        )
        .get_matches();

    // the init command
    if let Some(matches) = matches.subcommand_matches("init") {
        if let Some(config_type) = matches.get_one::<String>("file") {
            parsers::create_config_file(String::from(config_type))
                .expect(&RodeoError::new("message"));
            std::process::exit(0)
        };
    }

    // thr run command
    if let Some(matches) = matches.subcommand_matches("run") {
        let port = matches
            .get_one::<String>("port")
            .unwrap_or(&String::from("10465"))
            .parse::<u16>()
            .ok()
            .unwrap();

        // read the config path, or create one if it does not exist
        let config_path = matches
            .get_one::<String>("config")
            .expect(&RodeoError::new("invalid config path"));

        //detect the config type, and create a config file if it does not exist
        let Some(config_type) = Path::new(config_path).extension().and_then(OsStr::to_str) else {
            todo!();
        };

        if !std::path::Path::new(&config_path).exists() {
            parsers::create_config_file(String::from(config_type))
                .expect(&RodeoError::new("error creating file"));
        }

        Rodeo::new(config_path.into()).run(port).await?;
    }

    Ok(())
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
