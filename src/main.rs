mod biz;
mod infra;
mod error;

use std::io;

use actix_cors::Cors;
use actix_web::{HttpServer, App, web};
use actix_web::http::{Method, header};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use log::info;
use tokio_postgres::NoTls;

use biz::account::handler::{login, register};
use crate::infra::init::Initializer;


#[actix_web::main]
async fn main() -> io::Result<()> {
    let initializer = Initializer::default()
        .must_init()
        .expect("Failed to init setup");

    let settings = initializer.settings();

    let pool = settings.pg.create_pool(None, NoTls).expect("Failed to create a pg pool");

    let server = HttpServer::new(move || {
        let app = App::new();

        let app = app.app_data(Data::new(pool.clone()));

        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(
                vec![
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                ]
            )
            .allowed_headers(
                vec![
                    header::AUTHORIZATION,
                    header::ACCEPT,
                    header::CONTENT_TYPE,
                ]
            )
            .max_age(3600);

        let app = app.wrap(cors);

        let app = app.wrap(Logger::new("%a | %t | %r | %s | %Ts"));

        let account = web::scope("/account")
            .service(login)
            .service(register);

        let api = web::scope("/api").service(account);

        app.service(api)
    });

    info!("Running on {}:{}",settings.ip, settings.port);
    info!("Log Level is {}",settings.log.level);

    server
        .bind(format!("{}:{}", settings.ip, settings.port))?
        .run()
        .await
}
