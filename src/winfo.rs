use std::convert::From;

use rustc_serialize::json::Json;

pub type Time = u64;

//temporary, until the real json data is parsed...
#[derive(Debug)]
pub struct WeatherInfo {
    dummy: isize,
}

#[derive(Debug)]
pub struct WeatherInfo2 {
    city: City,
    day: Vec<Day>
}
   
#[derive(Debug)]
pub struct City {
    name: String,
    country: String,
    lat: f64,
    lng: f64,
}

#[derive(Debug)]
pub struct Day {
    sunrise: Time,
    sunset: Time,
    weather: Vec<Weather>   
}

#[derive(Debug)]
pub struct Weather{
    description: String,
    temp: f64,
    humidity: u64,
    pressure: f64,
    sea_level: f64,
    grnd_level: f64,
    when: Time,
    wind: Wind
}

#[derive(Debug)]
pub struct Wind {
    speed: f64,
    degree: f64
}

impl From<String> for WeatherInfo {
    fn from(json: String) -> Self {
        let data = Json::from_str(&json).unwrap();
        WeatherInfo::get_city(data.clone());

        
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

    fn get_wind(data_parent: Json ) -> Option<Wind> {
       let value = data_parent.as_object()
           .and_then(|parent| parent.get("wind"))
           .and_then(|wind| wind.as_object());

       if value.is_none() {
            return None
       }
       let value = value.unwrap();

       let speed = value.get("speed")
           .and_then(|speed| speed.as_f64());

       let degree = value.get("deg")
           .and_then(|degree| degree.as_f64());

       let wind = Wind {
            speed: speed.unwrap(),
            degree: degree.unwrap(),
       };

        Some(wind)
    }

    fn get_country(data_root: Json) -> Option<String> {
        let root_obj = data_root.as_object().unwrap();

        let country = match root_obj.get("city") {
            Some(n) => {
                n.as_object()
                .and_then(|city| city.get("country"))
                .and_then(|country| country.as_string())
            }
            None => { 
                root_obj.get("sys")
                .and_then(|sys| sys.as_object())
                .and_then(|sys| sys.get("country"))
                .and_then(|c| c.as_string())
            }
        };

        match country {
            Some(n) => Some(n.to_string()),
            None => None,
        }
    }

    fn get_city(data_root: Json) -> Option<City> {
        let root_obj = data_root.as_object().unwrap();
        let name = root_obj.get("name").unwrap();

        let coord = root_obj.get("coord").unwrap().as_object().unwrap();
        let lat = coord.get("lat").unwrap();
        let lng = coord.get("lon").unwrap();

        let country: String = match root_obj.get("country") {
            Some(n) => n.to_string(),
            None => String::from(""),
        };

        let city = City {
            name: name.to_string(),
            lat: lat.as_f64().unwrap(),
            lng: lng.as_f64().unwrap(),
            country: country,
        };

        Some(city)
    }
}
