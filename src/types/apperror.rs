use surrealdb::Error as SurrealError;
//error
#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Surreal(SurrealError),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<SurrealError> for AppError {
    fn from(err: SurrealError) -> Self {
        AppError::Surreal(err)
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
