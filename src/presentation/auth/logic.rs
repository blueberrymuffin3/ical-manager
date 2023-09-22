use std::{env, sync::Arc};

use anyhow::Context;
use chrono::{DateTime, Duration, Utc};
use openidconnect::{
    core::{CoreClient, CoreIdTokenClaims, CoreProviderMetadata, CoreResponseType},
    reqwest::async_http_client,
    AuthenticationFlow, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce,
    RedirectUrl, Scope,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{
    data::user::{User, UserData, UserOAuthLink},
    presentation::cookies::CookieType,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuthProvider {
    Google,
}

#[derive(Clone)]
pub struct AuthManager(Arc<AuthManagerInner>);

struct AuthManagerInner {
    google: CoreClient,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaim {
    pub user_id: i64,
    pub expires: chrono::DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum AuthClaimDecodeError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error("AuthClaim expired at {}", .0)]
    Expired(DateTime<Utc>),
}

impl CookieType for AuthClaim {
    fn name() -> &'static str {
        "auth-claim"
    }

    type Rejection = AuthClaimDecodeError;

    fn encode(&self) -> String {
        serde_json::to_string(&self).expect("Serialization should not fail")
    }

    fn decode(value: &str) -> Result<Self, Self::Rejection> {
        let value: AuthClaim = serde_json::from_str(value)?;
        if value.expires < Utc::now() {
            return Err(AuthClaimDecodeError::Expired(value.expires));
        }

        Ok(value)
    }
}

const DEFAULT_AUTH_CLAIM_EXPIRY_HOURS: i64 = 6;

impl AuthManager {
    fn get_client(&self, provider: AuthProvider) -> &CoreClient {
        match provider {
            AuthProvider::Google => &self.0.google,
        }
    }

    pub async fn new() -> anyhow::Result<Self> {
        let google_client_id = ClientId::new(
            env::var("GOOGLE_CLIENT_ID")
                .context("Missing the GOOGLE_CLIENT_ID environment variable.")?,
        );
        let google_client_secret = ClientSecret::new(
            env::var("GOOGLE_CLIENT_SECRET")
                .context("Missing the GOOGLE_CLIENT_SECRET environment variable.")?,
        );
        let google_issuer_url: IssuerUrl =
            IssuerUrl::new("https://accounts.google.com".to_string())
                .context("Invalid issuer URL")?;

        let google_redirect_url = RedirectUrl::new(
            env::var("GOOGLE_OAUTH_REDIRECT_URL")
                .context("Missing the GOOGLE_OAUTH_REDIRECT_URL environment variable.")?,
        )
        .context("Invalid GOOGLE_OAUTH_REDIRECT_URL")?;

        let google_metadata =
            CoreProviderMetadata::discover_async(google_issuer_url, async_http_client)
                .await
                .context("Error discovering google OIDC provider")?;

        let google = CoreClient::from_provider_metadata(
            google_metadata,
            google_client_id,
            Some(google_client_secret),
        )
        .set_redirect_uri(google_redirect_url);

        Ok(Self(Arc::new(AuthManagerInner { google })))
    }

    pub async fn make_url(&self, provider: AuthProvider, csrf: CsrfToken, nonce: Nonce) -> String {
        let client = self.get_client(provider);

        let (authorize_url, _, _) = client
            .authorize_url(
                AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
                || csrf,
                || nonce,
            )
            // .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .url();

        authorize_url.into()
    }

    async fn exchange(
        &self,
        provider: AuthProvider,
        code: AuthorizationCode,
        nonce: Nonce,
    ) -> anyhow::Result<CoreIdTokenClaims> {
        let client = self.get_client(provider);
        let response = client
            .exchange_code(code)
            .request_async(async_http_client)
            .await
            .context("Error exchanging code")?;

        let id_token_verifier = client.id_token_verifier();
        let id_token_claims = response
            .extra_fields()
            .id_token()
            .context("ID Token not received")?
            .claims(&id_token_verifier, &nonce)
            .context("Id Token invalid")?;

        Ok(id_token_claims.to_owned())
    }

    pub async fn setup_user(
        &self,
        provider: AuthProvider,
        code: AuthorizationCode,
        nonce: Nonce,
        pool: &SqlitePool,
    ) -> anyhow::Result<AuthClaim> {
        let claims = self.exchange(provider, code, nonce).await?;

        let mut txn = pool.begin().await?;
        let link =
            UserOAuthLink::select_by_issuer_subject(claims.issuer(), claims.subject(), &mut txn)
                .await?;

        // Fetch the user, or create a new user
        let user_id = match link {
            Some(link) => link.id,
            None => {
                let user = User::create(&mut txn).await?;
                let link = UserOAuthLink {
                    id: user.id,
                    issuer: claims.issuer().to_string(),
                    subject: claims.subject().to_string(),
                };
                link.create(&mut txn).await?;
                user.id
            }
        };

        let user_icon = claims
            .picture()
            .and_then(|claim| claim.get(None))
            .map(|x| x.to_string());
        let user_name = claims
            .name()
            .and_then(|claim| claim.get(None))
            .map(|x| x.to_string());

        User {
            id: user_id,
            data: UserData {
                icon: user_icon,
                name: user_name,
            },
        }
        .update(&mut txn)
        .await?;

        txn.commit().await?;

        Ok(AuthClaim {
            user_id: user_id,
            expires: Utc::now() + Duration::hours(DEFAULT_AUTH_CLAIM_EXPIRY_HOURS),
        })
    }
}
