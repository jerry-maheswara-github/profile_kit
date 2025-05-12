#[cfg(test)]
mod tests {
    use serde_json::json;
    use profile_kit::model::{UserAttributes, UserPreferences, UserProfile};

    #[test]
    fn test_user_profile_basic_setters_getters() {
        let mut profile = UserProfile::new("123".into(), "User@Email.COM".into());
        assert_eq!(profile.get_id(), "123");
        assert_eq!(profile.get_email(), "user@email.com"); // lowercase check

        profile.set_id("456".into());
        profile.set_email("Another@Email.com".into());

        assert_eq!(profile.get_id(), "456");
        assert_eq!(profile.get_email(), "another@email.com");
    }

    #[test]
    fn test_user_attributes_setters_getters() {
        let mut attrs = UserAttributes::new();
        attrs.set_first_name("John".into());
        attrs.set_last_name("Doe".into());
        attrs.set_extra("age", json!(30));

        assert_eq!(attrs.get_first_name(), Some(&"John".to_string()));
        assert_eq!(attrs.get_last_name(), Some(&"Doe".to_string()));
        assert_eq!(attrs.get_extra("age"), Some(&json!(30)));
    }

    #[test]
    fn test_user_preferences_setters_getters() {
        let mut prefs = UserPreferences::new();
        prefs.set_newsletter_opt_in(true);
        prefs.set_language("en".into());
        prefs.set_currency("USD".into());
        prefs.set_extra("theme".to_string(), json!("dark"));

        assert!(prefs.get_newsletter_opt_in());
        assert_eq!(prefs.get_language(), Some(&"en".to_string()));
        assert_eq!(prefs.get_currency(), Some(&"USD".to_string()));
        assert_eq!(prefs.get_extra("theme"), Some(&json!("dark")));
    }

    #[test]
    fn test_user_profile_full_structure() {
        let mut attrs = UserAttributes::new();
        attrs.set_first_name("Alice".into());
        attrs.set_last_name("Smith".into());
        attrs.set_extra("nickname", json!("Al"));

        let mut prefs = UserPreferences::new();
        prefs.set_newsletter_opt_in(true);
        prefs.set_language("id".into());
        prefs.set_currency("IDR".into());
        prefs.set_extra("timezone".to_string(), json!("Asia/Jakarta"));

        let mut profile = UserProfile::new("789".into(), "alice@example.com".into());
        profile.set_attributes(Some(attrs.clone()));
        profile.set_preferences(Some(prefs.clone()));

        assert_eq!(profile.get_attributes(), Some(&attrs));
        assert_eq!(profile.get_preferences(), Some(&prefs));
    }

    #[test]
    fn test_serialization_deserialization() {
        let mut profile = UserProfile::new("321".into(), "Test@Example.com".into());
        let mut attrs = UserAttributes::new();
        attrs.set_first_name("Jane".into());
        attrs.set_extra("extra_field", json!("value"));

        let mut prefs = UserPreferences::new();
        prefs.set_currency("EUR".into());

        profile.set_attributes(Some(attrs));
        profile.set_preferences(Some(prefs));

        let json_str = serde_json::to_string(&profile).unwrap();
        let deserialized: UserProfile = serde_json::from_str(&json_str).unwrap();

        assert_eq!(profile, deserialized);
    }
}
