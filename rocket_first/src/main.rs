#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello World"
}

#[get("/<name>")]
fn hello_name(name: &str) -> String {
    format!("Hello, {}", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello, hello_name])
}
