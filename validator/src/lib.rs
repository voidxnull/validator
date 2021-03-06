extern crate url;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate idna;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod types;
mod validation;
mod traits;

pub use validation::ip::{validate_ip, validate_ip_v4, validate_ip_v6};
pub use validation::email::{validate_email};
pub use validation::length::{validate_length};
pub use validation::range::{validate_range};
pub use validation::urls::{validate_url};
pub use validation::must_match::{validate_must_match};
pub use validation::contains::{validate_contains};
pub use validation::Validator;

pub use types::{ValidationErrors, ValidationError};
pub use traits::{Validate, HasLen, Contains};
