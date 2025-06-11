use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use utoipa::ToSchema;

use crate::api::auth::hash_password;

use crate::api::auth::RegisterPayload;

#[derive(Serialize, Deserialize, Queryable, Insertable, Selectable, ToSchema)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[schema(example = "01F8MECHZX3TBDSZ7XK4F5G9ZQ")]
    pub id: String,
    #[schema(example = "johndoe")]
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,

    pub jwt_intra_epitech: Option<String>,
    pub jwt_expires_at: Option<chrono::NaiveDateTime>,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        Self {
            id: Ulid::new().to_string(),
            username,
            password_hash: hash_password(password.as_str()),
            jwt_intra_epitech: None,
            jwt_expires_at: None,
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        hash_password(password) == self.password_hash
    }
}

impl From<RegisterPayload> for User {
    fn from(payload: RegisterPayload) -> Self {
        Self::new(payload.username, payload.password)
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct JwtPayload {
    #[schema(example = "01F8MECHZX3TBDSZ7XK4F5G9ZQ")]
    pub jwt: String,
}