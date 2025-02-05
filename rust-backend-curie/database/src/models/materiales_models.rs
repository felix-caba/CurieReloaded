use crate::{models::productos_models::Producto, schema::material};
use diesel::prelude::*;
use chrono::NaiveDate;
use serde::Serialize;

use super::productos_models::Model;

impl Model for Material {
    type Table = material::table;
    type Model = Material;
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Insertable)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(primary_key(idProducto))]
#[diesel(belongs_to(Producto, foreign_key = idProducto))]
#[diesel(table_name = material)]
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
#[diesel(table_name = material)]
#[allow(non_snake_case)] 
pub struct MaterialForm {
    pub subcategoria: Option<String>,
    pub numero_serie: Option<String>,
    pub descripcion: Option<String>,
    pub fecha_compra: Option<NaiveDate>,
    pub fechaCaducidad: Option<NaiveDate>,
}