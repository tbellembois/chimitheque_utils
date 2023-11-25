use std::io::Cursor;

use base64::{engine::general_purpose, Engine};
use futures::executor::block_on;
use governor::{
    clock,
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
    RateLimiter,
};
use log::debug;
use serde::{Deserialize, Serialize};
use urlencoding::encode;

// XML shema available at:
// https://pubchem.ncbi.nlm.nih.gov/pug_rest/pug_rest.xsd
// https://pubchem.ncbi.nlm.nih.gov/pug_view/pug_view.xsd

#[derive(Serialize, Deserialize, Debug)]
pub struct AutocompleteTerm {
    compound: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Autocomplete {
    total: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    dictionary_terms: Option<AutocompleteTerm>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Markup {
    #[serde(rename = "Start")]
    start: f64,

    #[serde(rename = "Length")]
    length: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "URL")]
    url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Type")]
    the_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Extra")]
    extra: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StringWithMarkup {
    #[serde(rename = "String")]
    string: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Markup")]
    markup: Option<Vec<Markup>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Number")]
    number: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "DateISO8601")]
    date_iso_8601: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Boolean")]
    boolean: Option<Vec<bool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Binary")]
    binary: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "BinaryToStore")]
    binary_to_store: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ExternalDataURL")]
    external_data_url: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ExternalTableName")]
    external_table_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Unit")]
    unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "MimeType")]
    mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ExternalTableNumRows")]
    external_table_num_rows: Option<isize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "StringWithMarkup")]
    string_with_markup: Option<Vec<StringWithMarkup>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Information {
    #[serde(rename = "ReferenceNumber")]
    reference_number: isize,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Name")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Description")]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Reference")]
    reference: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "LicenseNote")]
    license_note: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "LicenseURL")]
    license_url: Option<String>,

    #[serde(rename = "Value")]
    value: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Section {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "TOCHeading")]
    toc_heading: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "TOCID")]
    toc_id: Option<isize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Description")]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "URL")]
    url: Option<String>,

    #[serde(rename = "Section")]
    section: Option<Vec<Section>>,

    #[serde(rename = "Information")]
    information: Option<Vec<Information>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "RecordType")]
    record_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "RecordNumber")]
    record_number: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "RecordAccession")]
    record_accession: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "RecordTitle")]
    record_title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "RecordExternalURL")]
    record_external_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Section")]
    section: Option<Vec<Section>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Information")]
    information: Option<Vec<Information>>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "Prop_URN")]
