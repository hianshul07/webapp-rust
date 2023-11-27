#[macro_use]
extern crate rocket;

#[get("/henlo/<name>/<age>")]
fn henlo(name: &str, age: u8) -> String {
    format!("Henlo, {} hehe {}!", age, name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![henlo])
}
