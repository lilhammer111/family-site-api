mod biz;
mod infra;

use std::io;

use actix_cors::Cors;
use actix_files;
use actix_web::{App, HttpServer};
use actix_web::http::{header, Method};
use actix_web::middleware::Logger;
use actix_web::web::{self, Data};
use deadpool_postgres::Pool;
use log::info;
use tokio_postgres::NoTls;

use biz::account::handler::{login, register};
use crate::biz::user::handler::{get_user_info, update_user_info};
use crate::biz::file::handler::save;
use crate::infra::{
    init::Initializer,
};
use crate::infra::middleware::jwt::JwtMiddleware;


#[derive(Clone, Debug)]
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

    let app_data = AppState {
        jwt_secret: settings.jwt_secret.clone(),
        pool: pool.clone(),
    };

    let server = HttpServer::new(move || {
        let app = App::new();

        let app = app.app_data(Data::new(app_data.clone()));


        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://127.0.0.1:5173")
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
                    header::COOKIE,
                ]
            )
            .supports_credentials();

        let app = app
            .wrap(Logger::new("%a | %t | %r | %s | %Ts"))
            .wrap(cors);


        let account_scope = web::scope("/account")
            .service(login)
            .service(register);


        let user_scope = web::scope("/user")
            .wrap(JwtMiddleware)
            .service(get_user_info)
            .service(update_user_info);

        let file_scope = web::scope("/file")
            .wrap(JwtMiddleware)
            .service(save);

        let api_service = web::scope("/api")
            .service(account_scope)
            .service(user_scope)
            .service(file_scope);

        let static_file_service = web::scope("/static")
            .wrap(JwtMiddleware)
            .service(
                actix_files::Files::new("/files", &settings.static_file_path)
                    .show_files_listing()
                    .use_etag(true)
                    .use_last_modified(true)
            );

        app.service(api_service)
            .service(static_file_service)
    });

    info!("Running on {}:{}",settings.ip, settings.port);
    info!("Log Level is {}",settings.log.level);

    server
        .bind(format!("{}:{}", settings.ip, settings.port))?
        .run()
        .await
}
