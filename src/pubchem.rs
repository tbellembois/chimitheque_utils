use serde::Deserialize;
use urlencoding::encode;

#[derive(Deserialize)]
pub struct Autocomplete {
    total: usize,
    dictionary_terms: Vec<String>,
}

pub fn autocomplete(search: &str) -> Result<Autocomplete, String> {
    let urlencoded_search = encode(search);

    // Call NCBI REST API.
    let resp = match reqwest::blocking::get(format!(
        "https://pubchem.ncbi.nlm.nih.gov/rest/autocomplete/compound/{urlencoded_search}/jsonp",
    )) {
        Ok(resp) => resp,
        Err(e) => return Err(e.to_string()),
    };

    // Decode JSON.
    let autocomplete: Autocomplete = match resp.json() {
        Ok(autocomplete) => autocomplete,
        Err(e) => return Err(e.to_string()),
    };

    Ok(autocomplete)
}

pub fn get_compound_by_name(name: &str) -> Result<String, String> {
    Ok(String::from(""))
}
