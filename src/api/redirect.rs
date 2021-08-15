use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
use url::Url;

use serde::Deserialize;
use std::{env, str};

use crate::api::utils::R2D2Error;
use crate::api::utils::{DieselError, OAuthError};
use crate::models::*;
use actix_web::{web, HttpResponse};
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::backend::UsesAnsiSavepointSyntax;
use diesel::connection::AnsiTransactionManager;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::types::FromSql;
use diesel::types::HasSqlType;
use r2d2::PooledConnection;

#[derive(Deserialize)]
pub struct Info {
    state: String,
    session_state: String,
    code: String,
}

pub async fn index<T>(
    pool: web::Data<r2d2::Pool<ConnectionManager<T>>>,
    info: web::Query<Info>,
) -> actix_web::Result<HttpResponse>
where
    T: Connection<TransactionManager = AnsiTransactionManager> + 'static,
    <T>::Backend: UsesAnsiSavepointSyntax,
    <T>::Backend: HasSqlType<diesel::sql_types::Bool>,
    bool: FromSql<diesel::sql_types::Bool, <T>::Backend>,
    NaiveDate: FromSql<diesel::sql_types::Date, <T>::Backend>,
    NaiveDateTime: FromSql<diesel::sql_types::Timestamp, <T>::Backend>,
    i32: FromSql<diesel::sql_types::Integer, <T as diesel::Connection>::Backend>,
    f64: FromSql<diesel::sql_types::Double, <T as diesel::Connection>::Backend>,
    f32: FromSql<diesel::sql_types::Float, <T as diesel::Connection>::Backend>,
    *const str: FromSql<diesel::sql_types::Text, <T as diesel::Connection>::Backend>,
{
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");

    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(
        ClientId::new("not-grocy".to_string()),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(
            "https://sso.selfmade4u.de/auth/realms/not-grocy/protocol/openid-connect/auth"
                .to_string(),
        )
        .unwrap(),
        Some(
            TokenUrl::new(
                "https://sso.selfmade4u.de/auth/realms/not-grocy/protocol/openid-connect/token"
                    .to_string(),
            )
            .unwrap(),
        ),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new("http://localhost:8080/redirect".to_string()).unwrap());

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Once the user has been redirected to the redirect URL, you'll have access to the
    // TODO FIXME authorization code. For security reasons, your code should verify that the `state`
    // parameter returned by the server matches `csrf_state`.

    // Now you can trade it for an access token.
    let token_result = client
        .exchange_code(AuthorizationCode::new(info.code.to_string()))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .map_err(OAuthError)?;

    // Unwrapping token_result will either produce a Token or a RequestTokenError.

    println!("{:#?}", token_result);

    //let connection = pool.get().map_err(R2D2Error)?;
    //let json = web::block(move || action(connection).map_err(DieselError)).await??;
    //Ok(HttpResponse::Ok().json(json))
    Ok(HttpResponse::Ok().finish())
}
