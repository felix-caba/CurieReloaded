#[macro_use] extern crate rocket;

mod routes;
use rocket::http::Status;
use rocket::Request;
pub use routes::login;
pub use routes::register;
pub use routes::get_reactivos;

#[rocket::main]
pub async fn launch_api() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/auth", routes![login, register])
        .mount("/productos", routes![get_reactivos])
        .launch()

        .await?;

    Ok(())
}




