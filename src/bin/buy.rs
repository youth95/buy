use actix_web::{get, web, Responder, Result};
use buy::BuyCodeWithSign;

#[get("/check/{code}")]
async fn r_check(code: web::Path<String>) -> Result<impl Responder> {
    Ok(web::Json(BuyCodeWithSign::check_code_str(&code)))
}

#[get("/buy/{day}")]
async fn r_buy(day: web::Path<i64>) -> Result<impl Responder> {
    Ok(BuyCodeWithSign::make_code_str(day.into_inner()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    HttpServer::new(|| App::new().service(r_check).service(r_buy))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
