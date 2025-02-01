
/*

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
*/

mod models;
mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use rust_backend_curie::establish_connection;
use std::env;

fn main() {
    use self::schema::productos::dsl::*;  
    
    let connection = &mut establish_connection();
    let results = productos
        .limit(5)
        .select(models::Producto::as_select())
        .load(connection)
        .expect("Error loading products");

    println!("Displaying {} products", results.len());
    for product in results {
        println!("{}", product.nombre.unwrap_or_default());
    }
}

