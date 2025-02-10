use diesel::{prelude::{Associations, Identifiable, Insertable, Queryable}, Selectable};
use serde::Serialize;
use crate::schema::localizacion; 
use crate::schema::ubicacion; 


#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Insertable)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(primary_key(idLocalizacion))]
#[diesel(table_name = localizacion)]
#[allow(non_snake_case)] 
pub struct Localizacion {
    pub idLocalizacion: i32,
    pub nombre: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Insertable)]
#[derive(serde::Deserialize, Serialize)]
#[diesel(primary_key(idUbicacion))]
#[diesel(belongs_to(Localizacion, foreign_key = idLocalizacionFK))]
#[diesel(table_name = ubicacion)]
#[allow(non_snake_case)] 
pub struct Ubicacion {
    idUbicacion: i32,
    nombre: Option<String>,
    idLocalizacionFK: i32,
}