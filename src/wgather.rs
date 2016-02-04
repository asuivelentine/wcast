use std::io::Read;
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

#[derive(Debug)]
pub struct WeatherGather {
    api_key: String,
}

impl WeatherGather {
    pub fn new(apikey: String) -> WeatherGather {
        WeatherGather {
            api_key: apikey,
        }
    }

    pub fn get_weather(&self, li: LocationInformation) -> WeatherGetter {
        WeatherGetter {
            li: li,
            lang: String::new(),
            loc: String::new(),
            forecast: false,
        }
    }
}


#[derive(Debug)]
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
        let mut uri = match self.forecast {
            true => "http://api.openweathermap.org/data/2.5/forecast?".to_string(),
            false => "http://api.openweathermap.org/data/2.5/weather?".to_string(),
        };

        let location = match self.li {
            LocationInformation::Coord{ lat, lng } => { 
                format!("?lat={}&lon={}", lat.to_string(), lng.to_string())
            }
            LocationInformation::City{ city, country } => {
                format!("?q={},{}", city, country)
            }
            LocationInformation::Zip{ zip, country  } => {
                format!("?zip={},{}", zip, country)
            }
        };
        //uri = uri + &location;
        uri = uri + "&mode=xml";
        

        let xml = WeatherGetter::fetch_weather_data(&uri);
        Ok(WeatherInfo::new())
    }

    fn fetch_weather_data(url: &str) -> String{
        let client = Client::new();

        let mut res = client.get(url)
            .header(Connection::close())
            .send().unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
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
