use anyhow::{Context, Error};
use hyper::{body, Body, Client};
use hyper_sync_rustls;
use rocket::http::hyper::header::{qitem, Accept, Authorization, UserAgent};
use rocket::http::hyper::mime::Mime;
use rocket::http::hyper::net::HttpsConnector;
use rocket::http::{Cookie, Cookies, SameSite};
use rocket::request::{self, FromRequest, Request};
use rocket::response::{Debug, Redirect};
use rocket::{get, routes, Outcome};
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::Deserialize;
use serde_json::{self, Value};

#[derive(Deserialize)]
pub struct GitHubUserInfo {
    #[serde(default)]
    name: String,
    #[serde(default)]
    email: String,
}

#[get("/login/github")]
pub fn github_login(oauth2: OAuth2<GitHubUserInfo>, mut cookies: Cookies<'_>) -> Redirect {
    oauth2
        .get_redirect(&mut cookies, &["user:read, user:email, public_repo, gist"])
        .unwrap()
}

#[get("/auth/github")]
pub fn github_callback(token: TokenResponse<GitHubUserInfo>, mut cookies: Cookies<'_>) -> Redirect {
    //NOTE: This should only be used for accessing the GitHub api (or for retrieving login info)
    cookies.add_private(
        Cookie::build("gh_api_token", token.access_token().to_string())
            .same_site(SameSite::Lax)
            .finish(),
    );

    Redirect::to("/")
}
