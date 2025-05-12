use crate::model::UserProfile;
use crate::error::ProfileKitError;

/// Abstraction for user profile data access and persistence.
///
/// This trait defines the expected interface for any storage backend
/// (e.g., database, API, in-memory) that handles user profile data.
///
/// Implementors are expected to handle serialization, validation,
/// and error propagation in a backend-specific manner.
pub trait UserProfileRepository {
    /// Retrieves a user profile by its unique ID.
    ///
    /// Returns `Ok(Some(UserProfile))` if found,
    /// `Ok(None)` if not found,
    /// or `Err(ProfileKitError)` if an error occurred during the operation.
    fn get_by_id(&self, id: String) -> Result<Option<UserProfile>, ProfileKitError>;

    /// Creates a new user profile in the storage backend.
    ///
    /// Returns an error if the profile already exists, or if storage fails.
    fn create(&self, profile: UserProfile) -> Result<(), ProfileKitError>;

    /// Updates an existing user profile.
    ///
    /// Returns an error if the profile does not exist, or if storage fails.
    fn update(&self, profile: UserProfile) -> Result<(), ProfileKitError>;

    /// Deletes a user profile by its ID.
    ///
    /// Returns an error if deletion fails.
    fn delete(&self, id: String) -> Result<(), ProfileKitError>;
}
