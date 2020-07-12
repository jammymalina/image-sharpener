pub enum ErrorType {
    MissingHeaderError,
    InvaidHeaderError,
}

impl ErrorType {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::MissingHeaderError => "MissingHeaderError",
            Self::InvaidHeaderError => "InvalidHeaderError",
        }
    }
}
