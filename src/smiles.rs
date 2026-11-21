use purr::{graph::Builder, read::read};

/// Returns true if `smiles` is a syntactically valid SMILES string, following
/// the `OpenSMILES` grammar: known elements, balanced branches and brackets, and
/// closed rings.
///
/// This checks the syntax only: it does not check that the string is in a
/// canonical form, nor its chemistry (e.g. valences). Surrounding whitespace is
/// not accepted.
#[must_use]
pub fn is_canonical_smiles(smiles: &str) -> bool {
    let mut builder = Builder::new();
    // `read` checks the grammar; `build` checks that every ring is closed.
    read(smiles, &mut builder, None).is_ok() && builder.build().is_ok()
}

#[cfg(test)]
#[path = "smiles_tests.rs"]
mod smiles_tests;
