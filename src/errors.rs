use thiserror::Error;
use diesel::result::Error as DieselError;
use lettre::transport::smtp::Error as LettreError;

#[derive(Debug, Error)]
pub enum ContactError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] DieselError),

    #[error("Email sending error: {0}")]
    EmailError(#[from] LettreError),
}