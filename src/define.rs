use regex::Regex;
use std::sync::LazyLock;

pub static CAS_NUMBER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<group1>[0-9]{2,7})-(?P<group2>[0-9]{2})-(?P<checkdigit>[0-9]{1})$").unwrap()
});
pub static CE_NUMBER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<group1>[0-9]{3})-(?P<group2>[0-9]{3})-(?P<checkdigit>[0-9]{1})$").unwrap()
});
pub static ALL_ZERO_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^0+$").unwrap());

pub static INCHI_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^InChI=1[S]?/[A-Z0-9.+-]+(/c[0-9xX*(),-]+)?(/h[0-9hH,+-]+)?.*$").unwrap()
});
pub static INCHIKEY_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Z]{14}-[A-Z]{9}[SN][A][A-Z]-[A-Z]$").unwrap());
