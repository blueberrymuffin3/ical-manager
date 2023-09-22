use std::{
    borrow::Cow,
    convert::Infallible,
    marker::{PhantomData, Send},
};

use anyhow::anyhow;
use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::{IntoResponse, Response},
};

use openidconnect::{CsrfToken, Nonce};
use tower_cookies::{
    cookie::{Expiration, SameSite},
    Cookie, Cookies, Key, SignedCookies,
};

fn new_cookie<'c, N, V>(name: N, value: V) -> Cookie<'c>
where
    N: Into<Cow<'c, str>>,
    V: Into<Cow<'c, str>>,
{
    let mut cookie = Cookie::new(name, value);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_expires(Expiration::Session);
    cookie.set_path("/");
    cookie
}

pub trait CookieType: Sized {
    fn name() -> &'static str;
    type Rejection: Into<anyhow::Error>;
    fn encode(&self) -> String;
    fn decode(value: &str) -> Result<Self, Self::Rejection>;
}

pub trait CookieTypeGenerate: CookieType {
    fn generate() -> Self;
}

impl CookieType for CsrfToken {
    fn name() -> &'static str {
        "csrf-token"
    }

    type Rejection = Infallible;
    fn decode(value: &str) -> Result<Self, Self::Rejection> {
        Ok(CsrfToken::new(value.to_owned()))
    }

    fn encode(&self) -> String {
        self.secret().to_owned()
    }
}

impl CookieTypeGenerate for CsrfToken {
    fn generate() -> Self {
        CsrfToken::new_random()
    }
}

impl CookieType for Nonce {
    fn name() -> &'static str {
        "oauth-nonce"
    }

    type Rejection = Infallible;
    fn decode(value: &str) -> Result<Self, Self::Rejection> {
        Ok(Nonce::new(value.to_owned()))
    }

    fn encode(&self) -> String {
        self.secret().to_owned()
    }
}

impl CookieTypeGenerate for Nonce {
    fn generate() -> Self {
        Nonce::new_random()
    }
}

fn read_cookie<T: CookieType>(cookies: &SignedCookies) -> Option<T> {
    match cookies.get(T::name()) {
        Some(cookie) => match T::decode(cookie.value()) {
            Ok(value) => Some(value),
            Err(err) => {
                log::error!("Error decoding cookie {}: {:?}", T::name(), anyhow!(err));
                None
            }
        },
        None => None,
    }
}

pub struct AutoCookie<T>(pub T);

#[async_trait]
impl<S: Send + Sync, T> FromRequestParts<S> for AutoCookie<T>
where
    tower_cookies::Key: FromRef<S>,
    T: CookieTypeGenerate,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let key = Key::from_ref(state);
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(IntoResponse::into_response)?
            .signed(&key);

        let value = match read_cookie(&cookies) {
            Some(value) => value,
            None => {
                let value = T::generate();
                let cookie = new_cookie(T::name(), value.encode());
                cookies.add(cookie);
                value
            }
        };
        Ok(Self(value))
    }
}

pub struct GenerateNewCookie<T>(pub T);

#[async_trait]
impl<S: Send + Sync, T> FromRequestParts<S> for GenerateNewCookie<T>
where
    tower_cookies::Key: FromRef<S>,
    T: CookieTypeGenerate,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let key = Key::from_ref(state);
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(IntoResponse::into_response)?
            .signed(&key);

        let value = T::generate();
        let cookie = new_cookie(T::name(), value.encode());
        cookies.add(cookie);
        Ok(Self(value))
    }
}

pub struct ReadCookie<T>(pub Option<T>);

#[async_trait]
impl<S: Send + Sync, T> FromRequestParts<S> for ReadCookie<T>
where
    tower_cookies::Key: FromRef<S>,
    T: CookieType,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let key = Key::from_ref(state);
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(IntoResponse::into_response)?
            .signed(&key);

        Ok(Self(read_cookie(&cookies)))
    }
}

pub struct SetCookie<T> {
    key: Key,
    cookies: Cookies,
    _phantom_data: PhantomData<T>,
}

impl<T: CookieType + 'static> SetCookie<T> {
    pub fn set(&self, value: &T) {
        self.cookies
            .signed(&self.key)
            .add(new_cookie(T::name(), value.encode()))
    }
    pub fn delete(&self) {
        self.cookies.remove(new_cookie(T::name(), ""))
    }
}

#[async_trait]
impl<S: Send + Sync, T> FromRequestParts<S> for SetCookie<T>
where
    tower_cookies::Key: FromRef<S>,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let key = Key::from_ref(state);
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(IntoResponse::into_response)?;

        Ok(Self {
            key,
            cookies,
            _phantom_data: PhantomData,
        })
    }
}
