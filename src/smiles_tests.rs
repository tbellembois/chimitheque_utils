use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_smiles() {
        let canonical_smiles = "CCO";
        assert!(is_canonical_smiles(canonical_smiles));
    }

    #[test]
    fn test_non_canonical_smiles() {
        let non_canonical_smiles = "😀";
        assert!(!is_canonical_smiles(non_canonical_smiles));
    }

    #[test]
    fn test_empty_string() {
        let empty_string = "";
        assert!(!is_canonical_smiles(empty_string));
    }

    #[test]
    fn test_whitespace() {
        let whitespace = "   ";
        assert!(!is_canonical_smiles(whitespace));
    }

    #[test]
    fn test_valid_smiles() {
        for smiles in [
            "CC(=O)OC1=CC=CC=C1C(=O)O",     // aspirin
            "CN1C=NC2=C1C(=O)N(C(=O)N2C)C", // caffeine
            "c1ccccc1",                     // aromatic benzene
            "[Na+].[Cl-]",                  // salt
            "C/C=C/C",                      // double bond stereo
            "N[C@@H](C)C(=O)O",             // chirality
            "[2H]O[2H]",                    // isotopes
            "C%12CCC%12",                   // two-digit ring number
        ] {
            assert!(is_canonical_smiles(smiles), "{smiles}");
        }
    }

    #[test]
    fn test_invalid_smiles() {
        for smiles in [
            "hello world",
            "###",
            "C1CC",     // unclosed ring
            "Cc1ccccc", // unclosed aromatic ring
            "C(C",      // unclosed branch
            "CC)",      // unopened branch
            "[Xx]",     // unknown element
            "CC=",      // dangling bond
            " CCO",     // surrounding whitespace
        ] {
            assert!(!is_canonical_smiles(smiles), "{smiles}");
        }
    }
}
