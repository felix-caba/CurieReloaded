#[macro_use]
extern crate rocket;

mod routes;

pub use routes::get_reactivos;
pub use routes::login;
pub use routes::register;

#[rocket::main]
pub async fn launch_api() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/auth", routes![login, register])
        .mount("/reactivos", routes![get_reactivos])
        .launch()
        .await?;

    Ok(())
}
