use diesel::prelude::*;

use crate::models::productos_models::Producto;
use crate::models::productos_models::ProductoForm;
use crate::models::productos_models::ProductoFormWithDetails;
use crate::models::productos_models::ProductoWithDetails;
use crate::models::reactivos_models::Reactivo;
use crate::models::reactivos_models::ReactivoForm;
use crate::schema::{productos, reactivos};

use crate::establish_connection;

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


