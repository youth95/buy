use actix_web::{get, web, Responder, Result};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

static SECRET: &str = "89084E85-B083-437B-9301-EE9701F81B73";

fn timestamp() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() as i64 * 1000i64
        + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    ms
}

#[derive(Serialize)]
struct BuyCode {
    publish: i64, // 发布时间
    expired: i64, // code 的过期时间, 单位: 天
    nonce: i64,   // 随机数
}

#[derive(Serialize)]
struct BuyCodeWithStatus {
    ok: bool,
    publish: Option<i64>, // 发布时间
    expired: Option<i64>, // code 的过期时间, 单位: 天
}

#[derive(Serialize)]
struct Fail {
    pub ok: bool,
}

#[derive(Serialize, Deserialize)]
struct BuyCodeWithSign {
    publish: i64,  // 发布时间
    expired: i64,  // code 的过期时间, 单位: 天
    nonce: i64,    // 随机数
    sign: Vec<u8>, // 签名
}

impl BuyCode {
    pub fn new(day: i64) -> Self {
        Self {
            publish: timestamp(),
            expired: day,
            nonce: rand::random::<i64>(),
        }
    }
}

impl BuyCodeWithSign {
    pub fn check(&self) -> bool {
        let digest = md5::compute(format!(
            "{}{}{}{}",
            self.publish, self.expired, self.nonce, SECRET
        ));
        format!("{:?}", digest.0) == format!("{:?}", self.sign)
    }
}

impl Into<BuyCodeWithSign> for BuyCode {
    fn into(self) -> BuyCodeWithSign {
        let digest = md5::compute(format!(
            "{}{}{}{}",
            self.publish, self.expired, self.nonce, SECRET
        ));
        return BuyCodeWithSign {
            publish: self.publish,
            expired: self.expired,
            nonce: self.nonce,
            sign: digest.0.to_vec(),
        };
    }
}

#[get("/check/{code}")]
async fn check(code: web::Path<String>) -> Result<impl Responder> {
    let code = code.replace("_", "/");
    let code = base64::decode(code).unwrap();
    let result: BuyCodeWithSign = bincode::deserialize(&code).unwrap();
    if result.check() {
        Ok(web::Json(BuyCodeWithStatus {
            expired: Some(result.expired),
            publish: Some(result.publish),
            ok: true,
        }))
    } else {
        Ok(web::Json(BuyCodeWithStatus {
            ok: false,
            publish: None,
            expired: None,
        }))
    }
}

#[get("/buy/{day}")]
async fn buy(day: web::Path<i64>) -> Result<impl Responder> {
    let code: BuyCodeWithSign = BuyCode::new(day.into_inner()).into();
    Ok(base64::encode(bincode::serialize(&code).unwrap()).replace("/", "_"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    HttpServer::new(|| App::new().service(check).service(buy))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
