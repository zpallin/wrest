
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate env_logger;

#[macro_use]
extern crate error_chain;

#[cfg(test)]
extern crate mockito;

#[macro_use]
pub mod apipath;
pub mod rest;
pub mod request;

// easy prelude for use statements to do the basics
pub mod prelude {
    pub use rest::prelude::*;
    pub use rest::methods::*;
    pub use request::*;
    pub use request::Response as Response;
    pub use request::Error as Error;
    pub use apipath::*;
}
