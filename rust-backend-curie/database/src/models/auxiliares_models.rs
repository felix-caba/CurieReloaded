use crate::{models::productos_models::Producto, schema::productos_auxiliares};
use diesel::prelude::*;
use serde:: Serialize;

use super::productos_models::InsertDetail;

impl InsertDetail for ProductoAuxiliarForm {
    type Inserted = ProductoAuxiliar;

    fn insert_detail(self, id_producto: i32, conn: &mut MysqlConnection) -> Result<Self::Inserted, diesel::result::Error> {
       
        
        let nuevo_producto_auxiliar = ProductoAuxiliar {
            idProducto: id_producto,
            formato: self.formato,
        };

        diesel::insert_into(productos_auxiliares::table)
            .values(&nuevo_producto_auxiliar)
            .execute(conn)?;

        Ok(nuevo_producto_auxiliar)
    }
}

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