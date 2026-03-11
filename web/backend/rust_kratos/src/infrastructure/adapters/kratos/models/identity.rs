use crate::domain::entities::user_profile::UserProfile;

#[derive(serde::Deserialize)]
pub struct SessionResponse {
    pub identity: Identity,
}

#[derive(serde::Deserialize)]
pub struct Identity {
    pub traits: Traits,
}

#[derive(serde::Deserialize)]
pub struct Traits {
    pub email: String,
    pub username: String,
    pub geo_location: Option<String>,
}

impl From<Traits> for UserProfile {
    fn from(t: Traits) -> Self {
        Self {
            email: t.email,
            username: t.username,
            geo_location: t.geo_location,
        }
    }
}
