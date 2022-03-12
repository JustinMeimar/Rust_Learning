#[macro_use] extern crate rocket;
use rocket::http::Status;
use rocket::response::{content, status};


#[get("/hello")]
fn hello() -> &'static str {
    "Hello World"
}
 
#[get("/<name>")]
fn hello_name(name: &str) -> String {
    format!("Hello, {}", name)
}

/*
#[get("/<id>", rank=1)]
fn json(id: i32) -> status::Custom<content::Json<&'static str>> {

    let jsonResponse = format!(" \"hi\": \"world {}\" ", id).to_string();
    status::Custom(Status::ImATeapot, content::Json(jsonResponse))
}
*/

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello, json])
}
