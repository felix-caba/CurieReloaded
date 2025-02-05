use crate::schema::producto;
use diesel::{expression::NonAggregate, prelude::*, sql_types::{Integer, SqlType}};
use serde::{Deserialize, Serialize};

pub trait Model {
    type Table;
    type Model;

}

#[derive(Serialize, Deserialize)]
pub struct ProductoModel<Q> {
    #[serde(flatten)]
    pub producto_base: Producto,
    #[serde(flatten)]
    pub details: Q,
}

#[derive(Serialize, Deserialize)]
pub struct ProductoModelForm<Q> {
    #[serde(flatten)]
    pub producto_base: ProductoForm,
    #[serde(flatten)]
    pub details: Q,
}


#[derive(
    Queryable,
    Identifiable,
    Selectable,
    Debug,
    PartialEq,
    Clone,
    Insertable,
    serde::Deserialize,
    Serialize,
)]
#[diesel(primary_key(idProducto))]
#[diesel(table_name = producto)]
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

#[derive(PartialEq, Insertable, AsChangeset, serde::Deserialize, Serialize)]
#[diesel(table_name = producto)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[allow(non_snake_case)]
pub struct ProductoForm {
    pub idLocalizacion: i32,
    pub idUbicacion: i32,
    pub nombre: Option<String>,
    pub cantidad: Option<i32>,
    pub stock_minimo: Option<i32>,
}
