use jsonpath_rust::JsonPathQuery;
use log::debug;
use regex::Regex;
use serde::Serialize;
use serde_json::Value;

// A simplified pubchem product representation.
#[derive(Debug, Default, Serialize)]
pub struct PubchemProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    iupac_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    inchi: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    inchi_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    canonical_smiles: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    molecular_formula: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cas: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ec: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    molecular_weight: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    molecular_weight_unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    boiling_point: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    synonyms: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    signal: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hs: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ps: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub twodpicture: Option<String>, // base64 encoded png
}

impl PubchemProduct {
    pub(crate) fn from_pubchem(json_content: String) -> Option<PubchemProduct> {
        // Precautionary statement regex.
        let precautionary_statement_re = Regex::new(r"(?P<statement>P[0-9]{3}\+{0,1})+").unwrap();
        // Hazard statement regex.
        let hazard_statement_re =
            Regex::new(r"(?P<statement>(AU|EU){0,1}H[0-9]{3}F{0,1}f{0,1}D{0,1}d{0,1}\+{0,1})+")
                .unwrap();

        // Final result.
        let mut product = PubchemProduct {
            ..Default::default()
        };

        let json_content: Value = serde_json::from_str(&json_content).unwrap();

        // Name.
        let name = json_content.clone().path("$.Record.RecordTitle").unwrap();

        debug!("name: {:#?}", name);
        product.name = name
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());

        // IUPAC name.
        let iupac_name = json_content.clone()
        .path("$..Section[?(@.TOCHeading=='IUPAC Name')].Information[0].Value.StringWithMarkup[0].String")
        .unwrap();

