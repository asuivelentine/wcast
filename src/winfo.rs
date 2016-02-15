use std::convert::From;

use std::collections::BTreeMap;

use rustc_serialize::json::{ Json };

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
    coord: Coord,
}

#[derive(Debug)]
pub struct Coord {
    lat: f64,
    lng: f64,
}

#[derive(Debug)]
pub struct Day {
    sunrise: Option<Time>,
    sunset: Option<Time>,
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

    fn get_wind(data_parent: &BTreeMap<String, Json> ) -> Option<Wind> {
       let value = data_parent.get("wind")
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

        match root_obj.get("city") {
            Some(n) => {
                n.as_object()
                .and_then(|city| city.get("country"))
                .and_then(|country| country.as_string().map(|s| s.to_string()))
            }
            None => { 
                root_obj.get("sys")
                .and_then(|sys| sys.as_object())
                .and_then(|sys| sys.get("country"))
                .and_then(|c| c.as_string().map(|i| i.to_string()))
            }
        }
    }

    fn get_coords(data_root: Json) -> Option<Coord> {
        let root_obj = data_root.as_object().unwrap();

        let coord = match root_obj.get("city") {
            Some(n) => {
                n.as_object()
                    .and_then(|city| city.get("coord"))
                    .and_then(|coords| coords.as_object())
            }
            None => { 
                root_obj.get("coord")
                    .and_then(|coords| coords.as_object())
            }
        };
        
        if coord.is_none() {
            return None
        }
        let coord = coord.unwrap();
        let lat = coord.get("lat")
            .and_then(|lat| lat.as_f64());
        let lng = coord.get("lon")
            .and_then(|lat| lat.as_f64());

        Some(Coord {
            lat: lat.unwrap(),
            lng: lng.unwrap()
        })
    }

    fn get_name(data_root: Json) -> Option<String> {
        let root_obj = data_root.as_object().unwrap();

        match root_obj.get("city") {
            Some(n) => {
                n.as_object()
                .and_then(|city| city.get("name"))
                .and_then(|name| name.as_string().map(|s| s.to_string()))
            }
            None => { 
                root_obj.get("name")
                .and_then(|name| name.as_string().map(|i| i.to_string()))
            }
        }
    }

    fn get_Day(data_root: Json) -> Option<Day> {
        let root_obj = data_root.as_object().unwrap();
        let mut sunset = None;
        let mut sunrise = None;
        
        let weather = match root_obj.get("list") {
            Some(n) => {
               let list = n.as_array().unwrap().into_iter();
               let mut w = Vec::new();
               for x in list.map(|item| item.as_object()) {
                   if let Some(main) = x {
                       if let Some(x)  = WeatherInfo::get_weather(main){
                            w.push(x);
                       }
                   }

               }
               w
            }
            None => {
                let mut weather = Vec::new();
                if let Some(w) = WeatherInfo::get_weather(root_obj){
                    weather.push(w);
                }

                let sys = root_obj.get("sys")
                    .and_then(|sys| sys.as_object()).unwrap();
                sunset = sys.get("sunset")
                    .and_then(|sunset| sunset.as_u64());
                sunrise = sys.get("sunrise")
                    .and_then(|sunrise| sunrise.as_u64());

                weather
            }
        };

        let day = Day {
            sunset: sunset,
            sunrise: sunrise,
            weather: weather,
        };

        Some(day)
    }

    fn get_weather(data_root: &BTreeMap<String, Json>) -> Option<Weather> {
        let time = data_root.get("dt").and_then(|time| time.as_u64()).unwrap();
        let weather = data_root.get("main").and_then(|main| main.as_object()).unwrap();

        let description = data_root.get("weather")
            .and_then(|weather| weather.as_array()).unwrap();
        let description = description.into_iter().next()
            .and_then(|next| next.as_object())
            .and_then(|weather| weather.get("description"))
            .and_then(|description| description.as_string())
            .map(|des| des.to_string()).unwrap();

        println!("{:?}", description);

        let temp = weather.get("temp")
            .and_then(|temp| temp.as_f64())
            .unwrap();
        let pressure = weather.get("pressure")
            .and_then(|pressure| pressure.as_f64())
            .unwrap();
        let humidity = weather.get("humidity")
            .and_then(|humidity| humidity.as_u64())
            .unwrap();
        let sea_level = weather.get("sea_level")
            .and_then(|sea_level| sea_level.as_f64())
            .unwrap();
        let grnd_level = weather.get("grnd_level")
            .and_then(|grnd_level| grnd_level.as_f64())
            .unwrap();
        if let Some(wind) = WeatherInfo::get_wind(data_root) {
            let weather = Weather {
                description: description,
                temp: temp,
                humidity: humidity,
                pressure: pressure,
                sea_level: sea_level,
                grnd_level: grnd_level,
                when: time,
                wind: wind,
            };
            Some(weather)
        } else {
            None
        }
    }

