#[macro_use] extern crate rocket;


mod routes;

use routes::create_reactivo;
pub use routes::login;
pub use routes::register;
pub use routes::get_reactivos;

#[rocket::main]
pub async fn launch_api() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/auth", routes![login, register])
        .mount("/productos", routes![get_reactivos, create_reactivo])
        .launch()

        .await?;

    Ok(())
}




