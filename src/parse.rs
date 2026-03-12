
use once_cell::sync::Lazy;
use regex::Regex;

use crate::phone::PhoneNumber;
use crate::operator:: operator_for;
use crate::area::areas_by_code;

///  Compiled regexes (initialised once, shared across threads)

static MOBILE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?:\+977|977)?-?(?:98|97|96)\d{8}$").unwrap()
});

static LANDLINE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?:\+977|977)?-?0?(?:[01][1-9]|2[13-9]|[3-9]\d)\d{6,7}$").unwrap()
});

/// Parses a Nepali phone number and returns a [`PhoneNumber`] on success,
/// or `None` if the number is invalid or unrecognised.
///
/// # Examples
/// ```rust
/// use nepal_phone::{parse, PhoneNumber, Operator};
/// 
/// let p = parse("9841234567").unwrap();
/// assert!(matches!(p, PhoneNumber::Mobile { operator: Operator::NepalTelecom, .. }));
///
/// assert!(parse("abc").is_none());
/// ```
pub fn parse(number: &str) -> Option<PhoneNumber> {
    if number.is_empty() {
        return None;
    }
    let normalised  = number.replace('-', "");
    if is_mobile_number(&normalised) {
        parse_mobile(&normalised)
    } else if is_landline_number(&normalised) {
        parse_landline(&normalised)
    } else {
        None
    }
}


// Public API

/// Returns `true` if `number` looks like a valid Nepali mobile number.
pub fn is_mobile_number(number: &str) -> bool {
    MOBILE_RE.is_match(number)
}

/// Returns `true` if `number` looks like a valid Nepali landline number.
pub fn is_landline_number(number: &str) -> bool {
    LANDLINE_RE.is_match(number)
}

/// Returns `true` if `number` is a valid Nepali phone number of any kind.
pub fn is_valid(number: &str) -> bool {
    is_mobile_number(number) || is_landline_number(number)
}

/// Strip the `+977` / `977` country-code prefix, if present.
fn strip_country_code(s: &str) -> &str {
    s.strip_prefix("+977")
        .or_else(|| s.strip_prefix("977"))
        .unwrap_or(s)
}



fn parse_mobile(raw: &str) -> Option<PhoneNumber> {
    let number = strip_country_code(raw).to_owned();
    let operator = operator_for(&number)?;
    Some(PhoneNumber::Mobile { number, operator })
}

/// Extracts the STD area code from a normalised landline string that starts
/// with `0` and has no country code.
///
/// The Kathmandu valley (Kathmandu, Lalitpur, Bhaktapur) all share the short
/// code `"01"`.  Codes `"010"`, `"011"`, and `"019"` are unrelated and must
/// not be collapsed.
fn area_code_of(number: &str) -> &str {
    let three = &number[..3];
    if number.starts_with("01") && !matches!(three, "010" | "011" | "019") {
        "01"
    } else {
        three
    }
}

fn parse_landline(raw: &str) -> Option<PhoneNumber> {
    let stripped = strip_country_code(raw);
    // Ensure the number begins with a leading zero.
    let with_zero: String = if stripped.starts_with('0') {
        stripped.to_owned()
    } else {
        format!("0{stripped}")
    };

    let area_code = area_code_of(&with_zero).to_owned();
    let areas = areas_by_code(&area_code);
    if areas.is_empty() {
        return None;
    }

    Some(PhoneNumber::Landline { number: with_zero, area_code, areas })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn landline_kathmandu_valley() {
        let p = parse("014567890").unwrap();
        match p {
            PhoneNumber::Landline { area_code, ref areas, .. } => {
                assert_eq!(area_code, "01");
                let names: Vec<&str> = areas.iter().map(|a| a.name).collect();
                assert!(names.contains(&"Kathmandu"));
                assert!(names.contains(&"Lalitpur"));
                assert!(names.contains(&"Bhaktapur"));
            }
            _ => panic!("expected Landline"),
        }
    }

    #[test]
    fn landline_pokhara() {
        let p = parse("0612345678").unwrap();
        match p {
            PhoneNumber::Landline { area_code, ref areas, .. } => {
                assert_eq!(area_code, "061");
                assert!(areas.iter().any(|a| a.name == "Kaski"));
            }
            _ => panic!("expected Landline"),
        }
    }

    #[test]
    fn landline_010_not_collapsed_to_01() {

        let p = parse("010520133").unwrap();
        match p {
            PhoneNumber::Landline { area_code, .. } => assert_eq!(area_code, "010"),
            _ => panic!("expected Landline"),
        }
    }

    

    #[test]
    fn invalid_short_number() {
        assert!(parse("12345").is_none());
    }

    #[test]
    fn invalid_empty_string() {
        assert!(parse("").is_none());
    }

    #[test]
    fn invalid_all_zeros() {
        assert!(!is_valid("00000000000"));
    }


    #[test]
    fn mobile_with_dashes() {
        assert!(is_mobile_number("+977-9842536789"));
    }

    #[test]
    fn mobile_977_prefix() {
        assert!(is_mobile_number("9779841234567"));
    }
}