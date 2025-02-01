use diesel::prelude::*;

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

