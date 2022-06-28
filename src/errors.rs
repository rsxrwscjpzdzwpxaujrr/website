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

use std::fmt;
use std::error::Error;
use std::sync::PoisonError;
use actix_web::{ web, HttpResponse, HttpRequest };
use tera::Context;
use crate::state::State;

#[macro_export]
macro_rules! try_500 {
    ($e:expr, $state:expr, $req:expr) => {{
        let temp_state = $state.clone();
        let temp_req = $req.clone();

        match $e {
            Ok(e) => e,
            Err(e) => {
                if (e.to_string() == "terrorrussia") {
                    eprintln!("Error 401 Russia");
                    return error_401_russia(temp_req, temp_state)
                }

                eprintln!("Error 500: {}", e);
                return error_500(temp_req, temp_state)
            },
        }
    }};
}

#[macro_export]
macro_rules! try_emergency_500 {
    ($e:expr) => {{
        match $e {
            Ok(e) => e,
            Err(e) => { eprintln!("Error 500: {}", e); return error_emergency_500() },
        }
    }};
}

#[derive(Debug)]
pub struct MyError {
    details: String,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl<T> From<PoisonError<T>> for MyError {
    fn from(err: PoisonError<T>) -> Self {
        MyError { details: err.to_string() }
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError { details: err.to_string() }
    }
}

impl From<geoip2::Error> for MyError {
    fn from(_: geoip2::Error) -> Self {
        MyError { details: "geoip2 error".to_owned() }
    }
}

impl MyError {
    pub fn new_russia() -> Self {
        MyError { details: "terrorrussia".to_owned() }
    }
}

pub async fn error_404(req: HttpRequest,
                       state: web::Data<State<'_>>) -> HttpResponse {
    let mut context = Context::new();
    let auth = try_500!(state.auth.read(), state, req);

    context.insert("authorized", &auth.authorized(&req));

    return HttpResponse::NotFound()
        .body(try_500!(state.tera.render("404.html", &context), state, req));
}

pub fn error_401_russia(req: HttpRequest,
                        state: web::Data<State<'_>>) -> HttpResponse {
    let mut context = Context::new();
    let auth = try_500!(state.auth.read(), state, req);

    context.insert("authorized", &auth.authorized(&req));

    return HttpResponse::Unauthorized()
        .body(try_500!(state.tera.render("401_russia.html", &context), state, req));
}

pub fn error_emergency_500() -> HttpResponse {
    return HttpResponse::InternalServerError().body("500 Internal Server Error");
}

pub fn error_500(req: HttpRequest,
                 state: web::Data<State>) -> HttpResponse {
    let mut context = Context::new();
    let auth = try_500!(state.auth.read(), state, req);

    context.insert("authorized", &auth.authorized(&req));

    if let Ok(body) = state.tera.render("500.html", &context) {
        return HttpResponse::InternalServerError().body(body);
    } else {
        return error_emergency_500();
    }
}
