use std::convert::From;

use rustc_serialize;
use rustc_serialize::json::Json;

pub type Time = u64;

#[derive(Debug)]
pub struct WeatherInfo {
    dummy: isize,
}

pub struct WeatherInfo2 {
    city: City,
    day: Vec<Day>
}
   
pub struct City {
    name: String,
    country: String,
    lat: f64,
    lng: f64,
    sea_level: f64,
    grnd_level: f64
}

pub struct Day {
    sunrise: u64,
    sunset: u64,
    tmp_max: f64,
    tmp_min: f64,
    weather: Vec<Weather>   
}

pub struct Weather{
    description: String,
    temp: f64,
    humidity: u64,
    pressure: f64,
    wind: Wind,
    when: Time
}

pub struct Wind {
    speed: f64,
    degree: f64
}

impl From<String> for WeatherInfo {
    fn from(json: String) -> Self {
        WeatherInfo {
            dummy: 5,
        }
    }
}

impl WeatherInfo {
    fn new() -> WeatherInfo {
        WeatherInfo {
            dummy: 5,
        }
    }

    pub fn print(json: &str) {
        let data = Json::from_str(json);
        println!("{:?}", data);
    }
}
