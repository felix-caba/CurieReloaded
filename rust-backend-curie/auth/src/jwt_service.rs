use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
#[derive(Debug, Serialize, Deserialize)]

pub struct Claims {
    sub: String,  // subject (user id)
    exp: usize,   // expiration time
    iat: usize,   // issued at
    admin: bool,
}

pub struct JWT {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = Status;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Status> {
        
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_token(key)?)
        }

        let token = req.headers().get_one("Authorization");

        match token {
            None => Outcome::Error((Status::Unauthorized, Status::Unauthorized)),
            Some(token) => {
                let claims = is_valid(&token);
                match claims {
                    Ok(claims) => Outcome::Success(JWT { claims }),
                    Err(err) => match &err.kind() {
                        ErrorKind::InvalidToken => Outcome::Error((Status::Unauthorized, Status::Unauthorized)),
                        ErrorKind::ExpiredSignature => Outcome::Error((Status::Unauthorized, Status::from_code(419).unwrap())),
                        ErrorKind::InvalidSignature => Outcome::Error((Status::Unauthorized, Status::Unauthorized)),
                        _ => {
                            eprintln!("Error: {}", err);
                            Outcome::Error((Status::InternalServerError, Status::InternalServerError))
                        }
                    }
                }
            }
        }
    }
}

pub fn decode_token(token: &str) -> Result<Claims, Error> {
    let key = env::var("JWT_SECRET").unwrap();
    let key = DecodingKey::from_secret(key.as_bytes());
    let token_data = decode::<Claims>(&token, &key, &Validation::default())?;
    Ok(token_data.claims)
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
