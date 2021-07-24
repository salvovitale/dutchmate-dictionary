
mod conjugation;
mod dictionary;
use conjugation::retrieve_conjugation;
use dictionary::retrieve_translation;

#[macro_use] extern crate rocket;

#[get("/conjugate/<verb>")]
async fn conjugate(verb: &str) -> String {
    retrieve_conjugation(verb).await.unwrap()
}

#[get("/translate/<word>")]
async fn translate(word: &str) -> String {
    retrieve_translation(word).await.unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![conjugate, translate])
}