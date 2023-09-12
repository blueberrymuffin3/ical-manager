use std::{env, sync::Arc};

use anyhow::Context;
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata, CoreResponseType},
    reqwest::async_http_client,
    AuthenticationFlow, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, RedirectUrl, Scope,
};
use serde::{Deserialize, Serialize};

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

impl AuthManager {
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

        let redirect_url = RedirectUrl::new(
            env::var("OAUTH_REDIRECT_URL")
                .context("Missing the OAUTH_REDIRECT_URL environment variable.")?,
        )
        .context("Invalid OAUTH_REDIRECT_URL")?;

        let google_metadata =
            CoreProviderMetadata::discover_async(google_issuer_url, async_http_client)
                .await
                .context("Error discovering google OIDC provider")?;

        let google = CoreClient::from_provider_metadata(
            google_metadata,
            google_client_id,
            Some(google_client_secret),
        )
        .set_redirect_uri(redirect_url);

        Ok(Self(Arc::new(AuthManagerInner { google })))
    }

    pub async fn make_url(&self, provider: AuthProvider, csrf: CsrfToken, nonce: Nonce) -> String {
        let (authorize_url, _, _) = self
            .0
            .google
            .authorize_url(
                AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
                || csrf,
                || nonce,
            )
            // This example is requesting access to the "calendar" features and the user's profile.
            // .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .url();

        authorize_url.into()
    }
}
