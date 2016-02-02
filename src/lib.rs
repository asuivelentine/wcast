//! small lib to access weather information from openWeatherMap (owm)
//! You have to create an owm account to get an API key.

#[deny(missing_docs,
       non_camle_case_types,
       non_snake_case,
       unused_import_braces,
       unsafe_code)]

extern crate xmltree;
extern crate hyper;

mod wgather;
mod winfo;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
