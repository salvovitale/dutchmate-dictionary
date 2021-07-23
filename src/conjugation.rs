use serde::{Deserialize, Serialize};
use scraper::{Html, Selector};
use std::process;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ConjBase{
    pub pronoun: String,
    pub conj: String
}

#[derive(Debug, Deserialize, Serialize)]
struct ConjVerb{
    first_singular: ConjBase,
    second_singular: ConjBase,
    third_singular: ConjBase,
    first_plural: ConjBase,
    second_plural: ConjBase,
    third_plural: ConjBase
}
#[derive(Debug, Deserialize, Serialize)]
struct EntireConj{
    present: ConjVerb,
    present_perfect: ConjVerb,
    simple_past: ConjVerb,
    past_perfect: ConjVerb,
    future: ConjVerb,
    future_perfect: ConjVerb,
    conditional: ConjVerb,
    past_conditional: ConjVerb
}

fn unpack_verb_conj(vec_conj: Vec<ConjBase>) -> ConjVerb {
  ConjVerb{
      first_singular: vec_conj[0].clone(),
      second_singular: vec_conj[1].clone(),
      third_singular: vec_conj[2].clone(),
      first_plural: vec_conj[3].clone(),
      second_plural: vec_conj[4].clone(),
      third_plural: vec_conj[5].clone()
  }
}

pub async fn retrieve_conjugation(verb: &str) -> Result<String, Box<dyn std::error::Error>> {
    let base_url = "https://en.bab.la/conjugation/dutch/".to_owned();
    let url = base_url + verb;
    println!("Retrieving conjugation for {}", url);
    let resp = reqwest::get(url)
    .await?
    .text()
    .await?;
    // parses string of HTML as a document
    let fragment = Html::parse_document(&resp);
    // parses based on a CSS selector
    let stories = Selector::parse("div.conj-item").unwrap();

    let mut v: Vec<ConjBase> = Vec::new();
    // iterate over elements matching our selector
    for story in fragment.select(&stories) {
         // grab the headline text and place into a vector
         let story_txt = story.text().collect::<Vec<_>>();
         if !story_txt[1].contains("\n"){
            v.push(ConjBase{pronoun: String::from(story_txt[1]), conj: String::from(story_txt[3])})
         }
    }
    //TODO better error handling
    if v.len()== 0 {
      eprintln!("No conjugation found for: {}", verb);
      process::exit(1);
    }
    let entire_conj = EntireConj{
        present: unpack_verb_conj(v[0..6].to_vec()),
        present_perfect: unpack_verb_conj(v[6..12].to_vec()),
        simple_past: unpack_verb_conj(v[12..18].to_vec()),
        past_perfect: unpack_verb_conj(v[18..24].to_vec()),
        future: unpack_verb_conj(v[24..30].to_vec()),
        future_perfect: unpack_verb_conj(v[30..36].to_vec()),
        conditional: unpack_verb_conj(v[36..42].to_vec()),
        past_conditional: unpack_verb_conj(v[42..48].to_vec())
    };
    //TODO cast error instead of unwrap here
    Ok(serde_json::to_string(&entire_conj).unwrap())
}