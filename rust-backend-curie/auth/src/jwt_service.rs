use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // subject (user id)
    exp: usize,   // expiration time
    iat: usize,   // issued at
    admin: bool,
}

pub fn generate_token(user_id: String) -> String {

    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize + 24 * 3600;


    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        sub: user_id,
        exp,
        iat: now,
        admin: false,
    };

    let key = env::var("JWT_SECRET").unwrap();

    let key = EncodingKey::from_secret(key.as_bytes());

    encode(&Header::default(), &claims, &key).unwrap()

}
