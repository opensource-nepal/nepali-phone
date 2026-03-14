//! # nepali-phone
//!
//! Parse and validate Nepali mobile and landline phone numbers.
//! ...

mod operator;
mod area;
mod phone;
mod parse;

pub use operator::Operator;
pub use area::{Area, AREAS, areas_by_code};
pub use phone::PhoneNumber;
pub use parse::{parse, is_valid, is_mobile_number, is_landline_number};