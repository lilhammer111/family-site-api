mod biz;
mod infra;

use std::io;

use actix_cors::Cors;
use actix_files;
use actix_web::{App, HttpServer, web};
use actix_web::http::{header, Method};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use deadpool_postgres::Pool;
use log::info;
use tokio_postgres::NoTls;

use biz::account::handler::{login, register};
use crate::biz::file::handler::save;
use crate::infra::{
    init::Initializer,
};

#[derive(Clone)]
struct AppState {
    jwt_secret: String,
    pool: Pool,
}


#[actix_web::main]
async fn main() -> io::Result<()> {
    let initializer = Initializer::default()
        .must_init()
        .expect("Failed to init setup");
    let settings = initializer.settings().clone();

    let pool = settings.pg.create_pool(None, NoTls).expect("Failed to create a pg pool");

    let app_state = AppState {
        jwt_secret: settings.jwt_secret,
        pool: pool.clone(),
    };


    let server = HttpServer::new(move || {
        let app = App::new();

        let app = app.app_data(Data::new(app_state.clone()));

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

        let account_scope = web::scope("/account")
            .service(login)
            .service(register)
            .service(save);

        let file_scope = web::scope("/file")
            .service(save);
        // .wrap(JwtMiddleware);

        let api = web::scope("/api")
            .service(account_scope)
            .service(file_scope);

        let static_file = web::scope("/static")
            .service(
                actix_files::Files::new("/files", &settings.static_file_path)
                    .show_files_listing()
                    .use_etag(true)
                    .use_last_modified(true)
            );

        app.service(api).service(static_file)
    });

    info!("Running on {}:{}",settings.ip, settings.port);
    info!("Log Level is {}",settings.log.level);

    server
        .bind(format!("{}:{}", settings.ip, settings.port))?
        .run()
        .await
}
