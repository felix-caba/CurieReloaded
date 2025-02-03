use crate::{models::productos_models::Producto, schema::productos_auxiliares};
use diesel::prelude::*;
use serde:: Serialize;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Insertable)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(primary_key(idProducto))]
#[diesel(belongs_to(Producto, foreign_key = idProducto))]
#[diesel(table_name = productos_auxiliares)]
#[allow(non_snake_case)] 
pub struct ProductoAuxiliar {
    pub idProducto: i32,
    pub formato: Option<String>,
}

#[derive(Selectable, PartialEq, Insertable)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(table_name = productos_auxiliares)]
#[allow(non_snake_case)] 
pub struct ProductoAuxiliarForm {
    pub formato: Option<String>,
}