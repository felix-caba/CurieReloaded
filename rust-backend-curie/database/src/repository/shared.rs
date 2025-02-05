use diesel::{Connection, ExpressionMethods, MysqlConnection, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{establish_connection, models::productos_models::{Producto, ProductoForm}, schema::producto};

pub fn delete_producto(id: i32) -> Result<(), diesel::result::Error> {
    let mut connection = establish_connection();

    connection.transaction(|connection| {

        let producto_to_delete: Option<Producto> = producto::table
            .filter(producto::idProducto.eq(id))
            .select(Producto::as_select())
            .first(connection)
            .optional()?;

        if producto_to_delete.is_none() {
            return Err(diesel::result::Error::NotFound);
        }
        

        diesel::delete(producto::table)
            .filter(producto::idProducto.eq(id))
            .execute(connection)?;
    

        Ok(())

    })

}

pub fn update_producto(connection: &mut MysqlConnection, id: i32, producto: &ProductoForm) -> Result<Producto, diesel::result::Error> {
    diesel::update(producto::table)
        .set(producto)
        .filter(producto::idProducto.eq(id))
        .execute(connection)?;

    producto::table
        .filter(producto::idProducto.eq(id))
        .select(Producto::as_select())
        .first(connection)
}