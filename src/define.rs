use regex::Regex;

pub static CAS_NUMBER_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(r"^(?P<group1>[0-9]{2,7})-(?P<group2>[0-9]{2})-(?P<checkdigit>[0-9]{1})$").unwrap()
});
pub static CE_NUMBER_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(r"^(?P<group1>[0-9]{3})-(?P<group2>[0-9]{3})-(?P<checkdigit>[0-9]{1})$").unwrap()
});
pub static ALL_ZERO_RE: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(r"^0+$").unwrap());
