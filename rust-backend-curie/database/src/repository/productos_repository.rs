use diesel::prelude::*;


use crate::models::productos_models::InsertDetail;
use crate::models::productos_models::Producto;
use crate::models::productos_models::ProductoForm;
use crate::models::productos_models::ProductoFormWithDetails;
use crate::models::productos_models::ProductoWithDetails;

use crate::models::reactivos_models::Reactivo;
use crate::schema::{productos, reactivos};



use crate::establish_connection;

pub fn create_producto_with_details<F, D>(
    form: ProductoFormWithDetails<F>,
) -> Result<ProductoWithDetails<D>, diesel::result::Error>
where
    F: InsertDetail<Inserted = D>,
{
    let mut conn = establish_connection();
    conn.transaction(|conn| {
   
        diesel::insert_into(productos::table)
            .values(&form.producto)
            .execute(conn)?;

    
        let last_id = productos::table
            .order(productos::idProducto.desc())
            .select(productos::idProducto)
            .first(conn)?;  


        let producto = productos::table
            .select(Producto::as_select())
            .filter(productos::idProducto.eq(last_id))
            .first::<Producto>(conn)?;

        let details = form.details.insert_detail(last_id, conn)?;

        Ok(ProductoWithDetails {
            producto,
            details,
        })
    })
    
}


fn update_producto(connection: &mut MysqlConnection, id: i32, producto: &ProductoForm) -> Result<Producto, diesel::result::Error> {
    diesel::update(productos::table)
        .set(producto)
        .filter(productos::idProducto.eq(id))
        .execute(connection)?;

    productos::table
        .filter(productos::idProducto.eq(id))
        .select(Producto::as_select())
        .first(connection)
}

pub fn select_reactivos() -> Result<Vec<ProductoWithDetails<Reactivo>>, diesel::result::Error> {
    let connection = &mut establish_connection();

    let resultados = reactivos::table
        .inner_join(productos::table)
        .select((Reactivo::as_select(), Producto::as_select()))
        .load::<(Reactivo, Producto)>(connection)?;

    let productos_reactivo = resultados
        .into_iter()
        .map(|(detalle, producto)| ProductoWithDetails {
            producto,
            details: detalle,
        })
        .collect();

    Ok(productos_reactivo)
}
/* 
pub fn insert_reactivo(producto_form: ProductoFormWithDetails<ReactivoForm>) -> Result<ProductoWithDetails<Reactivo>, diesel::result::Error> {
    let mut connection = establish_connection();

    connection.transaction(|connection| {
        
        diesel::insert_into(productos::table)
            .values(&producto_form.producto)
            .execute(connection)?;

       
        let producto_insertado = productos::table
            .order(productos::idProducto.desc())
            .select(Producto::as_select())
            .first(connection)?;

    
        let reactivo_a_insertar = Reactivo {
            idProducto: producto_insertado.idProducto,
            formato: producto_form.details.formato,
            gradoPureza: producto_form.details.gradoPureza,
            fechaCaducidad: producto_form.details.fechaCaducidad,
        };

    
        diesel::insert_into(reactivos::table)
            .values(&reactivo_a_insertar)
            .execute(connection)?;

    
        let reactivo_insertado = reactivos::table
            .order(reactivos::idProducto.desc())
            .select(Reactivo::as_select())
            .first(connection)?;

       
        Ok(ProductoWithDetails {
            producto: producto_insertado,
            details: reactivo_insertado,
        })
    })
}

*/

/* 
pub fn update_reactivo(id: i32, producto_form: ProductoFormWithDetails<ReactivoForm>) -> Result<ProductoWithDetails<Reactivo>, diesel::result::Error> {

    let mut connection = establish_connection();

    connection.transaction(|connection| {

        let producto_actualizado = update_producto(connection, id, &producto_form.producto)?;

        diesel::update(reactivos::table)
            .set(&producto_form.details)
            .filter(reactivos::idProducto.eq(id))
            .execute(connection)?;

        let reactivo_actualizado = reactivos::table
            .filter(reactivos::idProducto.eq(id))
            .select(Reactivo::as_select())
            .first(connection)?;

        Ok(ProductoWithDetails {
            producto: producto_actualizado,
            details: reactivo_actualizado,
        })

    })

    


    
}
*/
pub fn delete_producto(id: i32) -> Result<(), diesel::result::Error> {
    let mut connection = establish_connection();

    connection.transaction(|connection| {

        let producto_to_delete: Option<Producto> = productos::table
            .filter(productos::idProducto.eq(id))
            .select(Producto::as_select())
            .first(connection)
            .optional()?;

        if producto_to_delete.is_none() {
            return Err(diesel::result::Error::NotFound);
        }
        

        diesel::delete(productos::table)
            .filter(productos::idProducto.eq(id))
            .execute(connection)?;
    

        Ok(())

    })

}
