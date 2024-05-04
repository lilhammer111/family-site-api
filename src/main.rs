mod api;
mod prelude;
mod setup;
mod error;

use std::io::Result;
use actix_web::{web, App, HttpServer};
use prelude::login;
use prelude::register;
use crate::setup::setup::Setup;
use actix_web::middleware::Logger;
use tokio_postgres::NoTls;


#[actix_web::main]
async fn main() -> Result<()> {
    let mut setup = Setup::default_init();

    setup.init_logger().init_pg();

    let pool = setup.pg.create_pool(None, NoTls).expect("Failed to create a pg pool");

    let server = HttpServer::new(move || {
        let app = App::new();

        let app_with_data = app.app_data(web::Data::new(pool.clone()));

        let app_with_data_and_log = app_with_data.wrap(Logger::new("%a | %t | %r | %s | %Ts"));

        let account = web::scope("/account")
            .service(login)
            .service(register);

        let api = web::scope("/api").service(account);

        app_with_data_and_log.service(api)
    });

    server.bind(setup.server_addr)?.run().await
}
