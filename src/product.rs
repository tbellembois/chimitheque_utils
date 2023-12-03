use log::debug;
use serde::Serialize;

use crate::pubchem_type::Compounds;

// A simplified product representation for Chimithèque.
#[derive(Debug, Default, Serialize)]
pub struct Product {
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    twodpicture: Option<String>, // base64 encoded png
}

impl Product {
    pub(crate) fn from_pubchem(compounds: Compounds) -> Option<Product> {
        // Final result.
        let mut product = Product {
            ..Default::default()
        };

        // Reaching the section.
        if let Some(record) = compounds.record {
            // Get product name.
            if let Some(title) = record.record.record_title {
                product.name = title;
            } else {
                debug!("no record title");
                return None;
            }

            if let Some(section) = record.record.section {
                for section_item in section {
                    let toc_heading = match section_item.toc_heading {
                        Some(toc_heading) => toc_heading,
                        None => return Some(product),
                    };
                    debug!("{:?}", toc_heading);

                    match toc_heading.as_str() {
                        // "Information": [
                        //   {
                        //     "ReferenceNumber": 17,
                        //     "Name": "Chemical Safety",
                        //     "Value": {
                        //       "StringWithMarkup": [
                        //         {
                        //           "String": "          ",
                        //           "Markup": [
                        //             {
                        //               "Start": 0,
                        //               "Length": 1,
                        //               "URL": "https://pubchem.ncbi.nlm.nih.gov/images/ghs/GHS05.svg",
                        //               "Type": "Icon",
                        //               "Extra": "Corrosive"
                        //             },
                        //             {
                        //               "Start": 1,
                        //               "Length": 1,
                        //               "URL": "https://pubchem.ncbi.nlm.nih.gov/images/ghs/GHS07.svg",
                        //               "Type": "Icon",
                        //               "Extra": "Irritant"
                        //             }
                        "Chemical Safety" => {
                            section_item
                                .information // maybe slice of Information
                                .and_then(|information| {
                                    information.into_iter().find(|information_item| {
                                        information_item
                                            .name
                                            .eq(&Some("Chemical Safety".to_string()))
                                    })
                                }) // Information
                                .map(|information_chemical_safety| {
                                    information_chemical_safety.value
                                }) // maybe Value
                                .and_then(|value| {
                                    value.string_with_markup.map(|string_with_markup| {
                                        // slice of StringWithMarkup
                                        string_with_markup
                                            .into_iter()
                                            .map(|string_with_markup_item| {
                                                string_with_markup_item.markup.map(|markup_item| {
                                                    product.symbols = Some(
                                                        markup_item
                                                            .into_iter()
                                                            .filter_map(|markup| match markup.url {
                                                                Some(_) => markup.url,
                                                                None => None,
                                                            })
                                                            .collect::<Vec<_>>(),
                                                    );
                                                })
                                            })
                                            .collect::<Vec<_>>()
                                    })
                                });
                            debug!("{:?}", product.symbols)
                        }
                        // "TOCHeading": "Names and Identifiers",
                        // "Description": "Chemical names, synonyms, identifiers, and descriptors.",
                        // "Section": [
                        //   {
                        //     "TOCHeading": "Computed Descriptors",
                        //     "Description": "Structural descriptors generated or computed for the structures of this compound, including the IUPAC name, InChI/InChIKey, and canonical/isomeric SMILES.",
                        //     "Section": [
                        //       {
                        //         "TOCHeading": "IUPAC Name",
                        //         "Description": "Chemical name of this compound, computed from its structure based on the International Union of Pure and Applied Chemistry (IUPAC) nomenclature standards.",
                        //         "URL": "https://iupac.org/what-we-do/nomenclature/",
                        //         "Information": [
                        //           {
                        //             "ReferenceNumber": 17,
                        //             "Reference": [
                        //               "Computed by Lexichem TK 2.7.0 (PubChem release 2021.05.07)"
                        //             ],
                        //             "Value": {
                        //               "StringWithMarkup": [
                        //                 {
                        //                   "String": "[(3R,4R)-4-acetyloxy-2,5-dioxooxolan-3-yl] acetate",
                        "Names and Identifiers" => (),
                        _ => (),
                    };
                }
            } else {
                debug!("no section");
                return None;
            }
        } else {
            debug!("no record");
            return None;
        }

        // 2d picture.
        product.twodpicture = compounds.base64_png;

        Some(product)
    }
}

#[cfg(test)]
mod tests {

    use std::{fs::File, path::Path};

    use log::info;

    use super::*;
    use crate::{pubchem_type::Record, testdata::defines::BASE64_FERRIS};

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_from_pubchem() {
        init_logger();

        let json_file_path = Path::new("src/testdata/pubchem_pug_view.json");
        let file = File::open(json_file_path).expect("error while opening json file");

        let record: Record = serde_json::from_reader(file).expect("error while reading or parsing");

        let mut compounds: Compounds = Compounds {
            ..Default::default()
        };
        compounds.record = Some(record);
        compounds.base64_png = Some(BASE64_FERRIS.to_string());

        let product = Product::from_pubchem(compounds);
        info!("{:#?}", product);
    }
}
