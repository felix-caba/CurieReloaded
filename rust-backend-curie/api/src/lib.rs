#[macro_use] extern crate rocket;

mod routes;
pub use routes::login;
pub use routes::register;

#[rocket::main]
pub async fn launch_api() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/auth", routes![login, register])
        .launch()

        .await?;

    Ok(())
}




