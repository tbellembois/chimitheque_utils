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
use urlencoding::encode;

use crate::pubchem_type::{Autocomplete, Compounds, PropertyTable, Record};

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
    // Get compound CID.
    //
    // Call NCBI REST API for JSON.
    block_on(rate_limiter.until_ready());

    // We need to query at least one property to get the CID. Choosing MolecularFormula.
    let query_url =
        format!("https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/{urlencoded_name}/property/MolecularFormula/JSON");
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
    let property_table: PropertyTable = match serde_json::from_str(&body_text.to_owned()) {
        Ok(property_table) => property_table,
        Err(e) => return Err(e.to_string()),
    };

    // Extract compound cid.
    let compound_cid = match property_table.property_table.properties.first() {
        Some(compound_cid) => compound_cid.cid,
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

    // Create the result.
    let mut compounds = Compounds {
        record: Some(record),
        ..Default::default()
    };

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
