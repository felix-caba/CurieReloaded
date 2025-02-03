use diesel::prelude::*;
use rocket::http::Status;  

use crate::models::productos_models::Producto;
use crate::models::productos_models::ProductoWithDetails;
use crate::models::reactivos_models::Reactivo;
use crate::schema::productos::idProducto;
use crate::schema::{productos, reactivos};


use crate::establish_connection;

pub fn get_reactivos() -> Result<Vec<ProductoWithDetails<Reactivo>>, Status> {
    let connection = &mut establish_connection();
    
    let resultados = reactivos::table
        .inner_join(productos::table)
        .select((Reactivo::as_select(), Producto::as_select()))
        .load::<(Reactivo, Producto)>(connection)
        .map_err(|_| Status::InternalServerError)?;
  
    let productos_reactivo = resultados.into_iter()
        .map(|(detalle, producto)| ProductoWithDetails {
            producto,
            details: detalle
        })
        .collect();

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