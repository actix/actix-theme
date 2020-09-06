// <compress>
use actix_web::{get, middleware, App, HttpResponse, HttpServer};

#[get("/")]
async fn index_br() -> HttpResponse {
    HttpResponse::Ok().body("data")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(index_br)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
// </compress>
