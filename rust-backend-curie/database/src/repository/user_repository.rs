use crate::schema::usuarios::dsl::*;
use crate::models::Usuario;
use crate::establish_connection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::NewUsuario;


#[derive(Deserialize, Serialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}


pub fn get_user_by_email(email_to_search: &str) -> Option<Usuario> {

    let mut connection = establish_connection();

    let user = usuarios
        .filter(email.eq(email_to_search))
        .first::<Usuario>(&mut connection)
        .optional()
        .expect("Error loading user");

    user
    
}

pub fn get_user_by_username(username_to_search: &str) -> Option<Usuario> {
    let mut connection = establish_connection();

    let user = usuarios
        .filter(username.eq(username_to_search))
        .first::<Usuario>(&mut connection)
        .optional()
        .expect("Error loading user");

    user
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

pub fn create_user(new_user: &NewUsuario) -> Option<Usuario> {
    let mut connection = establish_connection();

    connection.transaction(|connection  | {

        diesel::insert_into(usuarios)
            .values(new_user)
            .execute(connection)
            .expect("Error creating user");
        
        usuarios
        .order(id.desc())
        .select(Usuario::as_select())
        .first(connection)
        .optional()

    }).expect("Error creating user")

   


}
