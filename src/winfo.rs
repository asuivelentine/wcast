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

#[cfg(test)]
mod tests {
    use rustc_serialize::json::Json;
    use super::WeatherInfo;

    #[test]
    fn test_wind_forecast() {
        let json = "{\"main\":
            {\"temp\":273.4,\"temp_min\":272.173,\"temp_max\":273.4,\"pressure\":1009.3,\"sea_level\":
            1029.81,\"grnd_level\":1009.3,\"humidity\":91,\"temp_kf\":1.22},
            \"weather\":[{\"id\":803,\"main\":\"Clouds\",\"description\":\"broken clouds\",\"icon\":\"04d\"}],
            \"clouds\":{\"all\":80},
            \"wind\":{\"speed\":7.34,\"deg\":159.504},
            \"rain\":{},
            \"snow\":{},
            \"sys\":{\"pod\":\"d\"},
            \"dt_txt\":\"2016-02-11 12:00:00\"}";
        let json = Json::from_str(&json).unwrap();
        let wind = WeatherInfo::get_wind(json);
        assert!(wind.is_some());
        let wind = wind.unwrap();

        assert_eq!(7.34, wind.speed);
        assert_eq!(159.504, wind.degree);
    }

    #[test]
    fn test_wind() {
        let json = "{\"coord\":{\"lon\":-0.13,\"lat\":51.51},\"weather\":[{\"id\":802,\"main\":
            \"Clouds\",\"description\":\"scattered clouds\",\"icon\":\"03d\"}],\"base\":
            \"cmc stations\",\"main\":{\"temp\":273.706,\"pressure\":1007.64,\"humidity\":86,
            \"temp_min\":273.706,\"temp_max\":273.706,\"sea_level\":1017.9,\"grnd_level\":1007.64},
            \"wind\":{\"speed\":2.03,\"deg\":233.501},\"clouds\":{\"all\":32},\"dt\":1455182444,
            \"sys\":{\"message\":0.0059,\"country\":\"GB\",\"sunrise\":1455175339,\"sunset\":
                1455210476},\"id\":2643743,\"name\":\"London\",\"cod\":200}\n";
        let json = Json::from_str(&json).unwrap();
        let wind = WeatherInfo::get_wind(json);
        assert!(wind.is_some());
        let wind = wind.unwrap();

        assert_eq!(2.03, wind.speed);
        assert_eq!(233.501, wind.degree);

    }
}
