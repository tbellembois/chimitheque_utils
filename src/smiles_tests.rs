use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethanol_is_valid() {
        let valid_smiles = "CCO";
        assert!(is_smiles(valid_smiles));
    }

    #[test]
    fn test_emoji_is_invalid() {
        let invalid_smiles = "😀";
        assert!(!is_smiles(invalid_smiles));
    }

    #[test]
    fn test_empty_string() {
        let empty_string = "";
        assert!(!is_smiles(empty_string));
    }

    #[test]
    fn test_whitespace() {
        let whitespace = "   ";
        assert!(!is_smiles(whitespace));
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
            assert!(is_smiles(smiles), "{smiles}");
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
            assert!(!is_smiles(smiles), "{smiles}");
        }
    }
}
