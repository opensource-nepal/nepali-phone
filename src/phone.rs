use std::fmt;
use std::collections::HashMap;
use crate::area::Area;
use crate::operator::Operator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhoneNumber {
    Mobile {
        number: String,
        operator: Operator,
    },
    Landline {
        number: String,
        area_code: String,
        areas: Vec<&'static Area>,
    },
}

impl PhoneNumber {
    /// Returns the phone number as a `HashMap<&str, String>`.
///
/// Mobile keys: `type`, `number`, `operator`.  
/// Landline keys: `type`, `number`, `area_code`, `areas` (comma-separated).
///
/// # Example
/// ```rust
/// use nepal_phone::parse;
///
/// let map = parse("9841234567").unwrap().to_map();
/// // {"type": "mobile", "number": "9841234567", "operator": "Nepal Telecom"}
/// ```
    pub fn to_map(&self) -> HashMap<&'static str, String> {
        let mut map = HashMap::new();
        match self {
            PhoneNumber::Mobile { number, operator } => {
                map.insert("type", "mobile".to_string());
                map.insert("number", number.clone());
                map.insert("operator", operator.to_string());
            }
            PhoneNumber::Landline { number, area_code, areas } => {
                map.insert("type", "landline".to_string());
                map.insert("number", number.clone());
                map.insert("area_code", area_code.clone());
                map.insert("areas", areas.iter().map(|a| a.name).collect::<Vec<_>>().join(", "));
            }
        }
        map
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhoneNumber::Mobile { number, operator } => {
                write!(f, "Mobile({number}, {operator})")
            }
            PhoneNumber::Landline { number, area_code, areas } => {
                let names: Vec<&str> = areas.iter().map(|a| a.name).collect();
                write!(f, "Landline({number}, code={area_code}, areas=[{}])", names.join(", "))
            }
        }
    }
}