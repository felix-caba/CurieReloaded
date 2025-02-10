#[macro_use]
extern crate rocket;

mod auth_catchers;
mod reactivos_routes;
mod shared_routes;
mod usuarios_routes;
mod locations_routes;

use reactivos_routes::create_reactivo;
use reactivos_routes::get_reactivos;
use reactivos_routes::update_reactivo;
use shared_routes::delete_producto;
use usuarios_routes::login;
use usuarios_routes::register;

#[rocket::main]
pub async fn launch_api() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/api/auth", routes![login, register])
        .mount(
            "/api",
            routes![create_reactivo, update_reactivo, get_reactivos],
        )
        .mount("/api/shared", routes![delete_producto])
        .launch()
        .await?;

    Ok(())
}
