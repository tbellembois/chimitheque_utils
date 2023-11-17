use log::debug;

use serde::Deserialize;
use urlencoding::encode;

#[derive(Deserialize, Debug, Default)]
pub struct AutocompleteTerm {
    compound: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Autocomplete {
    total: usize,

    #[serde(default)]
    dictionary_terms: AutocompleteTerm,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Prop_value")]
enum PropValue {
    #[serde(rename = "ival")]
    Ival(isize),
    #[serde(rename = "fval")]
    Fval(f64),
    #[serde(rename = "binary")]
    Binary(String),
    #[serde(rename = "sval")]
    Sval(String),
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Prop_URN")]
pub struct PropURN {
    label: String,

    #[serde(default)]
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Prop {
    urn: PropURN,
    value: PropValue,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "PC_Compound")]
pub struct PCCompound {
    props: Vec<Prop>,
}

#[derive(Deserialize, Debug)]
pub struct Compounds {
    #[serde(rename = "PC_Compounds")]
    pc_compounds: Vec<PCCompound>,
}

pub fn autocomplete(search: &str) -> Result<Autocomplete, String> {
    let urlencoded_search = encode(search);

    // Call NCBI REST API.
    let resp = match reqwest::blocking::get(format!(
        "https://pubchem.ncbi.nlm.nih.gov/rest/autocomplete/compound/{urlencoded_search}/json",
    )) {
        Ok(resp) => resp,
        Err(e) => return Err(e.to_string()),
    };

    debug!("resp: {:#?}", resp);

    // Check HTTP code.
    if !resp.status().is_success() {
        // FIXME
        return Err("TODO".to_string());
    }

    // Get response body.
    let body_text = match resp.text() {
        Ok(body_text) => body_text,
        Err(e) => return Err(e.to_string()),
    };

    debug!("body_text: {:?}", body_text);

    // Unmarshall into JSON.
    let autocomplete: Autocomplete = match serde_json::from_str(&body_text.to_owned()) {
        Ok(autocomplete) => autocomplete,
        Err(e) => return Err(e.to_string()),
    };

    Ok(autocomplete)
}

pub fn get_compound_by_name(name: &str) -> Result<Compounds, String> {
    let urlencoded_name = encode(name);

    // Call NCBI REST API.
    let resp = match reqwest::blocking::get(format!(
        "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/{urlencoded_name}/JSON",
    )) {
        Ok(resp) => resp,
        Err(e) => return Err(e.to_string()),
    };

    debug!("resp: {:#?}", resp);

    // Check HTTP code.
    if !resp.status().is_success() {
        // FIXME
        return Err("TODO".to_string());
    }

    // Get response body.
    let body_text = match resp.text() {
        Ok(body_text) => body_text,
        Err(e) => return Err(e.to_string()),
    };

    debug!("body_text: {:?}", body_text);

    // Unmarshall into JSON.
    let compounds: Compounds = match serde_json::from_str(&body_text.to_owned()) {
        Ok(compounds) => compounds,
        Err(e) => return Err(e.to_string()),
    };

    Ok(compounds)
}

#[cfg(test)]
mod tests {

    use log::info;

    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_autocomplete() {
        init_logger();

        info!("aspirine: {:?}", autocomplete("aspirine").unwrap());
        info!(
            "DIACETYL-L-TARTARIC ANHYDRIDE: {:?}",
            autocomplete("DIACETYL-L-TARTARIC ANHYDRIDE").unwrap()
        );
        info!("#: {:?}", autocomplete("#").unwrap());
    }

    #[test]
    fn test_get_compound_by_name() {
        init_logger();

        info!("aspirine: {:?}", get_compound_by_name("aspirine"));
        info!(
            "D-Diacetyltartaric anhydride: {:?}",
            get_compound_by_name("D-Diacetyltartaric anhydride").unwrap()
        );
        info!(
            "(-)-Diacetyl-D-tartaric Anhydride: {:?}",
            get_compound_by_name("(-)-Diacetyl-D-tartaric Anhydride").unwrap()
        );
        info!(
            "(+)-Diacetyl-L-tartaric anhydride: {:?}",
            get_compound_by_name("(+)-Diacetyl-L-tartaric anhydride").unwrap()
        );
    }
}
