use chimitheque_defines::PERIODIC_TABLE;
use log::debug;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    num::ParseIntError,
};

/// Why an empirical formula can not be computed from a formula.
#[derive(Debug, PartialEq, Eq)]
pub enum ToEmpiricalFormulaError {
    /// A parenthesis or bracket is closed without being opened, or is not
    /// closed.
    UnbalancedParenthesis,
    /// An atom is not in the periodic table, e.g. `Xy`.
    UnknowAtom(String),
    /// A number is too big.
    CanNotParseNumber(ParseIntError),
    /// A number follows an unknown atom. Not returned: an unknown atom is
    /// rejected first.
    NumberAfterUnknowAtom,
    /// An atom has no count. Not returned anymore.
    UnexpectedNoneAtomCount(String),
    /// The formula is empty.
    EmptyFormula,
}

impl Display for ToEmpiricalFormulaError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ToEmpiricalFormulaError::UnbalancedParenthesis => write!(f, "unbalanced parenthesis"),
            ToEmpiricalFormulaError::UnknowAtom(s) => write!(f, "unknown atom {s}"),
            ToEmpiricalFormulaError::CanNotParseNumber(e) => {
                write!(f, "can not parse number: {e}")
            }
            ToEmpiricalFormulaError::NumberAfterUnknowAtom => {
                write!(f, "found a number after no known atom")
            }
            ToEmpiricalFormulaError::UnexpectedNoneAtomCount(s) => {
                write!(f, "unexpected empty atom_count_map value for key {s}")
            }
            ToEmpiricalFormulaError::EmptyFormula => write!(f, "empty formula"),
        }
    }
}

impl std::error::Error for ToEmpiricalFormulaError {}

