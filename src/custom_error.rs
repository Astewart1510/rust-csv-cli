#[derive(Debug)]
pub enum CSVError {
    MenuReset,
    InputError(String),
    ValidationError(String),
    Other(Box<dyn std::error::Error>),
    FileNotFound(String),
}

impl std::fmt::Display for CSVError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CSVError::MenuReset => write!(f, "Menu selected"),
            CSVError::InputError(msg) => write!(f, "Input error: {}", msg),
            CSVError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            CSVError::Other(e) => write!(f, "Error: {}", e),
            CSVError::FileNotFound(msg) => write!(f, "File not found, current directory : {}", msg),
        }
    }
}

impl std::error::Error for CSVError {}

impl From<csv::Error> for CSVError {
    fn from(error: csv::Error) -> Self {
        CSVError::Other(Box::new(error))
    }
}

impl From<std::io::Error> for CSVError {
    fn from(error: std::io::Error) -> Self {
        CSVError::Other(Box::new(error))
    }
}