    fn get_city(data_root: Json) -> Option<City> {
        let name = WeatherInfo::get_name(data_root.clone());
        let country = WeatherInfo::get_country(data_root.clone());
        let coord = WeatherInfo::get_coords(data_root.clone());

        let city = City {
            name: name.unwrap(),
            country: country.unwrap(),
            coord: coord.unwrap(),
        };

        Some(city)
    }
}

#[cfg(test)]
mod tests {
    use rustc_serialize::json::Json;
    use super::WeatherInfo;

    #[test]
    fn test_wind_current() {
        let wind = WeatherInfo::get_wind(get_json(false).as_object().unwrap());
        assert!(wind.is_some());
        let wind = wind.unwrap();

        assert_eq!(9.59, wind.speed);
        assert_eq!(206.501, wind.degree);

    }
    
    #[test]
    fn test_wind_forecast() {

        let json = get_json(true);
        let json = json.as_object()
            .and_then(|root| root.get("list"))
            .and_then(|list| list.as_array()).unwrap().into_iter().next()
            .and_then(|json| json.as_object());

        assert!(json.is_some());
        
        let wind = WeatherInfo::get_wind(json.unwrap());
        assert!(wind.is_some());
    }


    #[test]
    fn test_country_current() {
        let country= WeatherInfo::get_country(get_json(false));
        assert!(country.is_some());
        assert_eq!("JP", country.unwrap()) 
    }

    #[test]
    fn test_country_forecast() {
        let country= WeatherInfo::get_country(get_json(true));

        assert!(country.is_some());
        assert_eq!("JP", country.unwrap()) 
    }

    #[test]
    fn test_city_current() {
        let city= WeatherInfo::get_city(get_json(false));

        assert!(city.is_some());
        /*
        let city = city.unwrap();
        assert_eq!("London", city.name);
        assert_eq!("GB", city.country);
        assert_eq!(51.51, city.coord.lat);
        assert_eq!(-0.13, city.coord.lng);
        */
    }
    
    #[test]
    fn test_city_forecast() {
        let city= WeatherInfo::get_city(get_json(true));

        assert!(city.is_some());
        /*
        let city = city.unwrap();
        assert_eq!("Moscow", city.name);
        assert_eq!("RU", city.country);
        assert_eq!(55.75222, city.coord.lat);
        assert_eq!(37.615555, city.coord.lng);
        */
    }


    #[test]
    fn test_coord_current() {
        let coord = WeatherInfo::get_coords(get_json(false));
        assert!(coord.is_some());
        /*
        let coord = coord.unwrap();
        assert_eq!(51.51, coord.lat);
        assert_eq!(-0.13, coord.lng);
        */
    }

    #[test]
    fn test_coord_forecast() {
        let coord = WeatherInfo::get_coords(get_json(true));
        assert!(coord.is_some());
        /*
        let coord = coord.unwrap();
        assert_eq!(55.75222, coord.lat);
        assert_eq!(37.615555, coord.lng);
        */
    }

    #[test]
    fn test_name_current() {
        let name = WeatherInfo::get_coords(get_json(false));
        assert!(name.is_some());
        /*
        assert_eq!("London", name.unwrap());
        */
    }

    #[test]
    fn test_name_forecast() {
        let name = WeatherInfo::get_coords(get_json(true));
        assert!(name.is_some());
        /*
        assert_eq!("Moscow", name.unwrap());
        */
    }

    #[test]
    fn test_weather_current() {
        let weather = WeatherInfo::get_weather(get_json(false).as_object().unwrap());
        assert!(weather.is_some());
    }

    #[test]
    fn test_weather_forecast() {
        let weather = get_json(true);
        let mut count = 0;
        let weather = weather.as_object()
            .and_then(|obj| obj.get("list"))
            .and_then(|list| list.as_array()).unwrap().into_iter();

        for x in weather {
            let item = WeatherInfo::get_weather(x.as_object().unwrap());
            assert!(item.is_some());
            count += 1;
        }
        assert_eq!(5, count);
    }

    #[test]
    fn test_day_current() {
        let json = get_json(false);
        let day = WeatherInfo::get_Day(json);
        assert!(day.is_some());
        let day = day.unwrap();
        assert!(day.sunset.is_some());
        assert!(day.sunrise.is_some());
        assert_eq!(1455312750, day.sunrise.unwrap());
        assert_eq!(1455351896, day.sunset.unwrap());
    }

    #[test]
    fn test_day_forecast() {
        let json = get_json(true);
        let day = WeatherInfo::get_Day(json);
        assert!(day.is_some());
        let day = day.unwrap();
        assert_eq!(5, day.weather.len());
        assert!(day.sunset.is_none());
        assert!(day.sunrise.is_none());
    }

