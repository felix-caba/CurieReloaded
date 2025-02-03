use diesel::prelude::*;
use rocket::http::Status;
use serde::Deserialize;
use serde::Serialize;   


use crate::schema::productos::idProducto;
use crate::schema::{productos, reactivos};

/**
 * Flatten para omitir el ID duplicado
 */
#[derive(Serialize, Deserialize)]
pub struct ProductoReactivo {
    #[serde(flatten)]
    pub producto: Producto,
    #[serde(flatten)]
    pub reactivo: Reactivo,
}

use crate::establish_connection;

pub fn get_reactivos() -> Result<Vec<ProductoReactivo>, Status> {

    let connection = &mut establish_connection();
    
    let todos_los_reactivos_y_productos = reactivos::table
    .inner_join(productos::table)
    .select((Reactivo::as_select(), Producto::as_select()))
    .load::<(Reactivo, Producto)>(connection)
    .map_err(|_| Status::InternalServerError)?;
  
    let productos_reactivo = todos_los_reactivos_y_productos.into_iter()
    .map(|(reactivo, producto)| ProductoReactivo { producto, reactivo }).collect();

    Ok(productos_reactivo)
}

pub fn insert_producto_reactivo(producto_reactivo: ProductoReactivo) -> Result<ProductoReactivo, Status> {
    let connection = &mut establish_connection();

    let producto = producto_reactivo.producto;
    let reactivo = producto_reactivo.reactivo;

    connection.transaction(|connection| {

        diesel::insert_into(productos::table)
        .values(&producto)
        .execute(connection);

        let id= productos::table
        .order(idProducto.desc())
        .select(Producto::as_select())
        .first(connection)
        .optional();

        diesel::insert_into(reactivos::table)
        .values(&reactivo)
        .execute(connection);

        

  

       
    }).expect("Xd");

   Ok(producto_reactivo)
}