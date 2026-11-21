#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::inchi::*;

    #[test]
    fn test_is_inchi() {
        for inchi in [
            "InChI=1S/C2H6O/c1-2-3/h3H,2H2,1H3",
            "InChI=1S/H2O/h1H2",
            "InChI=1S/C9H8O4/c1-6(10)13-8-5-3-2-4-7(8)9(11)12/h2-5H,1H3,(H,11,12)",
        ] {
            assert!(is_inchi(inchi), "{inchi}");
        }
        for not_inchi in ["", "   ", "C2H6O", "inchi=1S/H2O/h1H2", "InChI="] {
            assert!(!is_inchi(not_inchi), "{not_inchi}");
        }
    }

    #[test]
    fn test_is_inchikey() {
        for key in [
            "LFQSCWFLJHTTHZ-UHFFFAOYSA-N", // ethanol
            "BSYNRYMUTXBXSQ-UHFFFAOYSA-N", // aspirin
            "XLYOFNOQVPJJNP-UHFFFAOYSA-N", // water
            "QTBSBXVTEAMEQO-UHFFFAOYSA-M", // acetate
            "RZVAJINKPMORJF-UHFFFAOYNA-N", // non-standard (N flag)
        ] {
            assert!(is_inchikey(key), "{key}");
        }
        for not_key in [
            "",
            "LFQSCWFLJHTTHZ-UHFFFAOYSA",
            "LFQSCWFLJHTTHZ-UHFFFAOYSB-N",
            "LFQSCWFLJHTTHZ-UHFFFAOYXA-N",
            "lfqscwfljhtthz-uhfffaoysa-n",
            " LFQSCWFLJHTTHZ-UHFFFAOYSA-N",
        ] {
            assert!(!is_inchikey(not_key), "{not_key}");
        }
    }
}
