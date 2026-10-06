use chimitheque_defines::CANONICAL_SMILES_REGEX;

pub fn is_canonical_smiles(inchi: &str) -> bool {
    CANONICAL_SMILES_REGEX.is_match(inchi)
}

#[cfg(test)]
#[path = "smiles_tests.rs"]
mod smiles_tests;
