mod conjugation;
mod dictionary;
use conjugation::retrieve_conjugation;
use dictionary::retrieve_translation;
// use rocket::http::{ContentType, Header, Method};
use rocket::http::{Header};
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

#[macro_use]
extern crate rocket;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        println!("Setting access control allow origin");
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        // response.set_header(Header::new(
        //     "Access-Control-Allow-Methods",
        //     "POST, GET, PATCH, OPTIONS",
        // ));
        // response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        // response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

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
    rocket::build().attach(CORS).mount("/dutchmate-dictionary/", routes![conjugate])
                                .mount("/dutchmate-dictionary/nl-to-en/", routes![translate])
}
