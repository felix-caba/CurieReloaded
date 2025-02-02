use serde::{Deserialize, Serialize};
use database::repository::user_repository;
use database::models::Usuario;
use crate::jwt_service;
use crate::bcrypt_encoder;


#[derive(Deserialize, Serialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}



pub fn authenticate_user(auth_request: AuthRequest) -> Result<AuthResponse, String> {

    let user = user_repository::get_user_by_username(&auth_request.username);

    if user.is_none() {
        return Err("Usuario no encontrado".to_string());
    }

    let user = user.unwrap();

    if !bcrypt_encoder::verify_password(&auth_request.password, &user.password.unwrap()) {
        return Err("Contraseña incorrecta".to_string());
    }

    let token = jwt_service::generate_token(user.id.to_string());

    Ok(AuthResponse { token })

}

pub fn register_user(mut user: Usuario) -> Result<AuthResponse, String> {

    let hashed_password = bcrypt_encoder::hash_password(&user.password.unwrap());
    user.password = Some(hashed_password);
    
    let user = user_repository::create_user(&user);

    if user.is_none() {
        return Err("Error al crear el usuario".to_string());
    } else {
        let token = jwt_service::generate_token(user.unwrap().id.to_string());
        return Ok(AuthResponse { token });
    }

}
