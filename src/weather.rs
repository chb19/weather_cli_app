use regex::Regex;
use serde::{Serialize, Deserialize};

#[derive(clap::Args, Debug, Clone, Serialize, Deserialize)]
pub struct Weather {    
    pub address: String, 
    pub date: Option<String>,
}

impl Weather {
    pub fn get_timestamp(&self, re: &Regex) -> Option<i64> {
        if let Some(date) = self.date.clone() {
            let parsed_date = chrono::NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%S");
            if re.is_match(&date) && parsed_date.is_ok() {
                Some(parsed_date.unwrap().timestamp())
            }
            else {
                eprintln!("Please, enter date in the format yyyy-mm-ddThh:mm:ss");
                None
            }
        }
        else {
            None
        }         
    }
}

#[cfg(test)]
mod weather_tests {
    use regex::Regex;

    use super::Weather;

    fn get_regex() -> Regex {
        Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$").unwrap()
    }

    #[test]
    fn regex_match() {
        let re = get_regex();
        let date = "2023-02-15T15:08:20";
        assert!(re.is_match(date));
    }       
    #[test]
    fn regex_not_match() {
        let re = get_regex();
        let date = "2023-02-15TT15:01:20";
        assert!(!re.is_match(date));
    }       
    #[test]
    fn timestamp() {
        let re = get_regex();
        let date = "2023-02-15T15:08:20";
        let weather = Weather {address : String::default(), date : Some(String::from(date))};
        assert_eq!(weather.get_timestamp(&re).unwrap(), 1676473700);
    }       
    #[test]
    #[should_panic]
    fn parse_wrong_date() {
        let re = get_regex();
        let date = "2023-02-15T15:78:20";
        let weather = Weather {address : String::default(), date : Some(String::from(date))};
        weather.get_timestamp(&re).unwrap();
    }       

}