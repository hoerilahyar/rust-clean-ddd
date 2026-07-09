use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::{
    config::Config,
    infrastructure::security::{AccessClaims, RefreshClaims},
};

pub struct JwtService {
    encoding: EncodingKey,
    decoding: DecodingKey,
    issuer: String,
    app_name: String,
    access_expired: u64,
    refresh_expired: u64,
}

impl JwtService {
    pub fn new(config: &Config) -> Self {
        Self {
            encoding: EncodingKey::from_secret(config.jwt.secret.as_bytes()),
            decoding: DecodingKey::from_secret(config.jwt.secret.as_bytes()),
            issuer: config.jwt.issuer.clone(),
            app_name: config.app.name.clone(),
            access_expired: config.jwt.access_token_expired,
            refresh_expired: config.jwt.refresh_token_expired,
        }
    }

    pub fn generate_access_token(
        &self,
        user_id: u64,
        username: &str,
        roles: Vec<String>,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now().timestamp() as usize;

        let claims = AccessClaims {
            sub: user_id,
            username: username.to_owned(),
            roles,
            iss: self.issuer.clone(),
            iat: now,
            exp: now + self.access_expired as usize,
        };

        encode(&Header::default(), &claims, &self.encoding)
    }

    pub fn generate_refresh_token(
        &self,
        user_id: u64,
        device_id: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now().timestamp() as usize;

        let claims = RefreshClaims {
            sub: user_id,
            device_id: device_id.to_owned(),
            iss: self.issuer.clone(),
            iat: now,
            exp: now + self.refresh_expired as usize,
        };

        encode(&Header::default(), &claims, &self.encoding)
    }

    pub fn verify_access_token(
        &self,
        token: &str,
    ) -> Result<AccessClaims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_issuer(&[self.issuer.clone()]);
        validation.set_audience(&[self.app_name.clone()]); // Add audience

        Ok(decode::<AccessClaims>(token, &self.decoding, &validation)?.claims)
    }

    pub fn verify_refresh_token(
        &self,
        token: &str,
    ) -> Result<RefreshClaims, jsonwebtoken::errors::Error> {
        Ok(decode::<RefreshClaims>(token, &self.decoding, &Validation::default())?.claims)
    }

    pub fn access_token_expired(&self) -> u64 {
        self.access_expired
    }

    pub fn refresh_token_expired(&self) -> u64 {
        self.refresh_expired
    }
}
