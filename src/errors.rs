#[derive(Debug)]
pub enum DataError {
    #[fail(display = "Error in getting result {}", _0)]
    ResultError(#[cause] diesel::result::Error),
    #[fail(display = "Error in getting result {}", _0)]
    Error(String),
}

impl DataError {
    pub fn message(self) -> String {
        format!("{:#?}", self)
    }
}

impl From<diesel::result::Error> for DataError {
    fn from(err: diesel::result::Error) -> Self {
        DataError::ResultError(err)
    }
}

impl From<String> for DataError {
    fn from(s: String) -> Self {
        DataError::Error(s)
    }
}
