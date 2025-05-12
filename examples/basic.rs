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
