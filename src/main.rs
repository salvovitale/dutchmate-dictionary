
mod conjugation;
use conjugation::retrieve_conjugation;

#[macro_use] extern crate rocket;

#[get("/conjugate/<verb>")]
async fn conjugate(verb: &str) -> String {
    retrieve_conjugation(verb).await.unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![conjugate])
}