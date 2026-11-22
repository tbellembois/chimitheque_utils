//! Utilities of Chimithèque, a management application for chemical products,
//! biological reagents and laboratory consumables.
//!
//! It checks chemical identifiers (CAS and CE numbers, `InChI`, SMILES),
//! computes empirical formulas and cleans strings.
#![warn(missing_docs)]

/// CAS registry number check.
pub mod casnumber;
/// European Community number check.
pub mod cenumber;
/// Empirical formula computation.
pub mod formula;
/// `InChI` and `InChIKey` checks.
pub mod inchi;
/// SMILES check.
pub mod smiles;
/// String cleaning.
pub mod string;
