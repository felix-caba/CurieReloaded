use crate::{models::productos_models::Producto, schema::materiales};
use diesel::prelude::*;
use chrono::NaiveDate;
use serde:: Serialize;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Insertable)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(primary_key(idProducto))]
#[diesel(belongs_to(Producto, foreign_key = idProducto))]
#[diesel(table_name = materiales)]
#[allow(non_snake_case)] 
pub struct Material {
    pub idProducto: i32,
    pub subcategoria: Option<String>,
    pub numero_serie: Option<String>,
    pub descripcion: Option<String>,
    pub fecha_compra: Option<NaiveDate>,
    pub fechaCaducidad: Option<NaiveDate>,
}

#[derive(Selectable, PartialEq, Insertable)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(table_name = materiales)]
#[allow(non_snake_case)] 
pub struct MaterialForm {
    pub subcategoria: Option<String>,
    pub numero_serie: Option<String>,
    pub descripcion: Option<String>,
    pub fecha_compra: Option<NaiveDate>,
    pub fechaCaducidad: Option<NaiveDate>,
}