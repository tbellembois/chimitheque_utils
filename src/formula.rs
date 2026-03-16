use log::debug;
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter},
    num::ParseIntError,
};

#[derive(Debug, PartialEq, Eq)]
pub enum SortEmpiricalFormulaError {
    UnbalancedParenthesis,
    UnknowAtom(String),
    CanNotParseNumber(ParseIntError),
    NumberAfterUnknowAtom,
    UnexpectedNoneAtomCount(String),
}

impl Display for SortEmpiricalFormulaError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            SortEmpiricalFormulaError::UnbalancedParenthesis => write!(f, "unbalanced parenthesis"),
            SortEmpiricalFormulaError::UnknowAtom(s) => write!(f, "unknown atom {s}"),
            SortEmpiricalFormulaError::CanNotParseNumber(e) => {
                write!(f, "can not parse number: {e}")
            }
            SortEmpiricalFormulaError::NumberAfterUnknowAtom => {
                write!(f, "found a number after no known atom")
            }
            SortEmpiricalFormulaError::UnexpectedNoneAtomCount(s) => {
                write!(f, "unexpected empty atom_count_map value for key {s}")
            }
        }
    }
}

impl std::error::Error for SortEmpiricalFormulaError {}

/// Sorts the empirical formula from a string.
/// Sort order: C and H atoms then the others in alphabetical order.
/// Example of parsing method:
/// Cl(CaC2(NaCl)3)2.Na=P
/// ^^. .. . . .. . .      Cl c=1 d=0
///   ^ .. . . .. . .      depth=1
///    ^^. . . .. . .      Ca c=1 d=1
///      ^ . . .. . .      C  c=2 d=1
///        ^ . .. . .      depth=2
///         ^^ .. . .      Na c=1 d=2
///           ^^. . .      Cl c=1 d=2
///             ^ . .      for each d>=2 multiply atom by 3; (Na c=3 Cl c=3) depth=1
///               ^ .      for each d>=1 multiply atom by 2; (Na c=6 Cl c=6 ; Ca=2 C=2) depth=0
///                 ^      forget any other char
pub fn sort_empirical_formula(formula: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    // A struct to store the atom count and parenthesis depth while parsing the formula.
    #[derive(Debug)]
    struct AtomBlock {
        atom_name: String,
        parenthesis_depth: isize, // use isize to avoid conversions.
        count: usize,
    }

    let periodic_table = HashMap::from([
        ("Ac", "actinium"),
        ("Ag", "silver"),
        ("Al", "aluminium"),
        ("Am", "americium"),
        ("Ar", "argon"),
        ("As", "arsenic"),
        ("At", "astatine"),
        ("Au", "gold"),
        ("B", "boron"),
        ("Ba", "barium"),
        ("Be", "berylium"),
        ("Bh", "bohrium"),
        ("Bi", "bismuth"),
        ("Bk", "berkelium"),
        ("Br", "bromine"),
        ("C", "carbon"),
        ("Ca", "calcium"),
        ("Cd", "cadmium"),
        ("Ce", "cerium"),
        ("Cf", "californium"),
        ("Cl", "chlorine"),
        ("Cm", "curium"),
        ("Cn", "copemicium"),
        ("Co", "cobalt"),
        ("Cr", "chromium"),
        ("Cs", "caesium"),
        ("Cu", "copper"),
        ("D", "deuterium"),
        ("Db", "dubnium"),
        ("Ds", "darmstadtium"),
        ("Dy", "dysprosium"),
        ("Er", "erbium"),
        ("Es", "einsteinium"),
        ("Eu", "europium"),
        ("F", "fluorine"),
        ("Fe", "iron"),
        ("Fm", "fermium"),
        ("Fr", "francium"),
        ("Ga", "gallium"),
        ("Gd", "gadolinium"),
        ("Ge", "germanium"),
        ("H", "hydrogen"),
        ("He", "helium"),
        ("Hf", "hafnium"),
        ("Hg", "mercury"),
        ("Ho", "holmium"),
        ("Hs", "hassium"),
        ("I", "iodine"),
        ("In", "indium"),
        ("Ir", "iridium"),
        ("K", "potassium"),
        ("Kr", "krypton"),
        ("La", "lanthanum"),
        ("Li", "lithium"),
        ("Lr", "lawrencium"),
        ("Lu", "lutetium"),
        ("Md", "mendelevium"),
        ("Mg", "magnesium"),
        ("Mn", "manganese"),
        ("Mo", "molybdenum"),
        ("Mt", "meitnerium"),
        ("N", "nitrogen"),
        ("Na", "sodium"),
        ("Nb", "niobium"),
        ("Nd", "neodymium"),
        ("Ne", "neon"),
        ("Ni", "nickel"),
        ("No", "nobelium"),
        ("Np", "neptunium"),
        ("O", "oxygen"),
        ("Os", "osmium"),
        ("P", "phosphorus"),
        ("Pa", "protactinium"),
        ("Pb", "lead"),
        ("Pd", "palladium"),
        ("Pm", "promethium"),
        ("Po", "polonium"),
        ("Pr", "praseodymium"),
        ("Pt", "platinium"),
        ("Pu", "plutonium"),
        ("Ra", "radium"),
        ("Rb", "rubidium"),
        ("Re", "rhenium"),
        ("Rf", "rutherfordium"),
        ("Rg", "roentgenium"),
        ("Rh", "rhodium"),
        ("Rn", "radon"),
        ("Ru", "ruthenium"),
        ("S", "sulfure"),
        ("Sb", "antimony"),
        ("Sc", "scandium"),
        ("Se", "sefeniuo"),
        ("Sg", "seaborgium"),
        ("Si", "silicon"),
        ("Sm", "samarium"),
        ("Sn", "tin"),
        ("Sr", "strontium"),
        ("Ta", "tantalum"),
        ("Tb", "terbium"),
        ("Tc", "technetium"),
        ("Te", "tellurium"),
        ("Th", "thorium"),
        ("Ti", "titanium"),
        ("Tl", "thallium"),
        ("Tm", "thulium"),
        ("U", "uranium"),
        ("V", "vanadium"),
        ("W", "tungsten"),
        ("Xe", "xenon"),
        ("Y", "yltrium"),
        ("Yb", "ytterbium"),
        ("Zn", "zinc"),
        ("Zr", "zirconium"),
    ]);

    // Creating a vec from input for parsing.
    let formula_vec: Vec<char> = formula.chars().collect();

    let mut atom_blocks: Vec<AtomBlock> = Vec::new();

    // Cursor index while parsing the formula.
    let mut cursor_index = 0;
    // Current parenthesis depth.
    let mut parenthesis_depth: isize = 0;
    // Char under cursor.
    let mut current_char: char;
    // Current char at the previous loop
    let mut previous_char: Option<char> = None;
    // Possible char after current char.
    let mut maybe_next_char: Option<char>;

    // Parsing the formula.
    while cursor_index < formula_vec.len() {
        // Reading the first char under the cursor.
        current_char = formula_vec[cursor_index];

        // And the next if we are not at the end of the formula.
        if cursor_index < formula_vec.len() - 1 {
            maybe_next_char = Some(formula_vec[cursor_index + 1]);
        } else {
            maybe_next_char = None;
        }

        debug!(
            "current_char: {current_char} maybe_next_char: {:?}",
            maybe_next_char
        );

        match current_char {
            '(' | '[' => {
                // Opening block, increase depth and increment cursor.
                parenthesis_depth += 1;
                cursor_index += 1;
                debug!("parenthesis_depth: {parenthesis_depth}");
            }
            ')' | ']' => {
                // Closing block, decrease depth and decrement cursor.
                parenthesis_depth -= 1;
                // Check wrong parenthesis number.
                if parenthesis_depth < 0 {
                    return Err(Box::new(SortEmpiricalFormulaError::UnbalancedParenthesis));
                }

                cursor_index += 1;
                debug!("parenthesis_depth: {parenthesis_depth}");
            }
            'A'..='Z' => {
                // Building the atom search string.
                let mut maybe_search_atom: Option<String> = None;

                // Beginning of an atom.
                // 1. Two chars atom?
                if let Some(next_char) = maybe_next_char
                    && next_char.is_ascii_lowercase()
                {
                    maybe_search_atom = Some(format!("{current_char}{next_char}"));
                }
                // 2. One char atom.
                if maybe_search_atom.is_none() {
                    maybe_search_atom = Some(format!("{current_char}"));
                }

                // We can unwrap safely.
                let search_atom = maybe_search_atom.unwrap();

                // Does the atom exists?
                if periodic_table.contains_key(&search_atom.as_str()) {
                    atom_blocks.push(AtomBlock {
                        atom_name: search_atom.clone(),
                        parenthesis_depth,
                        count: 1,
                    });
                    debug!("found atom: {search_atom}");
                } else {
                    return Err(Box::new(SortEmpiricalFormulaError::UnknowAtom(search_atom)));
                }

                // Updating the cursor.
                cursor_index += search_atom.len();
            }

            '0'..='9' => {
                // Building the count search string.
                let mut maybe_count_string: Option<String> = None;

                // Beginning a number.
                // 1. two digit number?
                // FIXME: we assume that the number of atoms is < 100.
                if let Some(next_char) = maybe_next_char
                    && next_char.is_ascii_digit()
                {
                    maybe_count_string = Some(format!("{current_char}{next_char}"));
                }

                // One digit number.
                if maybe_count_string.is_none() {
                    maybe_count_string = Some(format!("{current_char}"));
                }

                // We can unwrap safely.
                let count_string = maybe_count_string.unwrap();

                // Converting into usize.
                let count = match count_string.parse::<usize>() {
                    Ok(count) => Some(count),
                    Err(e) => {
                        return Err(Box::new(SortEmpiricalFormulaError::CanNotParseNumber(e)));
                    }
                };
                debug!("count: {:?}", count);

                // The count can be for an atom or a closing parenthesis.
                match previous_char {
                    Some(')' | ']') => {
                        // The count if for a parenthesis block.
                        // For each atom_count with a parenthesis_depth > parenthesis_depth, multiplying the count.
                        for atom in &mut atom_blocks {
                            debug!("atom: {:?}", atom);

                            if atom.parenthesis_depth > parenthesis_depth {
                                atom.count *= count.unwrap();
                                debug!(
                                    "updating atom count for {}: {}",
                                    atom.atom_name, atom.count
                                );
                            }
                        }
                    }
                    Some('a'..='a' | 'A'..='Z') => {
                        // The count is for the last atom of atom_blocks.
                        // Updating the entry.
                        if let Some(last_atom_count) = atom_blocks.last_mut() {
                            last_atom_count.count = count.unwrap();
                        } else {
                            // We have a number after no known atom, this is an error.
                            return Err(Box::new(SortEmpiricalFormulaError::NumberAfterUnknowAtom));
                        }
                    }
                    _ => (),
                }

                // Updating the cursor.
                cursor_index += count_string.len();
            }
            _ => {
                debug!("leaving char: {current_char}");
                cursor_index += 1;
            }
        }

        previous_char = Some(current_char);
    }

    debug!("{:#?}", atom_blocks);

    // Building a map from atom_count.
    let mut atom_count_map: HashMap<String, usize> = HashMap::new();

    for atom_block in &atom_blocks {
        if atom_count_map.contains_key(&atom_block.atom_name) {
            match atom_count_map.get_mut(&atom_block.atom_name) {
                Some(atom_count) => *atom_count += atom_block.count,
                None => {
                    // Should never happen.
                    return Err(Box::new(
                        SortEmpiricalFormulaError::UnexpectedNoneAtomCount(
                            atom_block.atom_name.clone(),
                        ),
                    ));
                }
            }
        } else {
            atom_count_map.insert(atom_block.atom_name.clone(), atom_block.count);
        }
    }

    debug!("{:#?}", atom_count_map);

    // Building empirical formula.
    // C, H and then in alphabetical order.
    let mut final_formula: String = String::new();

    if atom_count_map.contains_key("C") {
        if atom_count_map.get("C").unwrap() == &1 {
            final_formula.push('C');
        } else {
            final_formula.push_str(format!("C{}", atom_count_map.get("C").unwrap()).as_str());
        }

        atom_count_map.remove("C");
    }
    if atom_count_map.contains_key("H") {
        if atom_count_map.get("H").unwrap() == &1 {
            final_formula.push('H');
        } else {
            final_formula.push_str(format!("H{}", atom_count_map.get("H").unwrap()).as_str());
        }
        atom_count_map.remove("H");
    }

    // Sorting the last atoms.
    let mut sorted_keys: Vec<&String> = atom_count_map.keys().collect();
    sorted_keys.sort();

    for key in sorted_keys {
        let (atom_name, atom_count) = atom_count_map.get_key_value(key).unwrap();

        if atom_count == &1 {
            final_formula.push_str(atom_name.clone().as_str());
        } else {
            final_formula.push_str(format!("{atom_name}{atom_count}").as_str());
        }
    }

    debug!("final_formula: {final_formula}");

    Ok(final_formula)
}

#[cfg(test)]
#[path = "formula_tests.rs"]
mod formula_tests;
