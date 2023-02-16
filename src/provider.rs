use std::collections::HashMap;

use anyhow::Context;
use serde::{Serialize, Deserialize};
use strum::{EnumIter, IntoEnumIterator};


#[derive(Debug, Clone, Copy, clap::Subcommand, Serialize, Deserialize, strum_macros::Display, EnumIter, Hash, PartialEq, Eq)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Provider {
    OpenWeatherMap,
    WeatherApi,
}

impl Provider {
    pub fn default() -> anyhow::Result<HashMap<Provider, String>> {
        let mut api_keys : HashMap<Provider, String> = HashMap::new();
        for provider in Provider::iter() {
            let api_key = std::env::var(provider.to_string()).with_context(|| format!("Failed to get API_KEY for {}. Please, set this in .env file.", provider))?;
            api_keys.insert(provider, api_key);
        }
        Ok(api_keys)
    }
    pub fn get_response(&self, api_keys: &HashMap<Provider, String>, timestamp: Option<i64>, address: String) -> anyhow::Result<serde_json::Value> {
        match *self {
            Provider::OpenWeatherMap => {
                let api_key = api_keys.get(&self).unwrap_or_else(|| panic!("Failed to find provider {} in api_keys.", self));
                let uri = format!("https://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}", address, api_key);
                let resp = reqwest::blocking::get(uri)?.json::<serde_json::Value>()?;
                let (lat, lon) = (&resp[0]["lat"], &resp[0]["lon"]);
                let uri = if let Some(timestamp) = timestamp { 
                    format!("https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&dt={}&exclude=daily,minutely,hourly&appid={}", lat, lon, timestamp, api_key)
                }
                else {
                    format!("https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&exclude=daily,minutely,hourly&appid={}", lat, lon, api_key)                                    
                };
                let resp = reqwest::blocking::get(uri)?.json::<serde_json::Value>()?;
                Ok(resp)
            }
            Provider::WeatherApi => {
                let api_key = api_keys.get(&self).unwrap_or_else(|| panic!("Failed to find  provider {} in api_keys.", self));
                let uri = if let Some(timestamp) = timestamp { 
                    format!("http://api.weatherapi.com/v1/current.json?key={}&dt={}&q={}&aqi=no", api_key, timestamp, address)                            }
                else {
                    format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no", api_key, address)
                };
                let resp = reqwest::blocking::get(uri)?.json::<serde_json::Value>()?;
                Ok(resp)
            }
        }

    }
}

#[cfg(test)]
mod provider_tests {
    use strum::IntoEnumIterator;

    use crate::provider::Provider;
    #[test]
    fn provider_default() {
        for provider in Provider::iter() {
            std::env::set_var(provider.to_string(), provider.to_string());
        }

        let provider_default = Provider::default().unwrap();
        for provider in Provider::iter() {
            assert!(provider_default.contains_key(&provider));
        }
    }       
}