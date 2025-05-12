use thiserror::Error;

/// Errors that may occur during operations related to a user profile.
#[derive(Debug, Error)]
pub enum ProfileKitError {
    /// A general database error, containing an underlying message from the database layer.
    #[error("Database error: {0}")]
    Database(String),

    /// Returned when a requested user profile could not be found.
    #[error("User profile not found")]
    NotFound,

    /// Returned when the input provided to a function is considered invalid.
    /// Includes a message describing the reason.
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Returned when user preferences are required but missing in the profile.
    #[error("User preferences are missing")]
    MissingPreferences,

    /// Returned when attempting to create a user profile that already exists.
    #[error("User profile already exists")]
    AlreadyExists,

    /// A generic storage-related error, usually from file systems, in-memory stores, or cloud buckets.
    #[error("Storage error occurred")]
    StorageError,
}
