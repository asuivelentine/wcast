use std::result::Result as RResult;

use winfo::WeatherInfo;
use hyper;
use hyper::Client;
use hyper::header::Connection;

type Result<T> = RResult<T, WGError>;
//fn foo() -> Result<Weather>


#[derive(Debug)]
pub struct ApiKey {
    pub apiKey: String,
}

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

impl ApiKey {


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
