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

use actix_web::{ web, HttpResponse, HttpRequest };
use tera::Context;

use crate::errors::*;
use crate::state::State;
use crate::sitemap::url::Url;
use std::error::Error;

mod url;

pub async fn sitemap(req: HttpRequest,
                     state: web::Data<State<'_>>) -> HttpResponse {
    try_500!(sitemap_inner(req, state).await, state, req)
}

async fn sitemap_inner(_: HttpRequest,
                       state: web::Data<State<'_>>) -> Result<HttpResponse, Box<dyn Error>> {
    let mut context = Context::new();

    let mut stmt = state.conn.prepare("
        SELECT
            MAX(date),
            MAX(lastmod)
        FROM
            articles
    ")?;

    let mut rows = stmt.query([])?;
    let row = rows.next()?;
    let mut urls: Vec<Url> = Vec::new();

    let newest = match row {
        Some(row) => {
            let max_date = row.get(0)?;
            let max_lastmod = row.get(1)?;

            if max_date > max_lastmod { max_date } else { max_lastmod }
        }

        None => 0
    };

    urls.push(Url::from_link("/".to_owned(), state.config.host.to_owned(), newest));

    let mut stmt = state.conn.prepare("
        SELECT
            link,
            date,
            lastmod
        FROM
            articles
    ")?;

    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        urls.push(Url::from_row(row, state.config.host.to_owned())?);
    }

    context.insert("urls", &urls);

    Ok(HttpResponse::Ok().body(state.tera.render("sitemap.xml", &context)?))
}