pub struct PropURN {
    label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prop {
    urn: PropURN,
    value: PropValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    #[serde(rename = "Record")]
    record: RecordContent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CID {
    cid: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ID {
    id: CID,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "PC_Compound")]
pub struct PCCompound {
    id: ID,
    props: Vec<Prop>,
    record: Option<Record>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Compounds {
    #[serde(rename = "PC_Compounds")]
    pc_compounds: Vec<PCCompound>,

    #[serde(skip_serializing_if = "Option::is_none")]
    record: Option<Record>,

    #[serde(skip_serializing_if = "Option::is_none")]
    base64_png: Option<String>,
}

pub fn autocomplete(
    rate_limiter: &RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>,
    search: &str,
) -> Result<Autocomplete, String> {
    let urlencoded_search = encode(search);

    // Call NCBI REST API.
    block_on(rate_limiter.until_ready());

    let resp = match reqwest::blocking::get(format!(
        "https://pubchem.ncbi.nlm.nih.gov/rest/autocomplete/compound/{urlencoded_search}/json",
    )) {
        Ok(resp) => resp,
        Err(e) => return Err(e.to_string()),
    };

    debug!("resp: {:#?}", resp);

    // Check HTTP code.
    if !resp.status().is_success() {
        return Err(resp.status().to_string());
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

pub fn get_compound_by_name(
    rate_limiter: &RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>,
    name: &str,
) -> Result<Compounds, String> {
    let urlencoded_name = encode(name);

    //
    // Get basic informations.
    //
    // Call NCBI REST API for JSON.
    block_on(rate_limiter.until_ready());

    let query_url =
        format!("https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/{urlencoded_name}/JSON");
    debug!("query_url: {query_url}");

    let resp = match reqwest::blocking::get(query_url) {
        Ok(resp) => resp,
        Err(e) => return Err(e.to_string()),
    };

    debug!("resp.status(): {}", resp.status());

    // Check HTTP code.
    if !resp.status().is_success() {
        return Err(resp.status().to_string());
    }

    // Get response body.
    let body_text = match resp.text() {
        Ok(body_text) => body_text,
        Err(e) => return Err(e.to_string()),
    };

    // Unmarshall into JSON.
    let mut compounds: Compounds = match serde_json::from_str(&body_text.to_owned()) {
        Ok(compounds) => compounds,
        Err(e) => return Err(e.to_string()),
    };

    // Extract compound cid.
    let compound_cid = match compounds.pc_compounds.get(0) {
        Some(compound_cid) => compound_cid.id.id.cid,
        None => return Err("can not find compound cid".to_string()),
    };

    //
    // Get detailed informations.
    //
    // Call NCBI REST API for JSON.
    block_on(rate_limiter.until_ready());

    let query_url =
        format!("https://pubchem.ncbi.nlm.nih.gov/rest/pug_view/data/compound/{compound_cid}/JSON");
    debug!("query_url: {query_url}");

    let resp = match reqwest::blocking::get(query_url) {
        Ok(resp) => resp,
        Err(e) => return Err(e.to_string()),
    };

    debug!("resp.status(): {}", resp.status());

    // Check HTTP code.
    if !resp.status().is_success() {
        return Err(resp.status().to_string());
    }

    // Get response body.
    let body_text = match resp.text() {
        Ok(body_text) => body_text,
        Err(e) => return Err(e.to_string()),
    };

    // Unmarshall into JSON.
    let record: Record = match serde_json::from_str(&body_text.to_owned()) {
        Ok(record) => record,
        Err(e) => return Err(e.to_string()),
    };

    // Update the result.
    compounds.record = Some(record);

    //
    // Get 2d image.
    //
    // Call NCBI REST API for png.
    block_on(rate_limiter.until_ready());

    let query_url =
    format!("https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/{urlencoded_name}/PNG?image_size=300x300");
    debug!("query_url: {query_url}");

    let resp = match reqwest::blocking::get(query_url) {
        Ok(resp) => resp,
        Err(e) => return Err(e.to_string()),
    };

    debug!("resp.status(): {}", resp.status());

    // Check HTTP code.
    if !resp.status().is_success() {
        return Err(resp.status().to_string());
    }

    // Get response body.
    let body_bytes = match resp.bytes() {
        Ok(body_bytes) => body_bytes,
        Err(e) => return Err(e.to_string()),
    };

    // Create image.
    let image = match image::load_from_memory_with_format(&body_bytes, image::ImageFormat::Png) {
        Ok(image) => image,
        Err(e) => return Err(e.to_string()),
    };

    // Convert to base64.
    let mut image_data: Vec<u8> = Vec::new();
    if let Err(e) = image.write_to(
        &mut Cursor::new(&mut image_data),
        image::ImageOutputFormat::Png,
    ) {
        return Err(e.to_string());
    }
    let res_base64 = general_purpose::STANDARD.encode(&image_data);

    // Update the result.
    compounds.base64_png = Some(res_base64);

    Ok(compounds)
}

#[cfg(test)]
mod tests {

    use std::{num::NonZeroU32, time::SystemTime};

    use governor::{Quota, RateLimiter};
    use log::info;

    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_autocomplete() {
        init_logger();

        let rate_limiter = RateLimiter::direct(Quota::per_second(NonZeroU32::new(5).unwrap()));

        info!(
            "aspirine: {:?}",
            autocomplete(&rate_limiter, "aspirine").unwrap()
        );
        info!(
            "DIACETYL-L-TARTARIC ANHYDRIDE: {:?}",
            autocomplete(&rate_limiter, "DIACETYL-L-TARTARIC ANHYDRIDE").unwrap()
        );
        info!("#: {:?}", autocomplete(&rate_limiter, "#").unwrap());
    }

    #[test]
    fn test_get_compound_by_name() {
        init_logger();

        let rate_limiter = RateLimiter::direct(Quota::per_second(NonZeroU32::new(5).unwrap()));

        info!(
            "aspirine: {:#?}",
            get_compound_by_name(&rate_limiter, "aspirine")
        );
        info!(
            "D-Diacetyltartaric anhydride: {:#?}",
            get_compound_by_name(&rate_limiter, "D-Diacetyltartaric anhydride").unwrap()
        );
        info!(
            "(-)-Diacetyl-D-tartaric Anhydride: {:#?}",
            get_compound_by_name(&rate_limiter, "(-)-Diacetyl-D-tartaric Anhydride").unwrap()
        );
        info!(
            "(+)-Diacetyl-L-tartaric anhydride: {:#?}",
            get_compound_by_name(&rate_limiter, "(+)-Diacetyl-L-tartaric anhydride").unwrap()
        );
    }

    #[test]
    fn test_time_limiter() {
        init_logger();

        let rate_limiter = RateLimiter::direct(Quota::per_second(NonZeroU32::new(1).unwrap()));

        let before = SystemTime::now();
        for i in 1..6 {
            debug!("loop {i}");
            let _ = autocomplete(&rate_limiter, "aspirine");
        }
        info!("{:?}", before.elapsed());
        assert!(before.elapsed().unwrap().as_secs() >= 4);

        let before = SystemTime::now();
        for i in 1..6 {
            debug!("loop {i}");
            let _ = get_compound_by_name(&rate_limiter, "aspirine");
        }
        info!("{:?}", before.elapsed());
        assert!(before.elapsed().unwrap().as_secs() >= 4);
    }
}
