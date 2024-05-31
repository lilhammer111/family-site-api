mod biz;
mod infra;

use std::io;

use actix_cors::Cors;
use actix_files;
use actix_web::{App, HttpServer};
use actix_web::http::Method;
use actix_web::middleware::Logger;
use actix_web::web::{self, Data};
use deadpool_postgres::Pool;
use log::info;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use tokio_postgres::NoTls;

use biz::account::handler::{login, register};
use crate::biz::ai::handler::get_ai_response;
use crate::biz::article_category::handler::read_all_category;
use crate::biz::behavior::handler::{create_behavior, read_all_behavior_record, read_paginated_behavior};
use crate::biz::diet::handler::{create_diet_record, read_all_diet_record, read_paginated_diet_record};
use crate::biz::user::handler::{get_user_info, get_user_info_in_batches, update_user_info};
use crate::biz::file::handler::save;
use crate::biz::journal::handler::{create_journal, read_paginated_journal};
use crate::biz::health::handler::{create_health_record, read_all_health_record, read_health_record_paginated};
use crate::biz::wish::handler::{create_wish, get_paginated_wish};
use crate::infra::{
    init::Initializer,
};
use crate::infra::middleware::jwt::JwtMiddleware;


#[derive(Clone, Debug)]
struct AppState {
    jwt_secret: String,
    pool: Pool,
    path_to_static_dir: String,
    kimi_secret: String
}


#[actix_web::main]
async fn main() -> io::Result<()> {
    let initializer = Initializer::default()
        .must_init()
        .expect("Failed to init setup");

    let settings = initializer.settings().clone();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file(settings.path_to_cert_key, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(settings.path_to_cert_file).unwrap();

    let pool = settings.pg.create_pool(None, NoTls).expect("Failed to create a pg pool");

    let app_data = AppState {
        jwt_secret: settings.jwt_secret.clone(),
        pool: pool.clone(),
        path_to_static_dir: settings.path_to_static_dir.clone(),
        kimi_secret: settings.kimi_secret.clone()
    };

    let server = HttpServer::new(move || {
        let app = App::new();

        let app = app.app_data(Data::new(app_data.clone()));


        let cors = Cors::default()
            .allowed_origin("https://localhost:5173")
            .allowed_origin("https://127.0.0.1:5173")
            .supports_credentials()
            .allowed_methods(
                vec![
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                ]
            )
            .allow_any_header();

        let app = app
            .wrap(Logger::new("%a | %t | %r | %s | %Ts"))
            .wrap(cors);


        let account_scope = web::scope("/account")
            .service(login)
            .service(register);


        let user_scope = web::scope("/user")
            .wrap(JwtMiddleware)
            .service(get_user_info_in_batches)
            .service(get_user_info)
            .service(update_user_info);

        let file_scope = web::scope("/file")
            .wrap(JwtMiddleware)
            .service(save);

        let wish_scope = web::scope("/wish")
            .wrap(JwtMiddleware)
            .service(create_wish)
            .service(get_paginated_wish);

        let journal_scope = web::scope("/journal")
            .wrap(JwtMiddleware)
            .service(create_journal)
            .service(read_paginated_journal);

        let health_scope = web::scope("/health")
            .wrap(JwtMiddleware)
            .service(create_health_record)
            .service(read_health_record_paginated)
            .service(read_all_health_record);

        let diet_scope = web::scope("/diet")
            .wrap(JwtMiddleware)
            .service(create_diet_record)
            .service(read_paginated_diet_record)
            .service(read_all_diet_record);

        let behavior_scope = web::scope("/behavior")
            .wrap(JwtMiddleware)
            .service(create_behavior)
            .service(read_paginated_behavior)
            .service(read_all_behavior_record);

        let ai_scope = web::scope("/ai")
            .wrap(JwtMiddleware)
            .service(get_ai_response);

        let article_scope = web::scope("/article")
            .wrap(JwtMiddleware)
            .service(
                web::scope("/category")
                    .service(read_all_category)
            );

        let api_service = web::scope("/api")
            .service(account_scope)
            .service(user_scope)
            .service(file_scope)
            .service(wish_scope)
            .service(journal_scope)
            .service(health_scope)
            .service(diet_scope)
            .service(behavior_scope)
            .service(ai_scope)
            .service(article_scope);

        let static_file_service = web::scope("/static")
            .service(
                actix_files::Files::new("/file", &settings.path_to_static_dir)
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
        .bind_openssl(format!("{}:{}", settings.ip, settings.port), builder)?
        .run()
        .await
}
