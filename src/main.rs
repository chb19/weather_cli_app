use regex::Regex;

use clap::{ error::ErrorKind};
use dotenvy::dotenv;
use weather_cli_app::{provider::Provider, command::Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$").unwrap();

    let api_keys = Provider::default()?;
    let mut provider: Option<Provider> = None;
    loop {
        let command = Command::parse_command(); 

        match command {
            Ok(command) => command.process(&api_keys, &re, &mut provider)?,
            Err(err) => {
                match err.kind() {
                    ErrorKind::DisplayHelp => {
                        eprintln!("{}", err)                        
                    }
                    _ => {
                        eprintln!("Error: {}. Please, see help and try again!", err)
                    }
                }
            }
        }   
    }
}