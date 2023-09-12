use std::{
    borrow::Cow,
    convert::Infallible,
    marker::{PhantomData, Send},
};

use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::{IntoResponse, Response},
};

use openidconnect::{CsrfToken, Nonce};
use tower_cookies::{
    cookie::{Expiration, SameSite},
    Cookie, Cookies, Key,
};

fn new_cookie<'c, N, V>(name: N, value: V) -> Cookie<'c>
where
    N: Into<Cow<'c, str>>,
    V: Into<Cow<'c, str>>,
{
    let mut cookie = Cookie::new(name, value);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_expires(Expiration::Session);
    cookie
}

pub trait CookieType: Sized {
    fn name() -> &'static str;
    fn generate() -> Self;
    type Rejection: IntoResponse;
    fn encode(&self) -> String;
    fn decode(value: &str) -> Result<Self, Self::Rejection>;
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

    fn generate() -> Self {
        Nonce::new_random()
    }
}

pub struct AutoCookie<T>(pub T);

#[async_trait]
impl<S: Send + Sync, T> FromRequestParts<S> for AutoCookie<T>
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

        let value = match cookies.get(T::name()) {
            Some(cookie) => T::decode(cookie.value()).map_err(IntoResponse::into_response)?,
            None => {
                let token = T::generate();
                let cookie = new_cookie(T::name(), token.encode());
                cookies.add(cookie);
                token
            }
        };
        Ok(Self(value))
    }
}

pub struct NewCookie<T>(pub T);

#[async_trait]
impl<S: Send + Sync, T> FromRequestParts<S> for NewCookie<T>
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

        let value = T::generate();
        let cookie = new_cookie(T::name(), value.encode());
        cookies.add(cookie);
        Ok(Self(value))
    }
}

pub struct DeleteCookie<T>(pub Option<T>);

#[async_trait]
impl<S: Send + Sync, T> FromRequestParts<S> for DeleteCookie<T>
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

        let value = match cookies.get(T::name()) {
            Some(cookie) => {
                let value = T::decode(cookie.value()).map_err(IntoResponse::into_response)?;
                cookies.remove(cookie);
                Some(value)
            }
            None => None,
        };
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

        let value = match cookies.get(T::name()) {
            Some(cookie) => Some(T::decode(cookie.value()).map_err(IntoResponse::into_response)?),
            None => None,
        };
        Ok(Self(value))
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
        self.cookies
            .remove(new_cookie(T::name(), T::generate().encode()))
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
