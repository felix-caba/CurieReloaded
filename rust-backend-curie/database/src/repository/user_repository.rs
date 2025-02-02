use crate::schema::usuarios::dsl::*;
use crate::models::Usuario;
use crate::establish_connection;
use diesel::prelude::*;


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

pub fn create_user(user: &Usuario) -> Option<Usuario> {
    let mut connection = establish_connection();

    
    
    
    diesel::insert_into(usuarios)
        .values(user)
        .execute(&mut connection)
        .expect("Error creating user");
     
    usuarios
        .filter(email.eq(&user.email))
        .first(&mut connection)
        .optional()
        .expect("Error loading user")

}