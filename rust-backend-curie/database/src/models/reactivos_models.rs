use crate::{models::productos_models::Producto, schema::reactivo};
use diesel::prelude::*;
use chrono::NaiveDate;
use serde:: Serialize;

use super::productos_models::Model;

impl Model for Reactivo {
    type Table = reactivo::table;
    type Model = Reactivo;
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Insertable)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(primary_key(idProducto))]
#[diesel(belongs_to(Producto, foreign_key = idProducto))]
#[diesel(table_name = reactivo)]
#[allow(non_snake_case)] 
pub struct Reactivo {
    pub idProducto: i32,
    pub formato: Option<String>,
    pub gradoPureza: Option<String>,
    pub fechaCaducidad: Option<NaiveDate>, 
}

#[derive(Selectable, PartialEq, Insertable, AsChangeset)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(table_name = reactivo)]
#[allow(non_snake_case)] 
pub struct ReactivoForm {
    pub formato: Option<String>,
    pub gradoPureza: Option<String>,
    pub fechaCaducidad: Option<NaiveDate>, 
}