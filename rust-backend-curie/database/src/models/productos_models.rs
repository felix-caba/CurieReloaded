use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::productos;



#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Clone, Insertable)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(primary_key(idProducto))]
#[diesel(table_name = productos)]
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

#[derive(PartialEq, Insertable, AsChangeset)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(table_name = productos)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[allow(non_snake_case)] 
pub struct ProductoForm {
    pub idLocalizacion: i32,
    pub idUbicacion: i32,
    pub nombre: Option<String>,
    pub cantidad: Option<i32>,
    pub stock_minimo: Option<i32>,
}


#[derive(Serialize, Deserialize)]
pub struct ProductoWithDetails<T> {
    #[serde(flatten)]
    pub producto: Producto,
    #[serde(flatten)]
    pub details: T,
}

// Estructura genérica para formularios
#[derive(Serialize, Deserialize)]
pub struct ProductoFormWithDetails<T> {
    #[serde(flatten)]
    pub producto: ProductoForm,
    #[serde(flatten)]
    pub details: T,
}



