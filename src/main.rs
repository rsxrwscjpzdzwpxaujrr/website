/*
 * Copyright (c) 2019-2020, 2022 Мира Странная <rsxrwscjpzdzwpxaujrr@yahoo.com>
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

#[macro_use]
mod errors;
mod state;
mod post;
mod config;
mod pages;
mod sitemap;
mod auth;

use std::fs;
use std::path::Path;
use std::sync::RwLock;
use actix_web::{ web, App, middleware, HttpServer, HttpResponse, HttpRequest };
use actix_files::Files;
use geoip2::{ Country, Reader };
use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };

use errors::*;
use state::State;
use config::Config;
use pages::*;
use sitemap::sitemap;
use crate::auth::*;

async fn redirect(req: HttpRequest,
                  host: web::Data<String>) -> HttpResponse {
    let uri_parts: actix_web::http::uri::Parts = req.uri().to_owned().into_parts();
    let path_and_query = try_emergency_500!(
        uri_parts.path_and_query.ok_or("Can not get path_and_query")
    );

    return HttpResponse::PermanentRedirect().header(
        "Location",
        format!("https://{}{}",
            host.get_ref(),
            path_and_query.as_str()
        )
    ).finish();
}

fn init_reader<'a, P: AsRef<Path>>(path: P) -> Result<Reader<'a, Country<'a>>, MyError> {
    Ok(Reader::<Country>::from_bytes(Box::leak(fs::read(path)?.into_boxed_slice()))?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use std::sync::Arc;

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config = Arc::new(Config::read_from_file("config.json")
        .expect("Config reading failed"));

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .expect("SSL Acceptor Builder creating failed");

    builder.set_private_key_file(&config.priv_key_file, SslFiletype::PEM)
        .expect("SSL private key file setting failed");

    builder.set_certificate_chain_file(&config.cert_chain_file)
        .expect("SSL certificate chain file setting failed");

    let config_temp = config.clone();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(config_temp.host.clone())
            .default_service(web::route().to(redirect))
    })
    .bind(format!("{}:80", config.host))?
    .run();

    let config_temp = config.clone();

    HttpServer::new(move || {
        let state = State {
            tera: tera::Tera::new(&config_temp.templates)
                .expect("Tera template rendering failed"),

            conn: rusqlite::Connection::open(&config_temp.database)
                .expect("Database opening failed"),

            config: config_temp.clone(),

            auth: RwLock::new(Auth::new(config_temp.token.clone())
                .expect("Auth creation failed")),

            geoip_reader: match init_reader(&config_temp.geoip_db_file) {
                Ok(result) => Some(result),
                Err(e) => {
                    eprintln!("geoip2 init error: {}", e.to_string());
                    None
                },
            }
        };

        App::new()
            .wrap(middleware::Logger::default())
            .data(state)
            .service(web::resource("/articles/{link}/")
                .route(web::get().to(article_redirect))
            )
            .service(web::resource("/articles/{link}")
                .route(web::get().to(article_index))
            )
            .service(web::resource("/articles/hidden/{link}/")
                .route(web::get().to(hidden_article_redirect))
            )
            .service(web::resource("/articles/hidden/{link}")
                .route(web::get().to(hidden_article_index))
            )
            .service(web::resource("/articles/")
                .route(web::get().to(articles_redirect))
            )
            .service(web::resource("/articles")
                .route(web::get().to(articles))
            )
            .service(web::resource("/post/{link}/")
                .route(web::get().to(post_index))
            )
            .service(web::resource("/post/{link}")
                .route(web::get().to(post_index))
            )
            .service(web::resource("/posts/")
                .route(web::get().to(posts))
            )
            .service(web::resource("/posts")
                .route(web::get().to(posts))
            )
            .service(web::resource("/auth")
                .route(web::post().to(auth_submit))
                .route(web::get().to(auth))
            )
            .service(web::resource("/deauth")
                .route(web::get().to(deauth))
            )
            .service(web::resource("/sitemap.xml")
                .route(web::get().to(sitemap))
            )
            .service(web::resource("/")
                .route(web::get().to(index))
            )
            .service(Files::new("/", "static/"))
            .default_service(
                web::get().to(error_404)
            )
    })
    .bind_openssl(format!("{}:443", config.host), builder)?
    .run()
    .await
}
