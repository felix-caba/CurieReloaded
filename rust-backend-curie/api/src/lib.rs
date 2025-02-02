#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Test"
}

#[rocket::main]
pub async fn launch_api() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/hello", routes![index])
        .launch()
        .await?;

    Ok(())
}




