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
pub struct BuyCode {
    publish: i64, // 发布时间
    expired: i64, // code 的过期时间, 单位: 天
    nonce: i64,   // 随机数
}

#[derive(Serialize)]
pub struct BuyCodeWithStatus {
    pub ok: bool,
    pub publish: Option<i64>,        // 发布时间
    pub expired: Option<i64>,        // code 的过期时间, 单位: 天
    pub balance: Option<i64>,        // 结余时间, 单位 ms
    pub balance_str: Option<String>, // 结余时间 文本表示
}

#[derive(Serialize, Deserialize)]
pub struct BuyCodeWithSign {
    publish: i64,  // 发布时间
    expired: i64,  // code 的过期时间, 单位: 天
    nonce: i64,    // 随机数
    sign: Vec<u8>, // 签名
}

impl BuyCodeWithSign {
    pub fn check_code_str(code: &String) -> BuyCodeWithStatus {
        let code = code.replace("_", "/");
        if let Ok(code) = base64::decode(code) {
            if let Ok(result) = bincode::deserialize::<BuyCodeWithSign>(&code) {
                if result.check() {
                    return BuyCodeWithStatus {
                        expired: Some(result.expired),
                        publish: Some(result.publish),
                        balance: Some(result.balance()),
                        balance_str: Some(result.balance_str()),
                        ok: true,
                    };
                }
            }
        }
        BuyCodeWithStatus {
            ok: false,
            publish: None,
            expired: None,
            balance: None,
            balance_str: None,
        }
    }

    pub fn make_code_str(day: i64) -> String {
        let code: BuyCodeWithSign = BuyCode::new(day).into();
        base64::encode(bincode::serialize(&code).unwrap()).replace("/", "_")
    }
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
            && self.expired * 24 * 60 * 60 * 1000 + self.publish >= timestamp()
    }

    pub fn balance(&self) -> i64 {
        self.expired * 24 * 60 * 60 * 1000 + self.publish - timestamp()
    }

    pub fn balance_str(&self) -> String {
        format!("{:.2}H", self.balance() as f32 / 1000. / 60. / 60.)
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

#[cfg(test)]
mod test {
    use crate::{BuyCode, BuyCodeWithSign};

    #[test]
    fn test_timestamp() {
        assert_eq!(super::timestamp() > 0, true);
    }

    #[test]
    fn test_create_buy_code() {
        let code: BuyCodeWithSign = BuyCode::new(1).into();
        assert_eq!(code.check(), true);
        assert_eq!(
            BuyCodeWithSign::check_code_str(&BuyCodeWithSign::make_code_str(1)).expired,
            Some(1)
        );
        assert_eq!(
            BuyCodeWithSign::check_code_str(&"".to_string()).expired,
            None
        );
    }
}
