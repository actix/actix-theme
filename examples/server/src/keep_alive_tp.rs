// <example>
use actix_web::{HttpRequest, HttpResponse};

async fn index(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .force_close() // <- Close connection
        .finish()
}
// </example>

// ConnectionType::Close
// ConnectionType::KeepAlive
// ConnectionType::Upgrade
