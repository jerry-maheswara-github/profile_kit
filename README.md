# profile_kit

A flexible, lightweight Rust library for managing user profiles — framework-agnostic, developer-friendly, and production-ready.

---

## ✨ Features

- 🔌 Pluggable profile storage via trait abstraction
- ⚙️ Designed for composability in any Rust project
- 🌐 Supports user preferences (language, currency, newsletter opt-in)
- 🧪 Easy to test with in-memory implementation
- 📦 Lightweight dependencies, ready for web or backend apps

---

## 🚀 Quick Start

```rust
use serde_json::{json, Value};
use uuid::Uuid;
use profile_kit::model::{UserAttributes, UserPreferences, UserProfile};

fn main() {
    let mut prefs = UserPreferences::new();
    prefs.set_newsletter_opt_in(true);
    prefs.set_extra("theme".to_string(), json!("dark"));
    prefs.set_extra("custom_field".to_string(), json!({"key": "value"}));

    let mut attrs = UserAttributes::new();
    attrs.set_first_name("John".to_string());
    attrs.set_last_name("Doe".to_string());
    attrs.set_extra("theme", Value::String("dark".into()));
    attrs.set_extra("roles", json!(vec!["admin", "editor"]));

    let id = Uuid::now_v7().simple();
    let email = "".to_string();
    let mut profile = UserProfile::new(id.to_string(), email);
    profile.set_email("john.doe@example.com".to_string());
    profile.set_attributes(Some(attrs));
    profile.set_preferences(Some(prefs));
    
    let serialized = serde_json::to_string_pretty(&profile).expect("REASON");
    eprintln!("Serialized profile: {}", serialized);

    let deserialized: UserProfile = serde_json::from_str(&serialized).expect("REASON");
    eprintln!("Deserialized profile: {:?}", deserialized);
    
    eprintln!("roles : {:?}", deserialized.get_attributes().expect("REASON").get_extra("roles"));
    eprintln!("first_name : {:?}", deserialized.get_attributes().expect("REASON").get_first_name());
}
```

---

## 🔧 Trait-based Storage Example

```rust
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

use uuid::Uuid;
use profile_kit::model;

fn main() {
    let repo = InMemoryUserProfileRepository::new();
    let id= Uuid::now_v7().simple();
    let mut user = UserProfile::new(id.to_string(), "test@example.com".into());
    user.set_email("john.doe@example.com".to_string());
    
    repo.create(user).expect("Failed to create user");
    
    eprintln!("id {:?}", id.to_string());
    eprintln!("id {:?}", repo.get_by_id(id.to_string()));
}
```

---

## 📖 License

This project is licensed under the Apache-2.0 license. [LICENSE](http://www.apache.org/licenses/LICENSE-2.0.txt)

---

## 🧑‍ Author
Jerry Maheswara <jerrymaheswara@gmail.com>

---

## ❤️ Built with Love in Rust

This project is built with ❤️ using **Rust** — a systems programming language that is safe, fast, and concurrent. Rust is the perfect choice for building reliable and efficient applications.

---

## 👋 Contributing

Pull requests, issues, and feedback are welcome!  
If you find this crate useful, give it a ⭐ and share it with others in the Rust community.

---