    fn get_json(forecast: bool) -> Json {
        let json = match forecast {
            true => {
                "{\"city\":{\"id\":1851632,\"name\":\"Shuzenji\",\"coord\":{\"lon\":138.933334,
                \"lat\":34.966671},\"country\":\"JP\",\"population\":0,\"sys\":{\"population\":0}},
                \"cod\":\"200\",\"message\":0.0056,\"cnt\":40,\"list\":[{\"dt\":1455408000,\"main\":{
                \"temp\":285.62,\"temp_min\":284.269,\"temp_max\":285.62,\"pressure\":920.42,
                \"sea_level\":1014.58,\"grnd_level\":920.42,\"humidity\":98,\"temp_kf\":1.35},
                \"weather\":[{\"id\":501,\"main\":\"Rain\",\"description\":\"moderate rain\",
                \"icon\":\"10d\"}],\"clouds\":{\"all\":44},\"wind\":{\"speed\":0.83,\"deg\":200.5},
                \"rain\":{\"3h\":8.44},\"sys\":{\"pod\":\"d\"},\"dt_txt\":\"2016-02-14 00:00:00\"},
                {\"dt\":1455418800,\"main\":{\"temp\":289.44,\"temp_min\":288.168,\"temp_max\":289.44,
                \"pressure\":918.51,\"sea_level\":1012.27,\"grnd_level\":918.51,\"humidity\":81,
                \"temp_kf\":1.27},\"weather\":[{\"id\":500,\"main\":\"Rain\",\"description\":\"light rain\",
                \"icon\":\"10d\"}],\"clouds\":{\"all\":0},\"wind\":{\"speed\":1.89,\"deg\":242.002},
                \"rain\":{\"3h\":0.175},\"sys\":{\"pod\":\"d\"},\"dt_txt\":\"2016-02-14 03:00:00\"},
                {\"dt\":1455429600,\"main\":{\"temp\":288.45,\"temp_min\":287.246,\"temp_max\":288.45,
                \"pressure\":917.46,\"sea_level\":1011.19,\"grnd_level\":917.46,\"humidity\":68,
                \"temp_kf\":1.2},\"weather\":[{\"id\":800,\"main\":\"Clear\",\"description\":\"sky is clear\",
                \"icon\":\"01d\"}],\"clouds\":{\"all\":0},\"wind\":{\"speed\":2.5,\"deg\":249.501},\"rain\":{},
                \"sys\":{\"pod\":\"d\"},\"dt_txt\":\"2016-02-14 06:00:00\"},{\"dt\":1455440400,
                \"main\":{\"temp\":267.159,\"temp_min\":267.159,\"temp_max\":267.159,\"pressure\":940.32,
                \"sea_level\":1040.37,\"grnd_level\":940.32,\"humidity\":77,\"temp_kf\":0},
                \"weather\":[{\"id\":803,\"main\":\"Clouds\",\"description\":\"broken clouds\",
                \"icon\":\"04n\"}],\"clouds\":{\"all\":64},\"wind\":{\"speed\":1.06,\"deg\":229.5},\"rain\":{},
                \"snow\":{},\"sys\":{\"pod\":\"n\"},\"dt_txt\":\"2016-02-18 18:00:00\"},{\"dt\":1455829200,
                \"main\":{\"temp\":269.084,\"temp_min\":269.084,\"temp_max\":269.084,\"pressure\":939.91,
                \"sea_level\":1039.86,\"grnd_level\":939.91,\"humidity\":77,\"temp_kf\":0},\"weather\":[{\"id\":500,
                \"main\":\"Rain\",\"description\":\"light rain\",\"icon\":\"10n\"}],\"clouds\":{\"all\":0},
                \"wind\":{\"speed\":1.31,\"deg\":232.5},\"rain\":{\"3h\":0.0025},\"snow\":{},
                \"sys\":{\"pod\":\"n\"},\"dt_txt\":\"2016-02-18 21:00:00\"}]}".to_string()
            }
            false => {
                "{\"coord\":{\"lon\":138.93,\"lat\":34.97},\"weather\":[{\"id\":502,
                \"main\":\"Rain\",\"description\":\"heavy intensity rain\",\"icon\":\"10n\"}],
                \"base\":\"cmc stations\",\"main\":{\"temp\":288.555,\"pressure\":1009.58,
                \"humidity\":95,\"temp_min\":288.555,\"temp_max\":288.555,\"sea_level\":1018.89,
                \"grnd_level\":1009.58},\"wind\":{\"speed\":9.59,\"deg\":206.501},\"rain\":{\"3h\":12.41},
                \"clouds\":{\"all\":92},\"dt\":1455396748,\"sys\":{\"message\":0.0097,\"country\":\"JP\",
                \"sunrise\":1455312750,\"sunset\":1455351896},\"id\":1851632,\"name\":\"Shuzenji\",
                \"cod\":200}".to_string()
            }
        };
        Json::from_str(&json).unwrap()
    }
}
