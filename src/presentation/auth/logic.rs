use std::{env, sync::Arc};

use anyhow::Context;
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata, CoreResponseType},
    reqwest::async_http_client,
    AuthenticationFlow, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce,
    OAuth2TokenResponse, RedirectUrl, Scope,
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

    pub async fn exchange(
        &self,
        provider: AuthProvider,
        code: AuthorizationCode,
        nonce: Nonce,
    ) -> anyhow::Result<()> {
        let client = self.get_client(provider);
        let response = client
            .exchange_code(code)
            .request_async(async_http_client)
            .await
            .context("Error exchanging code")?;

        println!(
            "Google returned access token:\n{}\n",
            response.access_token().secret()
        );
        println!("Google returned scopes: {:?}", response.scopes());

        let id_token_verifier = client.id_token_verifier();
        let id_token_claims = response
            .extra_fields()
            .id_token()
            .context("ID Token not recieved")?
            .claims(&id_token_verifier, &nonce)
            .context("Id Token invalid")?;

        
        println!("Google returned ID token: {:?}", id_token_claims);

        dbg!(id_token_claims.issuer());
        dbg!(id_token_claims.subject());
        dbg!(id_token_claims.audiences());
        Ok(())
    }
}