        debug!("iupac_name: {:#?}", iupac_name);
        product.iupac_name = iupac_name
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());

        // InChi.
        let inchi = json_content
            .clone()
            .path(
                "$..Section[?(@.TOCHeading=='InChI')].Information[0].Value.StringWithMarkup[0].String",
            )
            .unwrap();

        debug!("inchi: {:#?}", inchi);
        product.inchi = inchi
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());

        // InChi key.
        let inchi_key = json_content
        .clone()
        .path(
            "$..Section[?(@.TOCHeading=='InChIKey')].Information[0].Value.StringWithMarkup[0].String",
        )
        .unwrap();

        debug!("inchi_key: {:#?}", inchi_key);
        product.inchi_key = inchi_key
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());

        // Canonical SMILES.
        let canonical_smiles = json_content
        .clone()
        .path(
            "$..Section[?(@.TOCHeading=='Canonical SMILES')].Information[0].Value.StringWithMarkup[0].String",
        )
        .unwrap();

        debug!("canonical_smiles: {:#?}", canonical_smiles);
        product.canonical_smiles = canonical_smiles
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());

        // Molecular formula.
        let molecular_formula = json_content
        .clone()
        .path(
            "$..Section[?(@.TOCHeading=='Molecular Formula')].Information[0].Value.StringWithMarkup[0].String",
        )
        .unwrap();

        debug!("molecular_formula: {:#?}", molecular_formula);
        product.molecular_formula = molecular_formula
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());

        // CAS.
        let cas = json_content
            .clone()
            .path("$..Section[?(@.TOCHeading=='CAS')].Information[0].Value.StringWithMarkup[0].String")
            .unwrap();

        debug!("cas: {:#?}", cas);
        product.cas = cas
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());

        // EC.
        let ec = json_content
        .clone()
        .path("$..Section[?(@.TOCHeading=='European Community (EC) Number')].Information[0].Value.StringWithMarkup[0].String")
        .unwrap();

        debug!("ec: {:#?}", ec);
        product.ec = ec
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());

        // Synonyms.
        let synonyms = json_content
            .clone()
            .path(
                "$..Section[?(@.TOCHeading=='Synonyms')].Section[?(@.TOCHeading!='Removed Synonyms')]..String",
            )
            .unwrap();

        debug!("synonyms: {:#?}", synonyms);
        product.synonyms = synonyms
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());
        if product.synonyms.is_some() {
            product.synonyms.as_mut().unwrap().sort();
            product.synonyms.as_mut().unwrap().dedup();
        }

        // Molecular weight.
        let molecular_weight = json_content
            .clone()
            .path("$..Section[?(@.TOCHeading=='Molecular Weight')].Information[0].Value.StringWithMarkup[0].String")
            .unwrap();

        debug!("molecular_weight: {:#?}", molecular_weight);
        product.molecular_weight = molecular_weight
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());

        // Molecular weight unit.
        let molecular_weight_unit = json_content
            .clone()
            .path("$..Section[?(@.TOCHeading=='Molecular Weight')].Information[0].Value.Unit")
            .unwrap();

        debug!("molecular_weight_unit: {:#?}", molecular_weight_unit);
        product.molecular_weight_unit = molecular_weight_unit
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());

        // Boiling point.
        let boiling_point = json_content
            .clone()
            .path("$..Section[?(@.TOCHeading=='Boiling Point')].Information[?(@.Description=='PEER REVIEWED')].Value.StringWithMarkup[0].String")
            .unwrap();

        debug!("boiling_point: {:#?}", boiling_point);
        product.boiling_point = boiling_point
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());

        // Symbols.
        let symbols = json_content
            .clone()
            .path("$..Information[?(@.Name=='Pictogram(s)')]..StringWithMarkup..Markup..URL")
            .unwrap();

        debug!("symbols: {:#?}", symbols);
        product.symbols = symbols
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());
        if product.symbols.is_some() {
            product.symbols.as_mut().unwrap().sort();
            product.symbols.as_mut().unwrap().dedup();
        }

        // Signal.
        let signal = json_content
            .clone()
            .path("$..Information[?(@.Name=='Signal')]..StringWithMarkup..String")
            .unwrap();

        debug!("signal: {:#?}", signal);
        product.signal = signal
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());
        if product.signal.is_some() {
            product.signal.as_mut().unwrap().sort();
            product.signal.as_mut().unwrap().dedup();
        }

        // Hazard statements.
        let hs = json_content
            .clone()
            .path("$..Information[?(@.Name=='GHS Hazard Statements')]..StringWithMarkup..String")
            .unwrap();

        debug!("hs: {:#?}", hs);

        let maybe_hs_string_vec: Option<Vec<String>> = hs
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());

        if let Some(hs_string_vec) = maybe_hs_string_vec {
            let hs_string = hs_string_vec.join(",");
            product.hs = hazard_statement_re
                .captures_iter(&hs_string)
                .map(|p| {
                    p.name("statement")
                        .map(|statement| statement.as_str().to_string())
                })
                .collect();
            product.hs.as_mut().unwrap().sort();
            product.hs.as_mut().unwrap().dedup();
        }

        // Precautionary statements.
        let ps = json_content
            .clone()
            .path("$..Information[?(@.Name=='Precautionary Statement Codes')]..StringWithMarkup[0].String")
            .unwrap();

        debug!("ps: {:#?}", ps);

        let maybe_ps_string_vec: Option<Vec<String>> = ps
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());

        if let Some(ps_string_vec) = maybe_ps_string_vec {
            let ps_string = ps_string_vec.join(",");
            product.ps = precautionary_statement_re
                .captures_iter(&ps_string)
                .map(|p| {
                    p.name("statement")
                        .map(|statement| statement.as_str().to_string())
                })
                .collect();
            product.ps.as_mut().unwrap().sort();
            product.ps.as_mut().unwrap().dedup();
        }

        Some(product)
    }
}

#[cfg(test)]
mod tests {

    use std::{
        fs::{self},
        path::Path,
    };

    use log::info;

    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_from_pubchem() {
        init_logger();

        let json_file_path = Path::new("src/testdata/pubchem_pug_view.json");
        let json_string =
            fs::read_to_string(json_file_path).expect("error while opening json file");

        let product = PubchemProduct::from_pubchem(json_string);
        info!("{:#?}", product);
    }
}
