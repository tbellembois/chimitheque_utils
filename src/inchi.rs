use chimitheque_defines::{INCHI_REGEX, INCHIKEY_REGEX};

/// Returns true if `inchi` looks like an `InChI`, e.g.
/// `InChI=1S/H2O/h1H2`. Only the format of the first layers is checked.
#[must_use]
pub fn is_inchi(inchi: &str) -> bool {
    INCHI_REGEX.is_match(inchi)
}

/// Returns true if `key` is formatted as an `InChIKey`, e.g.
/// `XLYOFNOQVPJJNP-UHFFFAOYSA-N`.
#[must_use]
pub fn is_inchikey(key: &str) -> bool {
    INCHIKEY_REGEX.is_match(key)
}

#[cfg(test)]
#[path = "inchi_tests.rs"]
mod inchi_tests;
