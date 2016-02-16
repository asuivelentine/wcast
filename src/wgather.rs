use std::io::Read;
use std::result::Result as RResult;

use winfo::WeatherInfo;

use hyper::Client;
use hyper::header::Connection;

pub type Result<T> = RResult<T, WGError>;
pub type Language = String;
pub type Forecast = bool;


#[derive(Debug)]
pub enum ErrorKind {

}

#[derive(Debug)]
pub struct WGError {
    kind: ErrorKind,
    cause: Option<Box<::std::error::Error>>,
}

///The initial information that is needed for any request
#[derive(Debug)]
pub struct WeatherGather {
    api_key: String,
}

impl WeatherGather {
    ///Generate a new Object of WeatherGather.
    ///This is needed for every request you want to send
    pub fn new(apikey: String) -> WeatherGather {
        WeatherGather {
            api_key: apikey,
        }
    }

    ///Generates a new WeatherGetter Object to configure
    ///the details of your request.
    ///
    ///the Default configuration is:
    /// - coordinates: Berlin
    /// - language: en
    /// - forecast: false
    /// - Unit: Celsius
    ///
    /// You can override tne default behavior using the implementation
    /// of WeatherGetter
    pub fn get_weather(&self) -> WeatherGetter {
        WeatherGetter {
            li: LocationInformation::from_coords(52.5243700, 13.4105300),
            lang: String::from("en"),
            forecast: false,
            wu: WeatherUnit::Celsius,
        }
    }
}

///Configure the functionality your request 
#[derive(Debug)]
pub struct WeatherGetter {
    li: LocationInformation,
    lang: Language,
    forecast: Forecast,
    wu: WeatherUnit,
}

#[derive(Debug)]
pub enum WeatherUnit {
    Kelvin,
    Celsius,
    Fahrenheit
}

impl WeatherGetter {
    pub fn with_location(mut self, li: LocationInformation) -> WeatherGetter {
        self.li = li;
        self
    }

    pub fn with_language(mut self, lang: Language) -> WeatherGetter {
        self.lang = lang;
        self
    }

    pub fn with_forecast(mut self, forecast: Forecast) -> WeatherGetter {
        self.forecast = forecast;
        self
    }

    pub fn in_units(mut self, wu: WeatherUnit) -> WeatherGetter {               
        self.wu = wu;                                                           
        self                                                                    
    } 

    pub fn get(self, wg: WeatherGather) -> Result<WeatherInfo> {
        let mut uri = match self.forecast {
            true => "http://api.openweathermap.org/data/2.5/forecast?".to_string(),
            false => "http://api.openweathermap.org/data/2.5/weather?".to_string(),
        };

        let location = match self.li {
            LocationInformation::Coord{ lat, lng } => { 
                format!("lat={}&lon={}", lat.to_string(), lng.to_string())
            }
            LocationInformation::City{ city, country } => {
                format!("q={},{}", city, country)
            }
            LocationInformation::Zip{ zip, country  } => {
                format!("zip={},{}", zip, country)
            }
        };
        uri = format!("{}{}&lang={}", uri, location, self.lang );
        uri = format!("{}&appid={}", uri, wg.api_key);
        

        let json = WeatherGetter::fetch_weather_data(&uri);
        let weather = WeatherInfo::from(json);
        Ok(weather)
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

///You can request the specific weather for different places by coord, city, zip
///
///- Coords: specify latitude and longitude
///- City: specify the place by city- and countryname
///- Zip: by zip-code + countryname
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
