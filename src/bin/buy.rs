use actix_cors::Cors;
use actix_web::{get, http, middleware::Logger, web, App, HttpServer, Responder, Result};
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
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:7456")
            // .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(r_check)
            .service(r_buy)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
