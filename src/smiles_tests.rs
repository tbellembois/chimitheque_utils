#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::smiles::*;

    #[test]
    fn test_is_smiles() {
        for smiles in [
            "CCO",                          // ethanol
            "CC(=O)OC1=CC=CC=C1C(=O)O",     // aspirin
            "CN1C=NC2=C1C(=O)N(C(=O)N2C)C", // caffeine
            "c1ccccc1",                     // aromatic benzene
            "[Na+].[Cl-]",                  // salt
            "C/C=C/C",                      // double bond stereo
            "N[C@@H](C)C(=O)O",             // chirality
            "[2H]O[2H]",                    // isotopes
            "C%12CCC%12",                   // two-digit ring number
        ] {
            assert!(is_smiles(smiles), "{smiles}");
        }
    }

    #[test]
    fn test_is_smiles_rejects_invalid() {
        for smiles in [
            "",     // empty
            "   ",  // whitespace only
            " CCO", // surrounding whitespace
            "😀",   // emoji
            "hello world",
            "###",
            "C1CC",     // unclosed ring
            "Cc1ccccc", // unclosed aromatic ring
            "C(C",      // unclosed branch
            "CC)",      // unopened branch
            "[Xx]",     // unknown element
            "CC=",      // dangling bond
        ] {
            assert!(!is_smiles(smiles), "{smiles:?}");
        }
    }
}
