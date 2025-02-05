#[macro_use]
extern crate rocket;

mod usuarios_routes;
mod reactivos_routes;
mod shared_routes;
use reactivos_routes::create_reactivo;
use reactivos_routes::get_reactivos;
use reactivos_routes::update_reactivo;
use shared_routes::delete_producto;
use usuarios_routes::login;
use usuarios_routes::register;

#[rocket::main]
pub async fn launch_api() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/auth", routes![login, register])
        .mount("/reactivos", routes![create_reactivo, update_reactivo, get_reactivos])
        .mount("/shared", routes![delete_producto])
        .launch()
        .await?;

    Ok(())
}
