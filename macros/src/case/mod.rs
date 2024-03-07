mod cvt;

use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;
use cvt::snakecase;

pub(crate) enum Case {
    Lowercase,
    Uppercase,
    Snakecase,
}

impl Case {
    pub fn parse_str(&self, s: &str) -> String {
        match self {
            Self::Lowercase => s.to_lowercase(),
            Self::Snakecase => snakecase(s),
            Self::Uppercase => s.to_uppercase(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ParseCaseError {
    UnknownCase,
}

impl Error for ParseCaseError {}

impl Display for ParseCaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnknownCase => write!(f, r#"Parse case error: unknown "case" value"#),
        }
    }
}

impl FromStr for Case {
    type Err = ParseCaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lowercase" => Ok(Self::Lowercase),
            "uppercase" => Ok(Self::Uppercase),
            "snakecase" => Ok(Self::Snakecase),
            _ => Err(ParseCaseError::UnknownCase)
        }
    }
}

