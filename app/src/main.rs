#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    "Prometheus".to_owned()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1/", routes![index])
}
