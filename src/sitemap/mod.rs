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
use rusqlite::{ NO_PARAMS };
use tera::Context;

use crate::errors::*;
use crate::state::State;
use crate::sitemap::url::Url;

mod url;

pub async fn sitemap(state: web::Data<State>) -> HttpResponse {
    let mut context = Context::new();

    let mut stmt = try_500!(state.conn.prepare("
        SELECT
            link,
            date
        FROM
            posts
    "), state);

    let mut rows = try_500!(stmt.query(NO_PARAMS), state);
    let mut urls: Vec<Url> = Vec::new();

    urls.push(Url::from_link("/".to_owned(), state.config.host.to_owned(), 0));

    while let Some(row) = try_500!(rows.next(), state) {
        urls.push(try_500!(Url::from_row(row, state.config.host.to_owned()), state));
    }

    context.insert("urls", &urls);

    return HttpResponse::Ok().body(try_500!(state.tera.render("sitemap.xml", &context), state));
}
