// <main>
use actix_cors::Cors;
use actix_http::{HttpService, KeepAlive};
use actix_service::map_config;
use actix_web::{
    dev::{AppConfig, Server},
    web, App, Responder,
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

async fn h1() -> impl Responder {
    "H1"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Started H1 server: 0.0.0.0:8080");

    Server::build()
        .backlog(1024)
        .bind("example", "0.0.0.0:8080", || {
            let cors = Cors::default()
                .supports_credentials()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header();
            let mut builder =
                SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            builder
                .set_private_key_file("key.pem", SslFiletype::PEM)
                .unwrap();
            builder.set_certificate_chain_file("cert.pem").unwrap();
            let acceptor = builder.build();

            HttpService::build()
                .keep_alive(KeepAlive::Os)
                .client_timeout(0)
                .h1(map_config(
                    App::new().wrap(cors).service(web::resource("/").to(h1)),
                    |_| AppConfig::default(),
                ))
                .openssl(acceptor)
        })?
        .start()
        .await
}
// </main>
