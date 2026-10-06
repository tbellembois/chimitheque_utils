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
}
