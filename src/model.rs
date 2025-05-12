use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Represents a user profile containing identity, contact information,
/// and optional attributes and preferences.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserProfile {
    /// Unique identifier for the user.
    pub id: String,

    /// User's email address (stored in lowercase).
    pub email: String,

    /// Optional user attributes (e.g., first name, last name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<UserAttributes>,

    /// Optional user preferences (e.g., language, newsletter settings).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferences: Option<UserPreferences>,
}

impl UserProfile {
    /// Creates a new `UserProfile` with the given ID and email.
    pub fn new(id: String, email: String) -> Self {
        Self {
            id,
            email: email.to_ascii_lowercase(),
            attributes: None,
            preferences: None,
        }
    }

    /// Sets the user ID.
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    /// Returns a cloned user ID.
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    /// Sets the email address. It will be stored in lowercase.
    pub fn set_email(&mut self, email: String) {
        self.email = email.to_ascii_lowercase();
    }

    /// Returns a reference to the email address.
    pub fn get_email(&self) -> &str {
        &self.email
    }

    /// Sets the user attributes.
    pub fn set_attributes(&mut self, attributes: Option<UserAttributes>) {
        self.attributes = attributes;
    }

    /// Returns a reference to the user attributes, if any.
    pub fn get_attributes(&self) -> Option<&UserAttributes> {
        self.attributes.as_ref()
    }

    /// Sets the user preferences.
    pub fn set_preferences(&mut self, preferences: Option<UserPreferences>) {
        self.preferences = preferences;
    }

    /// Returns a reference to the user preferences, if any.
    pub fn get_preferences(&self) -> Option<&UserPreferences> {
        self.preferences.as_ref()
    }
}

/// Represents optional personal attributes of a user, such as name and custom fields.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserAttributes {
    /// User's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// User's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// A flexible map for additional custom fields.
    #[serde(flatten, default)]
    pub extra: Map<String, Value>,
}

impl UserAttributes {
    /// Creates a new, empty set of user attributes.
    pub fn new() -> Self {
        Self {
            first_name: None,
            last_name: None,
            extra: Default::default(),
        }
    }

    /// Sets the user's first name.
    pub fn set_first_name(&mut self, first_name: String) {
        self.first_name = Some(first_name);
    }

    /// Returns a reference to the first name, if any.
    pub fn get_first_name(&self) -> Option<&String> {
        self.first_name.as_ref()
    }

    /// Sets the user's last name.
    pub fn set_last_name(&mut self, last_name: String) {
        self.last_name = Some(last_name);
    }

    /// Returns a reference to the last name, if any.
    pub fn get_last_name(&self) -> Option<&String> {
        self.last_name.as_ref()
    }

    /// Inserts a custom attribute into the `extra` map.
    pub fn set_extra(&mut self, key: impl Into<String>, value: Value) {
        self.extra.insert(key.into(), value);
    }

    /// Retrieves a custom attribute value by key.
    pub fn get_extra(&self, key: &str) -> Option<&Value> {
        self.extra.get(key)
    }
}

/// Represents a user's application preferences, such as newsletter opt-in,
/// language, currency, and other customizable settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UserPreferences {
    /// Whether the user has opted in to receive newsletters.
    pub newsletter_opt_in: bool,

    /// Preferred language (e.g., "en", "id").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Preferred currency (e.g., "USD", "EUR").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Additional custom preferences.
    #[serde(flatten, default)]
    pub extra: Map<String, Value>,
}

impl UserPreferences {
    /// Creates a new `UserPreferences` instance with default values.
    pub fn new() -> Self {
        Self {
            newsletter_opt_in: false,
            language: None,
            currency: None,
            extra: Default::default(),
        }
    }

    /// Sets the newsletter opt-in flag.
    pub fn set_newsletter_opt_in(&mut self, opt_in: bool) {
        self.newsletter_opt_in = opt_in;
    }

    /// Returns the newsletter opt-in status.
    pub fn get_newsletter_opt_in(&self) -> bool {
        self.newsletter_opt_in
    }

    /// Sets the preferred language.
    pub fn set_language(&mut self, language: String) {
        self.language = Some(language);
    }

    /// Returns a reference to the preferred language, if any.
    pub fn get_language(&self) -> Option<&String> {
        self.language.as_ref()
    }

    /// Sets the preferred currency.
    pub fn set_currency(&mut self, currency: String) {
        self.currency = Some(currency);
    }

    /// Returns a reference to the preferred currency, if any.
    pub fn get_currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }

    /// Inserts a custom preference into the `extra` map.
    pub fn set_extra(&mut self, key: impl Into<String>, value: Value) {
        self.extra.insert(key.into(), value);
    }

    /// Retrieves a custom preference value by key.
    pub fn get_extra(&self, key: &str) -> Option<&Value> {
        self.extra.get(key)
    }
}
