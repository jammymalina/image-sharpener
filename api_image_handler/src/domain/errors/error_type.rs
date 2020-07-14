pub enum ErrorType {
    MissingHeaderError,
    InvalidHeaderError,
    InvalidBodyError,
}

impl ErrorType {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::MissingHeaderError => "MissingHeaderError",
            Self::InvalidHeaderError => "InvalidHeaderError",
            Self::InvalidBodyError => "InvalidBodyError",
        }
    }
}
