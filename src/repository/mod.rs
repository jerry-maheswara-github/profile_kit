/// Domain-level abstractions for user profile operations.
///
/// This module contains traits that define contracts
/// for interacting with user profile storage backends.
///
/// Implementors of this trait may provide in-memory, database,
/// remote API (e.g., gRPC), or mock implementations.
pub mod traits;
pub use traits::UserProfileRepository;