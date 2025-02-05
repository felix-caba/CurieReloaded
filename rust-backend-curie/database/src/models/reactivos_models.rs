use crate::{models::productos_models::Producto, schema::reactivos};
use diesel::prelude::*;
use chrono::NaiveDate;
use serde:: Serialize;
use crate::models::productos_models::InsertDetail;

impl InsertDetail for ReactivoForm {
    type Inserted = Reactivo;

    fn insert_detail(self, id_producto: i32, conn: &mut MysqlConnection) -> Result<Self::Inserted, diesel::result::Error> {
       
        
        let nuevo_reactivo = Reactivo {
            idProducto: id_producto,
            formato: self.formato,
            gradoPureza: self.gradoPureza,
            fechaCaducidad: self.fechaCaducidad,
        };

        diesel::insert_into(reactivos::table)
            .values(&nuevo_reactivo)
            .execute(conn)?;

        Ok(nuevo_reactivo)
    }
}


#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Insertable)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(primary_key(idProducto))]
#[diesel(belongs_to(Producto, foreign_key = idProducto))]
#[diesel(table_name = reactivos)]
#[allow(non_snake_case)] 
pub struct Reactivo {
    pub idProducto: i32,
    pub formato: Option<String>,
    pub gradoPureza: Option<String>,
    pub fechaCaducidad: Option<NaiveDate>, 
}

#[derive(Selectable, PartialEq, Insertable, AsChangeset)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(table_name = reactivos)]
#[allow(non_snake_case)] 
pub struct ReactivoForm {
    pub formato: Option<String>,
    pub gradoPureza: Option<String>,
    pub fechaCaducidad: Option<NaiveDate>, 
}