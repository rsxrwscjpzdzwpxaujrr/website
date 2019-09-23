extern crate env_logger;
extern crate time;

use std::{ fs, io::BufReader, error::Error };

use serde::Deserialize;
use serde_json::from_reader;    
use actix_web::{ web, App, HttpServer, Responder, HttpResponse, middleware };
use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };

struct State {
    template: String,
}

#[derive(Deserialize)]
struct Config {
    priv_key_file: String,
    cert_chain_file: String,
    address: String,
}

impl Config {
    fn read_from_file(path: &str) -> Result<Config, Box<Error>> {
        let buf = BufReader::new(fs::File::open(path)?);
        let config = from_reader(buf)?;

        Ok(config)
    }
}

fn index(data: web::Data<State>) -> impl Responder {
    let time = time::now();

    let time_name = 
    if time.tm_wday == 5 && time.tm_mday == 13 {
        "Пятница тринадцатого ебануться"
    } else {
        match time.tm_hour {
            06...11 => "Доброго утра",
            12...17 => "Добрый день",
            18...23 => "Доброго вечера",
            00...05 => "Спокойной ночи",
            _ => unreachable!(),
        }
    };

    return HttpResponse::Ok().body(data.template.replace("{}", time_name));
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config = Config::read_from_file("config.json").unwrap();
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder.set_private_key_file(config.priv_key_file, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(config.cert_chain_file).unwrap();

    HttpServer::new(|| { 
        App::new()
            .wrap(middleware::Logger::default())
            .data(State { template: fs::read_to_string("template.html").unwrap() })
            .route("/", web::get().to(index)) 
    })
    .bind_ssl(config.address, builder).unwrap()
    .run()
}
