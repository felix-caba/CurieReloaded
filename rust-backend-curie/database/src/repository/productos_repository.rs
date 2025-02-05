use diesel::prelude::*;

use crate::models::productos_models::*;
use crate::models::reactivos_models::*;
use crate::schema::*;
use crate::establish_connection;

use super::shared::update_producto;

pub fn select_reactivos() -> Result<Vec<ProductoModel<Reactivo>>, diesel::result::Error> {
    let connection = &mut establish_connection();
    let results = producto::table
        .inner_join(reactivo::table.on(reactivo::idProducto.eq(producto::idProducto)))
        .select((Producto::as_select(), Reactivo::as_select()))
        .load::<(Producto, Reactivo)>(connection)?;

    let producto_models = results
        .into_iter()
        .map(|(producto_base, details)| ProductoModel {
            producto_base,
            details,
        })
        .collect();
    Ok(producto_models)
}

pub fn insert_reactivo(
    reactivo_model_form: ProductoModelForm<ReactivoForm>,
) -> Result<ProductoModel<Reactivo>, diesel::result::Error> {
    let connection = &mut establish_connection();

    connection.transaction(|connection| {
        diesel::insert_into(producto::table)
            .values(&reactivo_model_form.producto_base)
            .execute(connection)?;

        let last_id_i64: i64 = diesel::select(last_insert_id()).get_result(connection)?;

        let producto_inserted: Producto = producto::table
            .filter(producto::idProducto.eq(last_id_i64 as i32))
            .select(Producto::as_select())
            .first::<Producto>(connection)?;

        let reactivo_with_id = Reactivo {
            idProducto: last_id_i64 as i32,
            formato: reactivo_model_form.details.formato,
            gradoPureza: reactivo_model_form.details.gradoPureza,
            fechaCaducidad: reactivo_model_form.details.fechaCaducidad,
        };

        diesel::insert_into(reactivo::table)
            .values(&reactivo_with_id)
            .execute(connection)?;

        let reactivo_inserted = reactivo::table
            .filter(reactivo::idProducto.eq(last_id_i64 as i32))
            .first::<Reactivo>(connection)?;

        let producto_model = ProductoModel {
            producto_base: producto_inserted,
            details: reactivo_inserted,
        };

        Ok(producto_model)
    })
}

pub fn update_reactivo(
    reactivo_model_form: ProductoModelForm<ReactivoForm>,
    id: i32,
) -> Result<ProductoModel<Reactivo>, diesel::result::Error> {
    let connection = &mut establish_connection();

    connection.transaction(|connection| {
        let producto_actualizado: Producto =
            update_producto(connection, id, &reactivo_model_form.producto_base)?;

        diesel::update(reactivo::table)
            .set(&reactivo_model_form.details)
            .filter(reactivo::idProducto.eq(id))
            .execute(connection)?;

        let reactivo_actualizado = reactivo::table
            .filter(reactivo::idProducto.eq(id))
            .select(Reactivo::as_select())
            .first(connection)?;

        Ok(ProductoModel {
            producto_base: producto_actualizado,
            details: reactivo_actualizado,
        })
    })
}
