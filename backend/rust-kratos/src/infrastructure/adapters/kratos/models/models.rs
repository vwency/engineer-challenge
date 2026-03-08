use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct KratosIdentity {
    pub id: String,
    pub schema_id: String,
    pub traits: IdentityTraits,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct IdentityTraits {
    pub email: String,
    pub username: String,
    pub geo_location: Option<String>,
}

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct KratosSession {
    pub id: String,
    pub active: bool,
    pub identity: KratosIdentity,
}
