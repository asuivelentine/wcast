use std::convert::From;

use rustc_serialize;
use rustc_serialize::json;

pub struct WeatherInfo {
    pub dummy: isize,
}

impl From<String> for WeatherInfo {
    fn from(json: String) -> Self {
        WeatherInfo {
            dummy: 5,
        }
    }
}

impl WeatherInfo {

}
