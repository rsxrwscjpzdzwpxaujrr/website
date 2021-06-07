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

use std::error::Error;

use actix_web::{ web, Responder, HttpResponse, HttpRequest };
use tera::Context;
use rusqlite::params;

use crate::errors::*;
use crate::state::State;
use crate::post::Post;

pub async fn article_redirect(link: web::Path<String>) -> impl Responder {
    return HttpResponse::PermanentRedirect()
        .header("Location", format!("/articles/{}", link))
        .finish();
}

pub async fn article_index(req: HttpRequest,
                           state: web::Data<State>,
                           link: web::Path<String>) -> HttpResponse {
    try_500!(article_index_inner(req, state, link).await, state, req)
}

pub async fn hidden_article_redirect(link: web::Path<String>) -> impl Responder {
    HttpResponse::PermanentRedirect()
        .header("Location", format!("/articles/hidden/{}", link))
        .finish()
}

pub async fn hidden_article_index(req: HttpRequest,
                                  state: web::Data<State>,
                                  link: web::Path<String>) -> HttpResponse {
    try_500!(hidden_article_index_inner(req, state, link).await, state, req)
}

pub async fn articles_redirect() -> impl Responder {
    HttpResponse::PermanentRedirect()
        .header("Location", "/articles")
        .finish()
}

pub async fn articles(req: HttpRequest,
                      state: web::Data<State>) -> HttpResponse {
    try_500!(articles_inner(req, state).await, state, req)
}

pub async fn post_index(link: web::Path<String>) -> HttpResponse {
    HttpResponse::PermanentRedirect()
        .header("Location", format!("/articles/{}", link))
        .finish()
}

pub async fn posts() -> impl Responder {
    HttpResponse::PermanentRedirect()
        .header("Location", "/articles")
        .finish()
}

pub async fn index(req: HttpRequest, state: web::Data<State>) -> impl Responder {
    articles(req, state).await
}

async fn article_index_inner(req: HttpRequest,
                             state: web::Data<State>,
                             link: web::Path<String>) -> Result<HttpResponse, Box<dyn Error>> {
    let mut context = Context::new();
    let auth = state.auth.read().map_err(MyError::from)?;

    context.insert("authorized", &auth.authorized(&req));

    let mut stmt = state.conn.prepare("
        SELECT *
        FROM
            articles
        WHERE
            link=?
    ")?;

    let mut rows = stmt.query(params![link.to_string()])?;

    let post = if let Some(row) = rows.next()? {
        Post::from_row(row)?
    } else {
        return Ok(error_404(req.clone(), state.clone()).await);
    };

    context.insert("post", &post);

    Ok(HttpResponse::Ok().body(state.tera.render("post.html", &context)?))
}

async fn hidden_article_index_inner(req: HttpRequest,
                                    state: web::Data<State>,
                                    link: web::Path<String>) -> Result<HttpResponse, Box<dyn Error>> {
    let mut context = Context::new();
    let auth = state.auth.read().map_err(MyError::from)?;

    context.insert("authorized", &auth.authorized(&req));

    let mut stmt = state.conn.prepare("
        SELECT *
        FROM
            hidden_articles
        WHERE
            link=?
    ")?;

    let mut rows = stmt.query(params![link.to_string()])?;

    let post = if let Some(row) = rows.next()? {
        Post::from_row(row)?
    } else {
        return Ok(error_404(req.clone(), state.clone()).await);
    };

    context.insert("post", &post);

    Ok(HttpResponse::Ok().body(state.tera.render("post.html", &context)?))
}

async fn articles_inner(req: HttpRequest,
                        state: web::Data<State>) -> Result<HttpResponse, Box<dyn Error>> {
    let mut context = Context::new();
    let auth = state.auth.read().map_err(MyError::from)?;

    context.insert("authorized", &auth.authorized(&req));

    let mut stmt = state.conn.prepare("
        SELECT *
        FROM
            articles
        WHERE
            dnshow=0
        ORDER BY
            date DESC
    ")?;

    let mut rows = stmt.query([])?;
    let mut posts: Vec<Post> = Vec::new();

    while let Some(row) = rows.next()? {
        posts.push(Post::from_row(row)?);
    }

    context.insert("posts", &posts);

    Ok(HttpResponse::Ok().body(state.tera.render("posts.html", &context)?))
}