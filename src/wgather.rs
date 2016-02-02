use std::result::Result as RResult;

use winfo::WeatherInfo;
use hyper;
use hyper::Client;
use hyper::header::Connection;

type Result<T> = RResult<T, WGError>;
//fn foo() -> Result<Weather>

#[derive(Debug)]
pub enum LocationInformation {
    Coord { lat: String, lng: String },
    City { city: String, country: String },
    Zip { zip: String, country: String },
}

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

    pub fn get_weather(&self, li: LocationInformation) -> Result<WeatherInfo> {
        match li {
            LocationInformation::Coord{ lat: lat, lng: lng } => println!("hjer"),
            LocationInformation::City{ city: city, country: country } => println!("hjer"),
            LocationInformation::Zip{ zip: zip, country: country } => println!("hjer"),
        };
        Ok(WeatherInfo::new())
    }

}

impl LocationInformation {
    pub fn from_coords(lat: String, lng: String) -> LocationInformation {
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