/// Computes the empirical formula of a formula, e.g. `C2H6O` for `CH3CH2OH`.
///
/// Atoms are sorted C, H, then the others in alphabetical order. A number after
/// an atom or a closing parenthesis or bracket multiplies it. A number at the
/// start of a part after a `.` or `·` multiplies the whole part, as in `PubChem`,
/// e.g. `CuSO4.5H2O` gives `H10CuO9S`. Other chars are ignored, and so is a
/// number after them.
///
/// Example of parsing method:
/// Cl(CaC2(NaCl)3)2.Na=P
/// ^^. .. . . .. . .      Cl c=1
///   ^ .. . . .. . .      open block 1
///    ^^. . . .. . .      Ca c=1
///      ^ . . .. . .      C  c=2
///        ^ . .. . .      open block 2
///         ^^ .. . .      Na c=1
///           ^^. . .      Cl c=1
///             ^ . .      close block 2, multiply its atoms by 3 (Na c=3 Cl c=3)
///               ^ .      close block 1, multiply its atoms by 2 (Ca c=2 C c=4 Na c=6 Cl c=6)
///                 ^      start a new part; forget any other char, and a number after it
pub fn to_empirical_formula(formula: &str) -> Result<String, ToEmpiricalFormulaError> {
    // What a number found while parsing applies to.
    enum Previous {
        // The last found atom.
        Atom,
        // The atoms of the block just closed, from this index of `atoms`.
        ClosedBlock(usize),
        // A `.` or `·` separator: the number multiplies the next part.
        Separator,
        // Nothing: the number is ignored.
        Other,
    }

    // Multiplies the counts of `atoms` by `factor`.
    fn multiply(atoms: &mut [(String, usize)], factor: usize) {
        for (_, count) in atoms {
            *count = count.saturating_mul(factor);
        }
    }

    // Rejecting empty formulas.
    if formula.is_empty() {
        return Err(ToEmpiricalFormulaError::EmptyFormula);
    }

    // Creating a vec from input for parsing.
    let formula_vec: Vec<char> = formula.chars().collect();

    // The found atoms, with their count.
    let mut atoms: Vec<(String, usize)> = Vec::new();
    // For each open block, the index in `atoms` of its first atom.
    let mut open_blocks: Vec<usize> = Vec::new();
    // The index in `atoms` of the first atom of the current part, and the
    // number multiplying the part, e.g. `5` for the `5H2O` part of `CuSO4.5H2O`.
    let mut part_start = 0;
    let mut part_multiplier = 1;
    let mut previous = Previous::Other;

    // Cursor index while parsing the formula.
    let mut cursor_index = 0;

    // Parsing the formula.
    while cursor_index < formula_vec.len() {
        let current_char = formula_vec[cursor_index];
        debug!("current_char: {current_char}");

        match current_char {
            '.' | '·' => {
                // A new part.
                multiply(&mut atoms[part_start..], part_multiplier);
                part_start = atoms.len();
                part_multiplier = 1;
                previous = Previous::Separator;
                cursor_index += 1;
            }
            '(' | '[' => {
                open_blocks.push(atoms.len());
                previous = Previous::Other;
                cursor_index += 1;
            }
            ')' | ']' => {
                let Some(block_start) = open_blocks.pop() else {
                    return Err(ToEmpiricalFormulaError::UnbalancedParenthesis);
                };
                previous = Previous::ClosedBlock(block_start);
                cursor_index += 1;
            }
            'A'..='Z' => {
                // A one or two chars atom.
                let atom_len = match formula_vec.get(cursor_index + 1) {
                    Some(next_char) if next_char.is_ascii_lowercase() => 2,
                    _ => 1,
                };
                let atom: String = formula_vec[cursor_index..cursor_index + atom_len]
                    .iter()
                    .collect();

                // Does the atom exists?
                if !PERIODIC_TABLE.contains_key(atom.as_str()) {
                    return Err(ToEmpiricalFormulaError::UnknowAtom(atom));
                }
                debug!("found atom: {atom}");

                atoms.push((atom, 1));
                previous = Previous::Atom;
                cursor_index += atom_len;
            }
            '0'..='9' => {
                // Reading all the digits of the number.
                let number_len = formula_vec[cursor_index..]
                    .iter()
                    .take_while(|c| c.is_ascii_digit())
                    .count();
                let number_string: String = formula_vec[cursor_index..cursor_index + number_len]
                    .iter()
                    .collect();
                let count = number_string
                    .parse::<usize>()
                    .map_err(ToEmpiricalFormulaError::CanNotParseNumber)?;
                debug!("count: {count}");

                match previous {
                    Previous::Atom => {
                        let Some((_, atom_count)) = atoms.last_mut() else {
                            return Err(ToEmpiricalFormulaError::NumberAfterUnknowAtom);
                        };
                        *atom_count = count;
                    }
                    Previous::ClosedBlock(block_start) => {
                        multiply(&mut atoms[block_start..], count);
                    }
                    Previous::Separator => part_multiplier = count,
                    Previous::Other => (),
                }

                previous = Previous::Other;
                cursor_index += number_len;
            }
            _ => {
                debug!("leaving char: {current_char}");
                // Spaces between a separator and its number are allowed,
                // e.g. `CuSO4 · 5H2O`.
                if !(current_char.is_whitespace() && matches!(previous, Previous::Separator)) {
                    previous = Previous::Other;
                }
                cursor_index += 1;
            }
        }
    }

    // The last part.
    multiply(&mut atoms[part_start..], part_multiplier);

    // A block not closed.
    if !open_blocks.is_empty() {
        return Err(ToEmpiricalFormulaError::UnbalancedParenthesis);
    }

    // Adding the counts of the same atoms.
    let mut atom_count_map: HashMap<String, usize> = HashMap::new();
    for (atom, count) in atoms {
        let total = atom_count_map.entry(atom).or_insert(0);
        *total = total.saturating_add(count);
    }

    debug!("{atom_count_map:#?}");

    // Building empirical formula.
    // C, H and then in alphabetical order.
    let mut sorted_atoms: Vec<(String, usize)> = Vec::new();
    for first_atom in ["C", "H"] {
        if let Some(count) = atom_count_map.remove(first_atom) {
            sorted_atoms.push((first_atom.to_string(), count));
        }
    }
    let mut other_atoms: Vec<(String, usize)> = atom_count_map.into_iter().collect();
    other_atoms.sort();
    sorted_atoms.extend(other_atoms);

    let mut final_formula = String::new();
    for (atom, count) in sorted_atoms {
        final_formula.push_str(&atom);
        if count != 1 {
            final_formula.push_str(&count.to_string());
        }
    }

    debug!("final_formula: {final_formula}");

    Ok(final_formula)
}

#[cfg(test)]
#[path = "formula_tests.rs"]
mod formula_tests;
