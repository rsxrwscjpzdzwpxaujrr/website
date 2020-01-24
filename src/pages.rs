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

use actix_web::{ web, Responder, HttpResponse };
use tera::Context;
use rusqlite::{ params, NO_PARAMS };

use crate::errors::*;
use crate::state::State;
use crate::post::Post;

pub async fn post_redirect(link: web::Path<String>) -> impl Responder {
    return HttpResponse::PermanentRedirect()
        .header("Location", format!("/post/{}", link))
        .finish();
}

pub async fn post_index(state: web::Data<State>, link: web::Path<String>) -> HttpResponse {
    let mut context = Context::new();

    let mut stmt = try_500!(state.conn.prepare("
        SELECT
            link,
            name,
            text,
            short_text,
            date
        FROM
            posts
        WHERE
            link=?
    "), state);

    let mut rows = try_500!(stmt.query(params![link.to_string()]), state);

    let post = if let Some(row) = try_500!(rows.next(), state) {
        try_500!(Post::from_row(row), state)
    } else {
        return error_404(state.clone()).await;
    };

    context.insert("post", &post);

    return HttpResponse::Ok().body(try_500!(state.tera.render("post.html", &context), state));
}

pub async fn posts_redirect() -> impl Responder {
    return HttpResponse::PermanentRedirect()
        .header("Location", "/posts")
        .finish();
}

pub async fn posts(state: web::Data<State>) -> impl Responder {
    let mut context = Context::new();

    let mut stmt = try_500!(state.conn.prepare("
        SELECT
            link,
            name,
            text,
            short_text,
            date
        FROM
            posts
        ORDER BY
            date DESC
    "), state);

    let mut rows = try_500!(stmt.query(NO_PARAMS), state);
    let mut posts: Vec<Post> = Vec::new();

    while let Some(row) = try_500!(rows.next(), state) {
        posts.push(try_500!(Post::from_row(row), state));
    }

    context.insert("posts", &posts);

    return HttpResponse::Ok().body(try_500!(state.tera.render("posts.html", &context), state));
}

pub async fn index(state: web::Data<State>) -> HttpResponse {
    return post_index(state, web::Path::from(String::from("anketa"))).await;
}