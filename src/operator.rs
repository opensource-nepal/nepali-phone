use std::fmt;
/// Mobile network operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    NepalTelecom,
    Ncell,
    SmartCell,
    Utl,
    HelloMobile,
}


impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::NepalTelecom => "Nepal Telecom",
            Self::Ncell        => "Ncell",
            Self::SmartCell    => "Smart Cell",
            Self::Utl          => "UTL",
            Self::HelloMobile  => "Hello Mobile",
        })
    }
}
pub fn operator_for(number: &str) -> Option<Operator> {
    // Expects a 10-digit string with no country code.
    match &number[..3] {
        "984" | "985" | "986" | "974" | "975" | "976" => Some(Operator::NepalTelecom),
        "980" | "981" | "982"                  => Some(Operator::Ncell),
        "961" | "962" | "988"                  => Some(Operator::SmartCell),
        "972"                                   => Some(Operator::Utl),
        "963"                                   => Some(Operator::HelloMobile),
        _                                       => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_operator() {
        assert_eq!(Operator::NepalTelecom.to_string(), "Nepal Telecom");
        assert_eq!(Operator::Ncell.to_string(), "Ncell");
    }
}