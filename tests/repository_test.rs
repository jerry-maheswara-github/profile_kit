use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use profile_kit::error::ProfileKitError;
use profile_kit::model::UserProfile;
use profile_kit::repository::UserProfileRepository;

pub struct InMemoryUserProfileRepository {
    storage: Arc<RwLock<HashMap<String, UserProfile>>>,
}

impl InMemoryUserProfileRepository {
    pub fn new() -> Self {
        InMemoryUserProfileRepository {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl UserProfileRepository for InMemoryUserProfileRepository {
    fn get_by_id(&self, id: String) -> Result<Option<UserProfile>, ProfileKitError> {
        let storage = self.storage.read().map_err(|_| ProfileKitError::StorageError)?;
        Ok(storage.get(&id).cloned())
    }

    fn create(&self, profile: UserProfile) -> Result<(), ProfileKitError> {
        let mut storage = self.storage.write().map_err(|_| ProfileKitError::StorageError)?;
        if storage.contains_key(&profile.id) {
            return Err(ProfileKitError::AlreadyExists);
        }
        storage.insert(profile.id.clone(), profile);
        Ok(())
    }

    fn update(&self, profile: UserProfile) -> Result<(), ProfileKitError> {
        let mut storage = self.storage.write().map_err(|_| ProfileKitError::StorageError)?;
        if !storage.contains_key(&profile.id) {
            return Err(ProfileKitError::NotFound);
        }
        storage.insert(profile.id.clone(), profile);
        Ok(())
    }

    fn delete(&self, id: String) -> Result<(), ProfileKitError> {
        let mut storage = self.storage.write().map_err(|_| ProfileKitError::StorageError)?;
        if storage.remove(&id).is_none() {
            return Err(ProfileKitError::NotFound);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use profile_kit::error::ProfileKitError;
    use profile_kit::model::{UserAttributes, UserProfile};
    use profile_kit::repository::UserProfileRepository;
    use crate::InMemoryUserProfileRepository;

    fn sample_profile() -> UserProfile {
        UserProfile::new("u1".to_string() , "test@example.com".to_string())
    }

    #[test]
    fn test_create_and_get() {
        let repo = InMemoryUserProfileRepository::new();
        let profile = sample_profile();
        repo.create(profile.clone()).unwrap();
        let fetched = repo.get_by_id("u1".to_string()).unwrap();
        assert_eq!(fetched, Some(profile));
    }

    #[test]
    fn test_duplicate_create() {
        let repo = InMemoryUserProfileRepository::new();
        let profile = sample_profile();
        repo.create(profile.clone()).unwrap();
        let result = repo.create(profile);
        assert!(matches!(result, Err(ProfileKitError::AlreadyExists)));
    }

    #[test]
    fn test_update_existing() {
        let repo = InMemoryUserProfileRepository::new();
        let mut profile = sample_profile();
        repo.create(profile.clone()).unwrap();
        
        let mut attr = UserAttributes::new();
        attr.set_first_name("Updated Name".to_string());

        profile.set_attributes(Some(attr));
        repo.update(profile.clone()).unwrap();

        let fetched = repo.get_by_id("u1".to_string()).unwrap();
        assert_eq!(fetched, Some(profile));
    }

    #[test]
    fn test_update_nonexistent() {
        let repo = InMemoryUserProfileRepository::new();
        let profile = sample_profile();
        let result = repo.update(profile);
        assert!(matches!(result, Err(ProfileKitError::NotFound)));
    }

    #[test]
    fn test_delete() {
        let repo = InMemoryUserProfileRepository::new();
        let profile = sample_profile();
        repo.create(profile.clone()).unwrap();
        repo.delete("u1".to_string()).unwrap();
        let fetched = repo.get_by_id("u1".to_string()).unwrap();
        assert!(fetched.is_none());
    }

    #[test]
    fn test_delete_nonexistent() {
        let repo = InMemoryUserProfileRepository::new();
        let result = repo.delete("does_not_exist".to_string());
        assert!(matches!(result, Err(ProfileKitError::NotFound)));
    }
}
