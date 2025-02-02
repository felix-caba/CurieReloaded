use api::launch_api;
use database::establish_connection;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    establish_connection();
    launch_api()?;
    Ok(())
}