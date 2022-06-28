/*
 * Copyright (c) 2020, 2022 Мира Странная <rsxrwscjpzdzwpxaujrr@yahoo.com>
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of the GNU Affero General Public License
 * as published by the Free Software Foundation, either version 3 of
 * the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::error::Error;
use serde::Deserialize;
use tera::Context;
use actix_web::{ HttpRequest, HttpMessage, HttpResponse, cookie::Cookie, web, http::header };

use crate::errors::*;
use crate::state::State;

pub struct Auth {
    token: String,
    cookie: Cookie<'static>,
}

impl Auth {
    pub fn new(token: String) -> Result<Auth, Box<dyn Error>> {
        Auth::check_token(token.as_str())?;

        Ok(Auth { token, cookie: Cookie::named("auth") })
    }

    pub fn authorized(&self, req: &HttpRequest) -> bool {
        match req.cookie("auth") {
            Some(cookie) => { cookie.value() == self.token }
            _ => { false }
        }
    }

    pub fn auth(&mut self, token: String) -> bool {
        if token == self.token {
            self.cookie.set_value(token);
            return true;
        }

        return false;
    }

    pub fn deauth(&self, response: &mut HttpResponse) -> Result<(), actix_web::http::Error> {
        response.add_cookie(&Cookie::named("auth"))
    }

    pub fn cookie(&self) -> &Cookie {
        &self.cookie
    }

    fn check_token(token: &str) -> Result<(), Box<dyn Error>> {
        if !token.is_ascii() {
            return Err("Token should be ascii string".into());
        }

        if !token.len() < 32 {
            return Err("Token length should be over 32".into());
        }

        Ok(())
    }
}

#[derive(Deserialize)]
pub struct AuthFormData {
    token: String,
}

pub async fn auth_submit(req: HttpRequest,
                         state: web::Data<State<'_>>,
                         form: web::Form<AuthFormData>) -> HttpResponse {
    let mut response = HttpResponse::SeeOther()
        .header("Location", "/")
        .finish();

    let mut auth = try_500!(state.auth.write(), state, req);

    if auth.auth(form.token.clone()) {
        try_500!(response.add_cookie(auth.cookie()), state, req);
    }

    response
}

pub async fn auth(req: HttpRequest,
                  state: web::Data<State<'_>>) -> HttpResponse {
    try_500!(auth_inner(req, state).await, state, req)
}

pub async fn deauth(req: HttpRequest,
                    state: web::Data<State<'_>>) -> HttpResponse {
    try_500!(deauth_inner(req, state).await, state, req)
}

async fn auth_inner(req: HttpRequest,
                    state: web::Data<State<'_>>) -> Result<HttpResponse, Box<dyn Error>> {
    let mut context = Context::new();
    let auth = state.auth.read().map_err(MyError::from)?;

    context.insert("authorized", &auth.authorized(&req));

    Ok(HttpResponse::Ok().body(state.tera.render("auth.html", &context)?))
}

async fn deauth_inner(req: HttpRequest,
                      state: web::Data<State<'_>>) -> Result<HttpResponse, Box<dyn Error>> {
    let mut url = "/";

    if let Some(temp_url) = req.headers().get(header::REFERER) {
        if let Ok(temp_url) = temp_url.to_str() {
            url = temp_url;
        }
    }

    let mut response = HttpResponse::SeeOther()
        .header("Location", url)
        .finish();

    let auth = state.auth.read().map_err(MyError::from)?;

    auth.deauth(&mut response)?;

    Ok(response)
}
