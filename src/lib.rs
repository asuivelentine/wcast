//! small lib to access weather information from openWeatherMap (owm)
//! You have to create an owm account to get an API key.

#[deny(missing_docs,
       non_camel_case_types,
       non_snake_case,
       unused_import_braces,
       unsafe_code)]

extern crate rustc_serialize;
extern crate hyper;

mod winfo;

///This module is the interface of the library
///You can use this module the configure the request to openWeatherMap
pub mod wgather;

pub use winfo::WeatherInfo;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
