use crate::models::usuarios_models::UsuarioForm;
use crate::{models::usuarios_models::Usuario, schema::usuarios::dsl::*};
use crate::establish_connection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]


pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: Option<String>,
    pub user: Option<Usuario>,
}



#[derive(Deserialize, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

pub fn select_user_by_id(id_to_check: i32) -> Option<Usuario> {
    let mut connection = establish_connection();

    let user = usuarios
        .filter(id.eq(id_to_check))
        .first::<Usuario>(&mut connection)
        .optional()
        .expect("Error loading user");
    user
}

pub fn select_user_by_email(email_to_search: &str) -> Option<Usuario> {
    let mut connection = establish_connection();

    let user = usuarios
        .filter(email.eq(email_to_search))
        .first::<Usuario>(&mut connection)
        .optional()
        .expect("Error loading user");

    user
}

pub fn select_user_by_username(username_to_search: &str) -> Option<Usuario> {
    let mut connection = establish_connection();
    let user = usuarios
        .filter(username.eq(username_to_search))
        .first::<Usuario>(&mut connection)
        .optional()
        .expect("Error loading user");

    user
}

pub fn username_exists(username_to_check: &str) -> bool {
    select_user_by_username(username_to_check).is_some()
}

pub fn email_exists(email_to_check: &str) -> bool {
    select_user_by_email(email_to_check).is_some()
}

pub fn check_duplicate_user(new_user: &UsuarioForm) -> bool {
    username_exists(&new_user.username) || email_exists(&new_user.email)
}



/**
 * 
 * 
 *  Creates a new user using the RegisterRequest struct
 *  It just provides the email, username and password
 *  The id is not provided because it is auto-incremented by the database
 * 
 *  MySQL Does not support returning clauses, so for getting the new inserted
 *  ID we need to do a transaction
 * 
 */



pub fn create_user(new_user: &UsuarioForm) -> Result<Usuario, diesel::result::Error> {
    let mut connection = establish_connection();
    
    connection.transaction(|connection| {
        diesel::insert_into(usuarios)
            .values(new_user)
            .execute(connection)?;

        usuarios
            .order(id.desc())
            .select(Usuario::as_select())
            .first(connection)
    })
}
