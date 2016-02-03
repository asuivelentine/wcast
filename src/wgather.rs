use std::result::Result as RResult;

use winfo::WeatherInfo;
use hyper;
use hyper::Client;
use hyper::header::Connection;

type Result<T> = RResult<T, WGError>;
type Language = String;
type Location = String;
type Forecast = bool;


#[derive(Debug)]
pub enum ErrorKind {

}

#[derive(Debug)]
pub struct WGError {
    kind: ErrorKind,
    cause: Option<Box<::std::error::Error>>,
}

pub struct WeatherGather {
    apiKey: String,
}

impl WeatherGather {
    pub fn new(apikey: String) -> WeatherGather {
        WeatherGather {
            apiKey: apikey,
        }
    }

    pub fn get_weather(&self, li: LocationInformation) -> WeatherGetter {
        WeatherGetter {
            li: LocationInformation::from_coords(52.5243700, 13.4105300),
            lang: String::new(),
            loc: String::new(),
            forecast: false,
        }
    }
}


pub struct WeatherGetter {
    li: LocationInformation,
    lang: Language,
    loc: Location,
    forecast: Forecast,
}

impl WeatherGetter {
    pub fn with_location(mut self, loc: Location) -> WeatherGetter {
        self.loc = loc;
        self
    }

    pub fn with_language(mut self, lang: Language) -> WeatherGetter {
        self.lang = lang;
        self
    }

    pub fn with_forcase(mut self, forecast: Forecast) -> WeatherGetter {
        self.forecast = forecast;
        self
    }

    pub fn get(self) -> Result<WeatherInfo> {
        let mut uri = "http://api.openweathermap.org/data/2.5/weather?".to_string();
        let locatoin = match self.li {
            LocationInformation::Coord{ lat: lat, lng: lng } => {
                let mut tmp = "?lat=".to_string();
                tmp = tmp + &lat.to_string();
                tmp = tmp + "&lon=";
                tmp = tmp + &lng.to_string();
            }
            LocationInformation::City{ city: city, country: country } => {
                let mut tmp = "?q=".to_string();
                tmp = tmp + &city;
                tmp = tmp + ",";
                tmp = tmp + &country;
            }
            LocationInformation::Zip{ zip: zip, country: country } => {
                let mut tmp = "?q=".to_string();
                tmp = tmp + &zip;
                tmp = tmp + ",";
                tmp = tmp + &country;
            }
        };
        Ok(WeatherInfo::new())
    }

    pub fn get_rss_feed(url: &str) -> String{
        let client = Client::new();

        let mut res = client.get(url)
            .header(Connection::close())
            .send().unwrap();

        let mut body = String::new();
        //res.read_to_string(&mut body).unwrap();
        body
    }
}


#[derive(Debug)]
pub enum LocationInformation {
    Coord { lat: f64, lng: f64},
    City { city: String, country: String },
    Zip { zip: String, country: String },
}

impl LocationInformation {
    pub fn from_coords(lat: f64, lng: f64) -> LocationInformation {
       LocationInformation::Coord {
           lat: lat,
           lng: lng
       }
    }

    pub fn from_city(city: String, country: String) -> LocationInformation {
        LocationInformation::City {
            city: city,
            country: country,
        }
    }

    pub fn from_zip(zip: String, country: String) -> LocationInformation {
        LocationInformation::Zip {
            zip: zip,
            country: country,
        }
    }
}
