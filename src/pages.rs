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

use actix_web::{ web, Responder, HttpResponse, HttpRequest };
use tera::Context;
use rusqlite::{ params, NO_PARAMS };

use crate::errors::*;
use crate::state::State;
use crate::post::Post;

pub async fn article_redirect(link: web::Path<String>) -> impl Responder {
    return HttpResponse::PermanentRedirect()
        .header("Location", format!("/articles/{}", link))
        .finish();
}

pub async fn article_index(req: HttpRequest,
                           state: web::Data<State<'_>>,
                           link: web::Path<String>) -> HttpResponse {
    let mut context = Context::new();
    let auth = &try_500!(state.auth.lock(), state, req);

    context.insert("authorized", &auth.authorized(&req));

    let mut stmt = try_500!(state.conn.prepare("
        SELECT *
        FROM
            articles
        WHERE
            link=?
    "), state, req);

    let mut rows = try_500!(stmt.query(params![link.to_string()]), state, req);

    let post = if let Some(row) = try_500!(rows.next(), state, req) {
        try_500!(Post::from_row(row), state, req)
    } else {
        return error_404(req.clone(), state.clone()).await;
    };

    context.insert("post", &post);

    return HttpResponse::Ok().body(try_500!(state.tera.render("post.html", &context), state, req));
}

pub async fn hidden_article_redirect(link: web::Path<String>) -> impl Responder {
    return HttpResponse::PermanentRedirect()
        .header("Location", format!("/articles/hidden/{}", link))
        .finish();
}

pub async fn hidden_article_index(req: HttpRequest,
                                  state: web::Data<State<'_>>,
                                  link: web::Path<String>) -> HttpResponse {
    let mut context = Context::new();
    let auth = try_500!(state.auth.lock(), state, req);

    context.insert("authorized", &auth.authorized(&req));

    let mut stmt = try_500!(state.conn.prepare("
        SELECT *
        FROM
            hidden_articles
        WHERE
            link=?
    "), state, req);

    let mut rows = try_500!(stmt.query(params![link.to_string()]), state, req);

    let post = if let Some(row) = try_500!(rows.next(), state, req) {
        try_500!(Post::from_row(row), state, req)
    } else {
        return error_404(req.clone(), state.clone()).await;
    };

    context.insert("post", &post);

    return HttpResponse::Ok().body(try_500!(state.tera.render("post.html", &context), state, req));
}

pub async fn articles_redirect() -> impl Responder {
    return HttpResponse::PermanentRedirect()
        .header("Location", "/articles")
        .finish();
}

pub async fn articles(req: HttpRequest,
                      state: web::Data<State<'_>>) -> impl Responder {
    let mut context = Context::new();
    let auth = try_500!(state.auth.lock(), state, req);

    context.insert("authorized", &auth.authorized(&req));

    let mut stmt = try_500!(state.conn.prepare("
        SELECT *
        FROM
            articles
        WHERE
            dnshow=0
        ORDER BY
            date DESC
    "), state, req);

    let mut rows = try_500!(stmt.query(NO_PARAMS), state, req);
    let mut posts: Vec<Post> = Vec::new();

    while let Some(row) = try_500!(rows.next(), state, req) {
        posts.push(try_500!(Post::from_row(row), state, req));
    }

    context.insert("posts", &posts);

    return HttpResponse::Ok().body(try_500!(state.tera.render("posts.html", &context), state, req));
}

pub async fn post_index(link: web::Path<String>) -> HttpResponse {
    return HttpResponse::PermanentRedirect()
        .header("Location", format!("/articles/{}", link))
        .finish();
}

pub async fn posts() -> impl Responder {
    return HttpResponse::PermanentRedirect()
        .header("Location", "/articles")
        .finish();
}

pub async fn index(req: HttpRequest, state: web::Data<State<'_>>) -> impl Responder {
    return articles(req, state).await;
}

