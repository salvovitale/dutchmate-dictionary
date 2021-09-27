use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use simple_error::bail;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ConjTense {
    pub header: String,
    pub conj: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct EntireConj {
    pub pronouns: Vec<String>,
    pub entire_conj: Vec<GroupConj>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GroupConj {
    pub group_name: String,
    pub group_conj: Vec<ConjTense>,
}




pub async fn retrieve_conjugation(verb: &str) -> Result<String, Box<dyn std::error::Error>> {
    let base_url = "https://en.bab.la/conjugation/dutch/".to_owned();
    let url = base_url + verb;
    let resp = reqwest::get(url).await?.text().await?;
    // parses string of HTML as a document
    let fragment = Html::parse_document(&resp);

    // retrieve values by parsing the document using CSS selectors
    let repeated_pronouns = retrieve_item(&fragment, "div.conj-person").unwrap();
    let all_conjugated_forms = retrieve_item(&fragment, "div.conj-result").unwrap();
    let headers = retrieve_item(&fragment, "h3.conj-tense-block-header").unwrap();
    let groups = retrieve_item(&fragment, "div.conj-block h3").unwrap();

    // package all the retrieved values into a struct
    let conj_tense = unpack_conj(&all_conjugated_forms, &headers);
    let pronouns = unpack_pronouns(&repeated_pronouns);
    let conj_by_groups = create_conj_groups(&groups, &conj_tense);

    // construct the final struct
    let entire_conj = EntireConj {
        entire_conj: conj_by_groups,
        pronouns: pronouns,
    };


    //TODO cast error instead of unwrap here
    // convert the struct to a JSON string
    Ok(serde_json::to_string(&entire_conj).unwrap())
}

fn retrieve_item(fragment: &Html, selector: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {

    let result = Selector::parse(selector).unwrap();
    let mut v: Vec<String> = Vec::new();

    // iterate over elements matching our selector
    for element in fragment.select(&result) {
        // grab the headline text and place into a vector
        let element_txt = element.text().collect::<Vec<_>>();
        if element_txt.len() > 0 {
            v.push(element_txt[0].to_string())
        }
    }
    if v.len() == 0 {
        bail!("No match found for selector: {}", selector)
    }
    Ok(v)
}

fn unpack_conj(conjs: &Vec<String>, headers: &Vec<String>) -> Vec<ConjTense> {
    let mut v: Vec<ConjTense> = Vec::new();
    let conj_param = vec![6,6,6,6,6,6,6,6,2,1,1,0];
    let mut bottom = 0;
    let mut top = conj_param[0];
    for i in 0..headers.len() {
        let ct = ConjTense {
            header: headers[i].clone(),
            conj: conjs[bottom..top].to_vec(),
        };
        bottom = top;
        top += conj_param[i+1];
        v.push(ct)
    }
    v
}

fn unpack_pronouns(pronouns: &Vec<String>) -> Vec<String> {
    pronouns[0..6].to_vec()
}

fn create_conj_groups(groups: &Vec<String>, conj_tense: &Vec<ConjTense>) -> Vec<GroupConj> {
    let mut v: Vec<GroupConj> = Vec::new();
    let group_struct= vec![6,2,1,2,0];
    let mut bottom = 0;
    let mut top = group_struct[0];
    for i in 0..groups.len() {
        let ct = GroupConj{
            group_name: groups[i].clone(),
            group_conj: conj_tense[bottom..top].to_vec(),
        };
        bottom = top;
        top += group_struct[i+1];
        v.push(ct)
    }
    v

}
