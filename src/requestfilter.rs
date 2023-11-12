use chimitheque_types::requestfilter::RequestFilter;
use log::debug;
use regex::Regex;
use url::Url;

pub fn request_filter(request: &str) -> Result<RequestFilter, String> {
    debug!("request:{request}");

    // Result populated after by the request parameters.
    let mut request_filter: RequestFilter = Default::default();

    // Parse request URL.
    let url = match Url::parse(request) {
        Ok(url) => url,
        Err(e) => return Err(format!("can not parse url: {}", e)),
    };

    // Regex to validate multi valued ids.
    let ids_match = match Regex::new(r"^((\d+),{0,1})+$") {
        Ok(ids_re) => ids_re,
        Err(e) => return Err(format!("error creating ids_match regex: {e}")),
    };

    // Regex to capture multi valued ids.
    let ids_capture = match Regex::new(r"(?<id>\d+),{0,1}") {
        Ok(ids_re) => ids_re,
        Err(e) => return Err(format!("error creating ids_capture regex: {e}")),
    };

    // Get the query parameters.
    for query_pair in url.query_pairs() {
        debug!("query_pair:{:?}", query_pair);

        match query_pair {
            (std::borrow::Cow::Borrowed(key), std::borrow::Cow::Borrowed(value)) => match key {
                "search" => request_filter.search = format!("%{}%", value),
                "order_by" => request_filter.order_by = value.to_string(),
                "order" => request_filter.order = value.to_string(),
                "offset" => match value.parse::<usize>() {
                    Ok(v) => request_filter.offset = v,
                    Err(e) => return Err(format!("error with offset query parameter: {e}")),
                },
                "limit" => match value.parse::<usize>() {
                    Ok(v) => request_filter.limit = v,
                    Err(e) => return Err(format!("error with limit query parameter: {e}")),
                },
                "bookmark" => match value.parse::<bool>() {
                    Ok(v) => request_filter.bookmark = v,
                    Err(e) => return Err(format!("error with bookmark query parameter: {e}")),
                },
                "borrowing" => match value.parse::<bool>() {
                    Ok(v) => request_filter.borrowing = v,
                    Err(e) => return Err(format!("error with borrowing query parameter: {e}")),
                },
                "cas_number" => match value.parse::<usize>() {
                    Ok(v) => request_filter.cas_number = v,
                    Err(e) => return Err(format!("error with cas_number query parameter: {e}")),
                },
                "cas_number_cmr" => match value.parse::<bool>() {
                    Ok(v) => request_filter.cas_number_cmr = v,
                    Err(e) => {
                        return Err(format!("error with cas_number_cmr query parameter: {e}"))
                    }
                },
                "category" => match value.parse::<usize>() {
                    Ok(v) => request_filter.category = v,
                    Err(e) => return Err(format!("error with category query parameter: {e}")),
                },
                "custom_name_part_of" => request_filter.custom_name_part_of = value.to_string(),
                "empirical_formula" => match value.parse::<usize>() {
                    Ok(v) => request_filter.empirical_formula = v,
                    Err(e) => {
                        return Err(format!("error with empirical_formula query parameter: {e}"))
                    }
                },
                "entity" => match value.parse::<usize>() {
                    Ok(v) => request_filter.entity = v,
                    Err(e) => return Err(format!("error with entity query parameter: {e}")),
                },
                "hazard_statements" => {
                    if !ids_match.is_match(value) {
                        return Err(String::from("invalid hazard_statements ids format"));
                    }

                    let caps = ids_capture.captures_iter(value);
                    for cap in caps {
                        // We can unwrap safely here because of validation (is_match) below.
                        let id_str = cap.name("id").unwrap().as_str();
                        let id = id_str.parse::<usize>().unwrap();

                        request_filter.hazard_statements.push(id);
                    }
                }
                "history" => match value.parse::<bool>() {
                    Ok(v) => request_filter.history = v,
                    Err(e) => return Err(format!("error with history query parameter: {e}")),
                },
                "storages" => {
                    if !ids_match.is_match(value) {
                        return Err(String::from("invalid storages ids format"));
                    }

                    let caps = ids_capture.captures_iter(value);
                    for cap in caps {
                        // We can unwrap safely here because of validation (is_match) below.
                        let id_str = cap.name("id").unwrap().as_str();
                        let id = id_str.parse::<usize>().unwrap();

                        request_filter.storages.push(id);
                    }
                }
                "name" => match value.parse::<usize>() {
                    Ok(v) => request_filter.name = v,
                    Err(e) => return Err(format!("error with name query parameter: {e}")),
                },
                "permission" => request_filter.permission = value.to_string(),
                "precautionary_statements" => {
                    if !ids_match.is_match(value) {
                        return Err(String::from("invalid precautionary_statements ids format"));
                    }

                    let caps = ids_capture.captures_iter(value);
                    for cap in caps {
                        // We can unwrap safely here because of validation (is_match) below.
                        let id_str = cap.name("id").unwrap().as_str();
                        let id = id_str.parse::<usize>().unwrap();

                        request_filter.precautionary_statements.push(id);
                    }
                }
                "producer" => match value.parse::<usize>() {
                    Ok(v) => request_filter.producer = v,
                    Err(e) => return Err(format!("error with producer query parameter: {e}")),
                },
                "producer_ref" => match value.parse::<usize>() {
                    Ok(v) => request_filter.producer_ref = v,
                    Err(e) => return Err(format!("error with producer_ref query parameter: {e}")),
                },
                "product" => match value.parse::<usize>() {
                    Ok(v) => request_filter.product = v,
                    Err(e) => return Err(format!("error with product query parameter: {e}")),
                },
                "product_specificity" => request_filter.product_specificity = value.to_string(),
                "show_bio" => match value.parse::<bool>() {
                    Ok(v) => request_filter.show_bio = v,
                    Err(e) => return Err(format!("error with show_bio query parameter: {e}")),
                },
                "show_chem" => match value.parse::<bool>() {
                    Ok(v) => request_filter.show_chem = v,
                    Err(e) => return Err(format!("error with show_chem query parameter: {e}")),
                },
                "show_consu" => match value.parse::<bool>() {
                    Ok(v) => request_filter.show_consu = v,
                    Err(e) => return Err(format!("error with show_consu query parameter: {e}")),
                },
                "signal_word" => match value.parse::<usize>() {
                    Ok(v) => request_filter.signal_word = v,
                    Err(e) => return Err(format!("error with signal_word query parameter: {e}")),
                },
                "storage" => match value.parse::<usize>() {
                    Ok(v) => request_filter.storage = v,
                    Err(e) => return Err(format!("error with storage query parameter: {e}")),
                },
                "storage_archive" => match value.parse::<bool>() {
                    Ok(v) => request_filter.storage_archive = v,
                    Err(e) => {
                        return Err(format!("error with storage_archive query parameter: {e}"))
                    }
                },
                "storage_barecode" => request_filter.storage_barecode = value.to_string(),
                "storage_batch_number" => request_filter.storage_batch_number = value.to_string(),
                "storage_to_destroy" => match value.parse::<bool>() {
                    Ok(v) => request_filter.storage_to_destroy = v,
                    Err(e) => {
                        return Err(format!(
                            "error with storage_to_destroy query parameter: {e}"
                        ))
                    }
                },
                "store_location" => match value.parse::<usize>() {
                    Ok(v) => request_filter.store_location = v,
                    Err(e) => {
                        return Err(format!("error with store_location query parameter: {e}"))
                    }
                },
                "store_location_can_store" => match value.parse::<bool>() {
                    Ok(v) => request_filter.store_location_can_store = v,
                    Err(e) => {
                        return Err(format!(
                            "error with store_location_can_store query parameter: {e}"
                        ))
                    }
                },
                "supplier" => match value.parse::<usize>() {
                    Ok(v) => request_filter.supplier = v,
                    Err(e) => return Err(format!("error with supplier query parameter: {e}")),
                },
                "symbols" => {
                    if !ids_match.is_match(value) {
                        return Err(String::from("invalid symbols ids format"));
                    }

                    let caps = ids_capture.captures_iter(value);
                    for cap in caps {
                        // We can unwrap safely here because of validation (is_match) below.
                        let id_str = cap.name("id").unwrap().as_str();
                        let id = id_str.parse::<usize>().unwrap();

                        request_filter.symbols.push(id);
                    }
                }
                "tags" => {
                    if !ids_match.is_match(value) {
                        return Err(String::from("invalid tags ids format"));
                    }

                    let caps = ids_capture.captures_iter(value);
                    for cap in caps {
                        // We can unwrap safely here because of validation (is_match) below.
                        let id_str = cap.name("id").unwrap().as_str();
                        let id = id_str.parse::<usize>().unwrap();

                        request_filter.tags.push(id);
                    }
                }
                "unit_type" => request_filter.unit_type = value.to_string(),
                _ => (),
            },
            _ => return Err(String::from("error extracting request query parameters")),
        }
    }

    Ok(request_filter)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_request_filter_ok() {
        init_logger();

        // Valid values.
        let filter = request_filter(
            "http://localhost/?search=foo\
        &order_by=foo\
        &order=foo\
        &offset=10\
        &limit=10\
        &bookmark=true\
        &borrowing=true\
        &cas_number=10\
        &cas_number_cmr=true\
        &category=10\
        &custom_name_part_of=foo\
        &empirical_formula=10\
        &entity=10\
        &hazard_statements=1,2,3\
        &history=true\
        &storages=1,2,3\
        &name=10\
        &permission=foo\
        &precautionary_statements=1,2,3\
        &producer=10\
        &producer_ref=10\
        &product=10\
        &product_specificity=foo\
        &show_bio=true\
        &show_chem=true\
        &show_consu=true\
        &signal_word=10\
        &storage=10\
        &storage_archive=true\
        &storage_barecode=foo\
        &storage_batch_number=foo\
        &storage_to_destroy=true\
        &store_location=10\
        &store_location_can_store=true\
        &supplier=10\
        &symbols=1,2,3\
        &tags=1,2,3\
        &unit_type=foo",
        );

        assert_eq!(filter.clone().unwrap().search, "%foo%");
        assert_eq!(filter.clone().unwrap().order_by, "foo");
        assert_eq!(filter.clone().unwrap().order, "foo");
        assert_eq!(filter.clone().unwrap().offset, 10);
        assert_eq!(filter.clone().unwrap().limit, 10);
        assert!(filter.clone().unwrap().bookmark);
        assert!(filter.clone().unwrap().borrowing);
        assert_eq!(filter.clone().unwrap().cas_number, 10);
        assert!(filter.clone().unwrap().cas_number_cmr);
        assert_eq!(filter.clone().unwrap().category, 10);
        assert_eq!(filter.clone().unwrap().custom_name_part_of, "foo");
        assert_eq!(filter.clone().unwrap().empirical_formula, 10);
        assert_eq!(filter.clone().unwrap().entity, 10);
        assert_eq!(filter.clone().unwrap().hazard_statements, vec![1, 2, 3]);
        assert!(filter.clone().unwrap().history);
        assert_eq!(filter.clone().unwrap().storages, vec![1, 2, 3]);
        assert_eq!(filter.clone().unwrap().name, 10);
        assert_eq!(filter.clone().unwrap().permission, "foo");
        assert_eq!(
            filter.clone().unwrap().precautionary_statements,
            vec![1, 2, 3]
        );
        assert_eq!(filter.clone().unwrap().producer, 10);
        assert_eq!(filter.clone().unwrap().producer_ref, 10);
        assert_eq!(filter.clone().unwrap().product, 10);
        assert_eq!(filter.clone().unwrap().product_specificity, "foo");
        assert!(filter.clone().unwrap().show_bio);
        assert!(filter.clone().unwrap().show_chem);
        assert!(filter.clone().unwrap().show_consu);
        assert_eq!(filter.clone().unwrap().signal_word, 10);
        assert_eq!(filter.clone().unwrap().storage, 10);
        assert!(filter.clone().unwrap().storage_archive);
        assert_eq!(filter.clone().unwrap().storage_barecode, "foo");
        assert_eq!(filter.clone().unwrap().storage_batch_number, "foo");
        assert!(filter.clone().unwrap().storage_to_destroy);
        assert_eq!(filter.clone().unwrap().store_location, 10);
        assert!(filter.clone().unwrap().store_location_can_store);
        assert_eq!(filter.clone().unwrap().supplier, 10);
        assert_eq!(filter.clone().unwrap().symbols, vec![1, 2, 3]);
        assert_eq!(filter.clone().unwrap().tags, vec![1, 2, 3]);
        assert_eq!(filter.clone().unwrap().unit_type, "foo");

        // Invalid values.
        let param_int = vec![
            "offset",
            "limit",
            "cas_number",
            "category",
            "empirical_formula",
            "entity",
            "name",
            "producer",
            "producer_ref",
            "product",
            "signal_word",
            "storage",
            "store_location",
            "supplier",
        ];
        // let param_string = vec![
        //     "search",
        //     "order_by",
        //     "order",
        //     "custom_name_part_of",
        //     "permission",
        //     "product_specificity",
        //     "storage_barecode",
        //     "storage_batch_number",
        //     "unit_type",
        // ];
        let param_bool = vec![
            "bookmark",
            "borrowing",
            "cas_number_cmr",
            "history",
            "show_bio",
            "show_chem",
            "show_consu",
            "storage_archive",
            "storage_to_destroy",
            "store_location_can_store",
        ];
        let param_vec_int = vec![
            "hazard_statements",
            "storages",
            "precautionary_statements",
            "symbols",
            "tags",
        ];

        for param in param_int {
            // test not digit
            let filter = request_filter(&format!("http://localhost/?{param}=ab"));
            assert!(filter.is_err());
        }

        for param in param_bool {
            // test not bool
            let filter = request_filter(&format!("http://localhost/?{param}=ab"));
            assert!(filter.is_err());
        }

        // for param in param_string {
        //     // test empty string
        //     let filter = request_filter(&format!("http://localhost/?{param}="));
        //     assert!(filter.is_err());
        // }

        for param in param_vec_int {
            // test not digit
            let filter = request_filter(&format!("http://localhost/?{param}=A"));
            assert!(filter.is_err());

            let filter = request_filter(&format!("http://localhost/?{param}=1,2,A"));
            assert!(filter.is_err());

            // test wrong separator
            let filter = request_filter(&format!("http://localhost/?{param}=1;2"));
            assert!(filter.is_err());
        }
    }
}
