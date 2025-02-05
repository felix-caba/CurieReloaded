

use database::repository::user_repository;
use rocket::http::Status;
use crate::jwt_service;
use crate::bcrypt_encoder;
use database::models::usuarios_models::UsuarioForm;


use database::repository::user_repository::{AuthRequest, AuthResponse};






/**
 * 
 * Autentico el usuario (login)
 * Espera un AuthRequest o devuelve un error 
 * 
 */


pub fn authenticate_user(auth_request: AuthRequest) -> Result<AuthResponse, Status> {

    let user = match user_repository::get_user_by_username(&auth_request.username) {
            Some(user) => user,
            None => return Err(Status::NotFound)
        };

    if !bcrypt_encoder::verify_password(&auth_request.password, &user.password) {
        return Err(Status::Unauthorized)
    }

    let token = jwt_service::generate_token(user.id.to_string());

    Ok(AuthResponse { token: Some(token) })
}

/**
 * 
 * Register a new user, hashing the password, and returning the token 
 * in a AuthResponse struct
 * 
 */

 

pub fn register_user(mut user: UsuarioForm) -> Result<AuthResponse, Status> {

    // First I check for dupes

    if user_repository::check_duplicate_user(&user) {
        return Err(Status::Conflict);
    }

    let hashed_password = bcrypt_encoder::hash_password(&user.password);
    user.password = hashed_password;

    /*
     * Uso un option donde me devolverá el Some(user) o None
     * Si es None, devuelvo un error
     * Si es Some(user), devuelvo el usuario
     */

     let user = match user_repository::create_user(&user) {
        Ok(user) => user,
        Err(_) => return Err(Status::InternalServerError),
    };

    let token = jwt_service::generate_token(user.id.to_string());
    Ok(AuthResponse { token: Some(token) })


}
