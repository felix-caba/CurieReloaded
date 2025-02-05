use crate::{models::productos_models::Producto, schema::auxiliar};
use diesel::prelude::*;
use serde:: Serialize;

use super::productos_models::Model;

impl Model for Auxiliar {
    type Table = auxiliar::table;
    type Model = Auxiliar;
}



#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Insertable)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(primary_key(idProducto))]
#[diesel(belongs_to(Producto, foreign_key = idProducto))]
#[diesel(table_name = auxiliar)]
#[allow(non_snake_case)] 
pub struct Auxiliar {
    pub idProducto: i32,
    pub formato: Option<String>,
}

#[derive(Selectable, PartialEq, Insertable)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(table_name = auxiliar)]
#[allow(non_snake_case)] 
pub struct AuxiliarForm {
    pub formato: Option<String>,
}