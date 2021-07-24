use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use simple_error::bail;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct DictionaryItem {
    word: String,
    dict_type: DictionaryType,
    translations: Vec<String>,
}

impl DictionaryItem {
    fn new(word: &str, dict_type: DictionaryType, translations: Vec<String>) -> DictionaryItem {
        DictionaryItem {
            word: word.to_string(),
            dict_type,
            translations,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
enum DictionaryType {
    Noun(String),
    Verb,
    Adverb,
    Adjective,
    Preposition,
    Conjunction,
    None,
}

impl DictionaryType {
    fn is_none(&self) -> bool {
        if let DictionaryType::None = self {
            true
        } else {
            false
        }
    }
}

pub async fn retrieve_translation(word: &str) -> Result<String, Box<dyn std::error::Error>> {
    let base_url = "https://en.bab.la/dictionary/dutch-english/".to_owned();
    let url = base_url + word;
    let resp = reqwest::get(url).await?.text().await?;
    // parses string of HTML as a document
    let fragment = Html::parse_document(&resp);
    // parses based on a CSS selector
    let stories = Selector::parse("div.quick-result-entry").unwrap();

    let mut translations: Vec<DictionaryItem> = Vec::new();
    // iterate over elements matching our selector
    for story in fragment.select(&stories) {
        // grab the headline text and place into a vector
        let story_txt = story.text().collect::<Vec<_>>();
        if story_txt.len() < 7 || story_txt[6].contains("\n") {
            continue;
        }
        let match_dict_type = match_type(story_txt[8]);
        if match_dict_type.is_none() {
            continue;
        }
        let dict_item = DictionaryItem::new(
            story_txt[6],
            match_dict_type,
            story_txt[15..]
                .to_vec()
                .iter()
                .filter(|&s| !s.contains("\n"))
                .map(|&s| s.to_string())
                .collect::<Vec<_>>(),
        );
        // println!(
        //   "{:?}", dict_item
        // );
        translations.push(dict_item);
    }
    if translations.len() == 0 {
        bail!("No translation found for: {}", word)
    }
    Ok(serde_json::to_string(&translations).unwrap())
}

fn match_type(t: &str) -> DictionaryType {
    match t {
        "{het}" => DictionaryType::Noun("het".to_string()),
        "{de}" => DictionaryType::Noun("de".to_string()),
        "{vb}" => DictionaryType::Verb,
        "{adv.}" => DictionaryType::Adverb,
        "{adj.}" => DictionaryType::Adjective,
        "{prp.}" => DictionaryType::Preposition,
        "{conj.}" => DictionaryType::Conjunction,
        _ => DictionaryType::None,
    }
}
