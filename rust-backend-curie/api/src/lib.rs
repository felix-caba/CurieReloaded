#[macro_use] extern crate rocket;


mod routes;



use routes::create_reactivo;
use routes::delete_producto;
pub use routes::login;
pub use routes::register;
pub use routes::get_reactivos;

#[rocket::main]
pub async fn launch_api() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/auth", routes![login, register])
        .mount("/reactivos", routes![get_reactivos, create_reactivo])
        .mount("/productos", routes![delete_producto])
        .launch()

        .await?;

    Ok(())
}




