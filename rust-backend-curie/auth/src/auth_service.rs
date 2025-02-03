use database::models::NewUsuario;
use database::repository::user_repository;
use crate::jwt_service;
use crate::bcrypt_encoder;

use database::repository::user_repository::{AuthRequest, AuthResponse};



/**
 * 
 * Autentico el usuario (login)
 * Espera un AuthRequest o devuelve un error 
 * 
 */


pub fn authenticate_user(auth_request: AuthRequest) -> Result<AuthResponse, String> {

    let user = match user_repository::get_user_by_username(&auth_request.username) {
        Some(user) => user,
        None => return Err("Usuario no encontrado".to_string()),
    };

    if !bcrypt_encoder::verify_password(&auth_request.password, &user.password) {
        return Err("Contraseña incorrecta".to_string());
    }

    let token = jwt_service::generate_token(user.id.to_string());

    Ok(AuthResponse { token })

}

/**
 * 
 * Register a new user, hashing the password, and returning the token 
 * in a AuthResponse struct
 * 
 */

pub fn register_user(mut user: NewUsuario) -> Result<AuthResponse, String> {


    let hashed_password = bcrypt_encoder::hash_password(&user.password);

    user.password = hashed_password;


    let user = user_repository::create_user(&user);

    if user.is_none() {
        return Err("Error al crear el usuario".to_string());
    }

    let token = jwt_service::generate_token(user.unwrap().id.to_string());

    Ok(AuthResponse { token })
     
     

}
