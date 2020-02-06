/*
 * Copyright (c) 2020, Мира Странная <rsxrwscjpzdzwpxaujrr@yahoo.com>
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

use actix_web::{ web, HttpResponse };
use tera::Context;
use crate::state::State;

#[macro_export]
macro_rules! try_500 {
    ($e:expr, $state:expr) => {{
        match $e {
            Ok(e) => e,
            Err(e) => { eprintln!("Error 500: {}", e); return error_500($state.clone()) },
        }
    }};
}

pub async fn error_404(state: web::Data<State>) -> HttpResponse {
    let context = Context::new();

    return HttpResponse::NotFound()
        .body(try_500!(state.tera.render("404.html", &context), state));
}

fn error_emergency_500() -> HttpResponse {
    return HttpResponse::InternalServerError().body("500 Internal Server Error");
}

pub fn error_500(state: web::Data<State>) -> HttpResponse {
    let context = Context::new();

    if let Ok(body) = state.tera.render("500.html", &context) {
        return HttpResponse::InternalServerError().body(body);
    } else {
        return error_emergency_500();
    }
}
