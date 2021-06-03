// <authorization>
use actix_casbin_auth::casbin::{DefaultModel, FileAdapter, Result};
use actix_casbin_auth::CasbinService;
use actix_web::{web, App, HttpResponse, HttpServer};

#[allow(dead_code)]
mod fake_auth;

#[actix_rt::main]
async fn main() -> Result<()> {
    let m = DefaultModel::from_file("examples/rbac_with_pattern_model.conf")
        .await
        .unwrap();
    let a = FileAdapter::new("examples/rbac_with_pattern_policy.csv");  //You can also use diesel-adapter or sqlx-adapter

    let casbin_middleware = CasbinService::new(m, a).await?;

    casbin_middleware
        .write()
        .await
        .get_role_manager()
        .write()
        .unwrap()
        .matching_fn(Some(key_match2), None);

    HttpServer::new(move || {
        App::new()
            .wrap(casbin_middleware.clone())
            .wrap(FakeAuth)
            .route("/pen/1", web::get().to(|| HttpResponse::Ok()))
            .route("/book/{id}", web::get().to(|| HttpResponse::Ok()))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}
// </authorization>
