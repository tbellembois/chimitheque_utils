use chimitheque_defines::{INCHI_REGEX, INCHIKEY_REGEX};

pub fn is_inchi(inchi: &str) -> bool {
    INCHI_REGEX.is_match(inchi)
}

pub fn is_inchikey(key: &str) -> bool {
    INCHIKEY_REGEX.is_match(key)
}
