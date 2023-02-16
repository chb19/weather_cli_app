use std::collections::HashMap;

use clap::Parser;
use regex::Regex;
use serde::{Serialize, Deserialize};

use crate::{provider::Provider, weather::Weather};

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    #[command(subcommand)]
    Configure(Provider),
    Get(Weather),
}

impl Command {
    pub fn parse_command() -> clap::error::Result<Command> {
        let mut line = String::default();
        let stdin = std::io::stdin();
        let _ = stdin.read_line(&mut line)?;
        let splitted_line = line.split_whitespace();
        Command::try_parse_from(splitted_line)
    }

    pub fn process(self, api_keys: &HashMap<Provider, String>, re: &Regex, provider: &mut Option<Provider>) -> anyhow::Result<()> {
        match self {
            Command::Get(weather) => {
                let timestamp = weather.get_timestamp(&re);
                if let Some(provider) = provider {
                    let resp = provider.get_response(&api_keys, timestamp, weather.address.clone())?;
                    println!("{}", serde_json::to_string_pretty(&resp)?);
                }
                else {
                    eprintln!(r#"Please, configure provider with command 'weather configure {{provider}}!"#);
                }
                Ok(())
            }
            Command::Configure(_provider) => {
                *provider = Some(_provider);
                Ok(())
            }
        }        
    }
}