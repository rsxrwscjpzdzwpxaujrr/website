/*
 * Copyright (c) 2019-2020, Мира Странная <rsxrwscjpzdzwpxaujrr@yahoo.com>
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

use std::{ fs, io::BufReader, error::Error };

use serde::Deserialize;
use serde_json::from_reader;
use actix_web::{ web, App, middleware, HttpServer, Responder, HttpResponse, HttpRequest };
use actix_files::Files;
use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };

struct State {
    template: String,
}

#[derive(Deserialize)]
struct Config {
    priv_key_file: String,
    cert_chain_file: String,
    host: String,
}

impl Config {
    fn read_from_file(path: &str) -> Result<Config, Box<dyn Error>> {
        let buf = BufReader::new(fs::File::open(path)?);
        let config = from_reader(buf)?;

        Ok(config)
    }
}

async fn index(data: web::Data<State>) -> impl Responder {
    let time = time::now();

    let time_name =
    if time.tm_wday == 5 && time.tm_mday == 13 {
        "Пятница тринадцатого ебануться"
    } else {
        match time.tm_hour {
            06..=11 => "Доброго утра",
            12..=17 => "Добрый день",
            18..=23 => "Доброго вечера",
            00..=05 => "Спокойной ночи",
            _ => unreachable!(),
        }
    };

    return HttpResponse::Ok().body(data.template.replace("{}", time_name));
}

async fn redirect(req: HttpRequest, host: web::Data<String>) -> impl Responder {
    let uri_parts: actix_web::http::uri::Parts = req.uri().to_owned().into_parts();

    return HttpResponse::PermanentRedirect().header("Location",
        format!("https://{}{}",
            host.get_ref(),
            uri_parts.path_and_query.unwrap().as_str())
        ).finish();
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config = Config::read_from_file("config.json").unwrap();
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder.set_private_key_file(config.priv_key_file, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(config.cert_chain_file).unwrap();

    let host = config.host.clone();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(host.clone())
            .default_service(web::route().to(redirect))
    })
    .bind(format!("{}:80", config.host))?
    .run();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(State { template: fs::read_to_string("template.html").unwrap() })
            .route("/", web::get().to(index))
            .service(Files::new("/", "static/"))
    })
    .bind_openssl(format!("{}:443", config.host), builder)?
    .run()
    .await
}
