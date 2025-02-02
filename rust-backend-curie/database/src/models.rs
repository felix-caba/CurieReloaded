use diesel::prelude::*;
use crate::schema::usuarios;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::productos)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[allow(non_snake_case)] 
pub struct Producto {
    pub idProducto: i32,
    pub idLocalizacion: i32,
    pub idUbicacion: i32,
    pub nombre: Option<String>,
    pub cantidad: Option<i32>,
    pub stock_minimo: Option<i32>,
}

#[derive(Queryable, Selectable, Insertable)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(table_name = usuarios)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[allow(non_snake_case)] 
pub struct Usuario {
    pub id: i32,
    pub email: Option<String>,
    pub password: Option<String>,
    pub username: Option<String>,
    pub admin: Option<i32>,
    pub pfp: Option<Vec<u8>>,
}

#[derive(Queryable, Selectable, Insertable)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(table_name = usuarios)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[allow(non_snake_case)] 
pub struct NewUsuario {
    pub email: Option<String>,
    pub password: Option<String>,
    pub username: Option<String>,
}

