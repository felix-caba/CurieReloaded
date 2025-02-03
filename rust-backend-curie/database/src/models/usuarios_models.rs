use diesel::prelude::*;
use crate::schema::usuarios;
use serde::Serialize;


#[derive(Queryable, Selectable, Insertable)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(table_name = usuarios)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[allow(non_snake_case)] 
pub struct Usuario {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub username: String,
    pub admin: Option<i32>,
    pub pfp: Option<Vec<u8>>,
}

#[derive(Queryable, Selectable, Insertable)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(table_name = usuarios)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[allow(non_snake_case)] 
pub struct NewUsuario {
    pub email: String,
    pub password: String,
    pub username: String,
}