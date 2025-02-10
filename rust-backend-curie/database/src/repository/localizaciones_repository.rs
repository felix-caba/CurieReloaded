use diesel::prelude::*;

use crate::models::localizaciones_models::*;
use crate::schema::*;
use crate::establish_connection;


pub fn select_locations() -> Result<Vec<Localizacion>, diesel::result::Error> {
    let connection = &mut establish_connection();
    let results = localizacion::table
        .select(Localizacion::as_select())
        .load(connection)?;
    
    Ok(results)
}

pub fn select_location_by_id(id: i32) -> Result<Localizacion, diesel::result::Error> {
    let connection = &mut establish_connection();
    let result = localizacion::table
        .select(Localizacion::as_select())
        .filter(localizacion::idLocalizacion.eq(id))
        .first(connection)?;
    Ok(result)
}

pub fn view_ubications_of_location(id_fk: i32) -> Result<Vec<Ubicacion>, diesel::result::Error> {
    let connection = &mut establish_connection();
    let results = ubicacion::table
        .select(Ubicacion::as_select())
        .filter(ubicacion::idLocalizacionFK.eq(id_fk))
        .load(connection)?;
    Ok(results)
}