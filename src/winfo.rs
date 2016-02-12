use std::convert::From;

use rustc_serialize::json;
use rustc_serialize::json::{ Json, Object };

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

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Coord {
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

#[derive(Debug, RustcDecodable, RustcEncodable)]
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

    fn get_city(data_root: Json) -> Option<City> {
        let root_obj = data_root.as_object().unwrap();
        
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
    
    #[test]
    fn test_wind_forecast() {

        let json = 
        "{\"main\":{\"temp\":273.4,\"temp_min\":272.173,\"temp_max\":273.4,\"pressure\":1009.3,
        \"sea_level\":1029.81,\"grnd_level\":1009.3,\"humidity\":91,\"temp_kf\":1.22},
        \"weather\":[{\"id\":803,\"main\":\"Clouds\",\"description\":\"broken clouds\",
        \"icon\":\"04d\"}],\"clouds\":{\"all\":80},\"wind\":{\"speed\":7.34,\"deg\":159.504},
        \"rain\":{},\"snow\":{},\"sys\":{\"pod\":\"d\"},\"dt_txt\":\"2016-02-11 12:00:00\"}";

        let json = Json::from_str(&json).unwrap();
        let wind = WeatherInfo::get_wind(json);
        assert!(wind.is_some());
        let wind = wind.unwrap();

        assert_eq!(7.34, wind.speed);
        assert_eq!(159.504, wind.degree);
    }


    #[test]
    fn test_country() {
        let json = "{\"coord\":{\"lon\":-0.13,\"lat\":51.51},\"weather\":[{\"id\":802,\"main\":
            \"Clouds\",\"description\":\"scattered clouds\",\"icon\":\"03d\"}],\"base\":
            \"cmc stations\",\"main\":{\"temp\":273.706,\"pressure\":1007.64,\"humidity\":86,
            \"temp_min\":273.706,\"temp_max\":273.706,\"sea_level\":1017.9,\"grnd_level\":1007.64},
            \"wind\":{\"speed\":2.03,\"deg\":233.501},\"clouds\":{\"all\":32},\"dt\":1455182444,
            \"sys\":{\"message\":0.0059,\"country\":\"GB\",\"sunrise\":1455175339,\"sunset\":
                1455210476},\"id\":2643743,\"name\":\"London\",\"cod\":200}\n";
        
        let json = Json::from_str(&json).unwrap();
        let country= WeatherInfo::get_country(json);
        assert!(country.is_some());
        assert_eq!("GB", country.unwrap()) 

    }

    #[test]
    fn test_country_forecast() {
        let json = "{\"city\":{\"id\":524901,\"name\":\"Moscow\",\"coord\":{\"lon\":37.615555,
        \"lat\":55.75222},\"country\":\"RU\",\"population\":0,\"sys\":{\"population\":0}},
        \"cod\":\"200\",\"message\":0.008,\"cnt\":37,\"list\":[{\"dt\":1455278400,\"main\":{
        \"temp\":274.089,\"temp_min\":274.089,\"temp_max\":274.089,\"pressure\":1009.52,
        \"sea_level\":1030.13,\"grnd_level\":1009.52,\"humidity\":96,\"temp_kf\":0},
        \"weather\":[{\"id\":500,\"main\":\"Rain\",\"description\":\"light rain\",\"icon\":\"10n\"}],
        \"clouds\":{\"all\":92},\"wind\":{\"speed\":4.82,\"deg\":270.001},\"rain\":{\"3h\":0.17},
        \"snow\":{\"3h\":0.05},\"sys\":{\"pod\":\"n\"},\"dt_txt\":\"2016-02-16 21:00:00\"},
        {\"dt\":1455667200,\"main\":{\"temp\":273.583,\"temp_min\":273.583,\"temp_max\":273.583,
        \"pressure\":1010.05,\"sea_level\":1030.65,\"grnd_level\":1010.05,\"humidity\":97,
        \"temp_kf\":0},\"weather\":[{\"id\":600,\"main\":\"Snow\",\"description\":\"light snow\",
        \"icon\":\"13n\"}],\"clouds\":{\"all\":92},\"wind\":{\"speed\":5.01,\"deg\":273.501},
        \"rain\":{},\"snow\":{\"3h\":0.12},\"sys\":{\"pod\":\"n\"},\"dt_txt\":\"2016-02-17 00:00:00\"
        }]}";

        let json = Json::from_str(&json).unwrap();
        let country= WeatherInfo::get_country(json);

        assert!(country.is_some());
        assert_eq!("RU", country.unwrap()) 
    }

    #[test]
    fn test_city() {
         let json = "{\"coord\":{\"lon\":-0.13,\"lat\":51.51},\"weather\":[{\"id\":802,\"main\":
         \"Clouds\",\"description\":\"scattered clouds\",\"icon\":\"03d\"}],\"base\":
         \"cmc stations\",\"main\":{\"temp\":273.706,\"pressure\":1007.64,\"humidity\":86,
         \"temp_min\":273.706,\"temp_max\":273.706,\"sea_level\":1017.9,\"grnd_level\":1007.64},
         \"wind\":{\"speed\":2.03,\"deg\":233.501},\"clouds\":{\"all\":32},\"dt\":1455182444,
         \"sys\":{\"message\":0.0059,\"country\":\"GB\",\"sunrise\":1455175339,\"sunset\":
         1455210476},\"id\":2643743,\"name\":\"London\",\"cod\":200}\n";
        
        let json = Json::from_str(&json).unwrap();
        let city= WeatherInfo::get_city(json);

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
         let json = "{\"city\":{\"id\":524901,\"name\":\"Moscow\",\"coord\":{\"lon\":37.615555,
        \"lat\":55.75222},\"country\":\"RU\",\"population\":0,\"sys\":{\"population\":0}},
        \"cod\":\"200\",\"message\":0.008,\"cnt\":37,\"list\":[{\"dt\":1455278400,\"main\":{
        \"temp\":274.089,\"temp_min\":274.089,\"temp_max\":274.089,\"pressure\":1009.52,
        \"sea_level\":1030.13,\"grnd_level\":1009.52,\"humidity\":96,\"temp_kf\":0},
        \"weather\":[{\"id\":500,\"main\":\"Rain\",\"description\":\"light rain\",\"icon\":\"10n\"}],
        \"clouds\":{\"all\":92},\"wind\":{\"speed\":4.82,\"deg\":270.001},\"rain\":{\"3h\":0.17},
        \"snow\":{\"3h\":0.05},\"sys\":{\"pod\":\"n\"},\"dt_txt\":\"2016-02-16 21:00:00\"},
        {\"dt\":1455667200,\"main\":{\"temp\":273.583,\"temp_min\":273.583,\"temp_max\":273.583,
        \"pressure\":1010.05,\"sea_level\":1030.65,\"grnd_level\":1010.05,\"humidity\":97,
        \"temp_kf\":0},\"weather\":[{\"id\":600,\"main\":\"Snow\",\"description\":\"light snow\",
        \"icon\":\"13n\"}],\"clouds\":{\"all\":92},\"wind\":{\"speed\":5.01,\"deg\":273.501},
        \"rain\":{},\"snow\":{\"3h\":0.12},\"sys\":{\"pod\":\"n\"},\"dt_txt\":\"2016-02-17 00:00:00\"
        }]}";


        let json = Json::from_str(&json).unwrap();
        let city= WeatherInfo::get_city(json);

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
    fn test_coord() {
        let json = "{\"coord\":{\"lon\":-0.13,\"lat\":51.51},\"weather\":[{\"id\":802,\"main\":
         \"Clouds\",\"description\":\"scattered clouds\",\"icon\":\"03d\"}],\"base\":
         \"cmc stations\",\"main\":{\"temp\":273.706,\"pressure\":1007.64,\"humidity\":86,
         \"temp_min\":273.706,\"temp_max\":273.706,\"sea_level\":1017.9,\"grnd_level\":1007.64},
         \"wind\":{\"speed\":2.03,\"deg\":233.501},\"clouds\":{\"all\":32},\"dt\":1455182444,
         \"sys\":{\"message\":0.0059,\"country\":\"GB\",\"sunrise\":1455175339,\"sunset\":
         1455210476},\"id\":2643743,\"name\":\"London\",\"cod\":200}\n";

        let json = Json::from_str(&json).unwrap();
        let coord = WeatherInfo::get_coords(json);
        assert!(coord.is_some());
        /*
        let coord = coord.unwrap();
        assert_eq!(51.51, coord.lat);
        assert_eq!(-0.13, coord.lng);
        */
    }

    #[test]
    fn test_coord_forecast() {
        let json = "{\"city\":{\"id\":524901,\"name\":\"Moscow\",\"coord\":{\"lon\":37.615555,
        \"lat\":55.75222},\"country\":\"RU\",\"population\":0,\"sys\":{\"population\":0}},
        \"cod\":\"200\",\"message\":0.008,\"cnt\":37,\"list\":[{\"dt\":1455278400,\"main\":{
        \"temp\":274.089,\"temp_min\":274.089,\"temp_max\":274.089,\"pressure\":1009.52,
        \"sea_level\":1030.13,\"grnd_level\":1009.52,\"humidity\":96,\"temp_kf\":0},
        \"weather\":[{\"id\":500,\"main\":\"Rain\",\"description\":\"light rain\",\"icon\":\"10n\"}],
        \"clouds\":{\"all\":92},\"wind\":{\"speed\":4.82,\"deg\":270.001},\"rain\":{\"3h\":0.17},
        \"snow\":{\"3h\":0.05},\"sys\":{\"pod\":\"n\"},\"dt_txt\":\"2016-02-16 21:00:00\"},
        {\"dt\":1455667200,\"main\":{\"temp\":273.583,\"temp_min\":273.583,\"temp_max\":273.583,
        \"pressure\":1010.05,\"sea_level\":1030.65,\"grnd_level\":1010.05,\"humidity\":97,
        \"temp_kf\":0},\"weather\":[{\"id\":600,\"main\":\"Snow\",\"description\":\"light snow\",
        \"icon\":\"13n\"}],\"clouds\":{\"all\":92},\"wind\":{\"speed\":5.01,\"deg\":273.501},
        \"rain\":{},\"snow\":{\"3h\":0.12},\"sys\":{\"pod\":\"n\"},\"dt_txt\":\"2016-02-17 00:00:00\"
        }]}";

        let json = Json::from_str(&json).unwrap();
        let coord = WeatherInfo::get_coords(json);
        assert!(coord.is_some());
        /*
        let coord = coord.unwrap();
        assert_eq!(55.75222, coord.lat);
        assert_eq!(37.615555, coord.lng);
        */
    }
}
