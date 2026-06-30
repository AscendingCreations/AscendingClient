use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TString {
    Str(&'static str),
    String(String),
}

impl TString {
    pub fn to_uppercase(&self) -> String {
        match self {
            TString::Str(s) => s.to_uppercase(),
            TString::String(s) => s.to_uppercase(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            TString::Str(s) => s.is_empty(),
            TString::String(s) => s.is_empty(),
        }
    }
}

impl fmt::Display for TString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TString::Str(s) => write!(f, "{}", s),
            TString::String(s) => write!(f, "{}", s),
        }
    }
}

impl From<&'static str> for TString {
    fn from(string: &'static str) -> Self {
        TString::Str(string)
    }
}

impl From<String> for TString {
    fn from(string: String) -> Self {
        TString::String(string)
    }
}

impl AsRef<str> for TString {
    fn as_ref(&self) -> &str {
        match self {
            TString::Str(s) => s,
            TString::String(s) => s,
        }
    }
}
