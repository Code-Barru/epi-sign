use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::api::users::User;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JwtClaims {
    pub sub: Ulid,
    pub name: String,
    pub iat: usize,
    pub exp: usize,
}

impl JwtClaims {
    pub fn to_string(&self) -> Result<String, jsonwebtoken::errors::Error> {
        let jwt_secret =
            std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in the environment");

        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            self,
            &jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_ref()),
        )
    }

    pub fn from_jwt(jwt: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let jwt_secret =
            std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in the environment");

        jsonwebtoken::decode::<Self>(
            jwt,
            &jsonwebtoken::DecodingKey::from_secret(jwt_secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )
        .map(|data| data.claims)
    }

    pub fn is_valid(&self) -> bool {
        let now = chrono::Local::now().timestamp() as usize;
        self.iat <= now && self.exp >= now
    }
}

impl From<User> for JwtClaims {
    fn from(user: User) -> Self {
        let ulid = Ulid::from_string(user.id.as_str()).unwrap_or(Ulid::nil());

        Self {
            sub: ulid,
            name: user.username,
            iat: chrono::Local::now().timestamp() as usize,
            exp: (chrono::Local::now() + chrono::Duration::days(1)).timestamp() as usize,
        }
    }
}
