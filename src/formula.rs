use log::debug;
use std::collections::HashMap;

// Cl(CaC2(NaCl)3)2.Na=P
// ^^. .. . . .. . .      Cl c=1 d=0
//   ^ .. . . .. . .      depth=1
//    ^^. . . .. . .      Ca c=1 d=1
//      ^ . . .. . .      C  c=2 d=1
//        ^ . .. . .      depth=2
//         ^^ .. . .      Na c=1 d=2
//           ^^. . .      Cl c=1 d=2
//             ^ . .      for each d>=2 multiply atom by 3; (Na c=3 Cl c=3) depth=1
//               ^ .      for each d>=1 multiply atom by 2; (Na c=6 Cl c=6 ; Ca=2 C=2) depth=0
//                 ^      forget any other char
//
// note: if a multiplier is not a digit return an error (can not convert)
// Returns the empirical formula from "formula".
// Only operates on basic formulas.
pub fn empirical_formula(formula: &str) -> Result<String, String> {
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

    // A struct to store the atom count and parenthesis depth while parsing the formula.
    #[derive(Debug)]
    struct AtomBlock {
        atom_name: String,
        parenthesis_depth: isize, // use isize to avoid conversions.
        count: usize,
    }
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
                    return Err("unbalanced parenthesis".to_string());
                }

                cursor_index += 1;
                debug!("parenthesis_depth: {parenthesis_depth}");
            }
            'A'..='Z' => {
                // Building the atom search string.
                let mut maybe_search_atom: Option<String> = None;

                // Beginning of an atom.
                // 1. Two chars atom?
                if let Some(next_char) = maybe_next_char {
                    if let 'a'..='z' = next_char {
                        maybe_search_atom = Some(format!("{current_char}{next_char}"));
                    }
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
                        atom_name: search_atom.to_string(),
                        parenthesis_depth,
                        count: 1,
                    });
                    debug!("found atom: {search_atom}");
                } else {
                    return Err("unknown atom:{search}".to_string());
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
                if let Some(next_char) = maybe_next_char {
                    if let '0'..='9' = next_char {
                        maybe_count_string = Some(format!("{current_char}{next_char}"));
                    }
                }

                // One digit number.
                if maybe_count_string.is_none() {
                    maybe_count_string = Some(format!("{}", current_char));
                }

                // We can unwrap safely.
                let count_string = maybe_count_string.unwrap();

                // Converting into usize.
                let count = match count_string.parse::<usize>() {
                    Ok(count) => Some(count),
                    Err(e) => return Err(format!("can not convert {count_string}: {e}")),
                };
                debug!("count: {:?}", count);

                // The count can be for an atom or a closing parenthesis.
                match previous_char {
                    Some(')') | Some(']') => {
                        // The count if for a parenthesis block.
                        // For each atom_count with a parenthesis_depth > parenthesis_depth, multiplying the count.
                        for atom in atom_blocks.iter_mut() {
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
                    Some('a'..='a') | Some('A'..='Z') => {
                        // The count is for the last atom of atom_blocks.
                        // Updating the entry.
                        let last_atom_count = match atom_blocks.last_mut() {
                            Some(last_atom_count) => last_atom_count,
                            None => {
                                // We have a number after no known atom, this is an error.
                                return Err("found a number after no known atom".to_string());
                            }
                        };

                        last_atom_count.count = count.unwrap();
                    }
                    None => return Err("found a number with no char before".to_string()),
                    _ => {
                        // We have a number after no known char, this is an error.
                        return Err("found a number after no known char".to_string());
                    }
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

    for atom_block in atom_blocks.iter() {
        if atom_count_map.contains_key(&atom_block.atom_name) {
            match atom_count_map.get_mut(&atom_block.atom_name) {
                Some(atom_count) => *atom_count += atom_block.count,
                None => {
                    // Should never happen.
                    return Err(format!(
                        "unexpected empty atom_count_map value for key {}",
                        atom_block.atom_name
                    ));
                }
            };
        } else {
            atom_count_map.insert(atom_block.atom_name.clone(), atom_block.count);
        }
    }

    debug!("{:#?}", atom_count_map);

    // Building empirical formula.
    // C, H and then in alphabetical order.
    let mut final_formula: String = "".to_string();

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
            final_formula.push_str(atom_name.to_string().as_str());
        } else {
            final_formula.push_str(format!("{atom_name}{atom_count}").as_str());
        }
    }

    debug!("final_formula: {final_formula}");

    Ok(final_formula)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_empirical_formula() {
        init_logger();

        let empirical_formulas = vec![
            "C5H11Br",
            "C7H6O3",
            "C19H10Br4O5S",
            "C9H8O",
            "CH2Cl2",
            "C2H8N2",
            "C20H12O5",
            "C7H16",
            "C10H18O",
            "C6H14N2O2",
            "C8H8N6O6",
            "C10H8O",
            "C10H16O",
            "C22H23N3O9",
            "C6H12",
            "C10H15NO",
            "C3H7Br",
            "CH4O",
            "C2H4O2",
            "C6H12O2",
            "C6H12N2",
            "C2H5N5O3",
            "C29H37IN2O2",
            "C8H8O",
            "CH2O2",
            "CH4O3S",
            "C5H10O2",
            "C4H8O2",
            "C7H12O4",
            "C6H8O3",
            "HClO3S",
            "C2H2Cl2O2",
            "HBF4",
            "C3H6O2",
            "C7H8O",
            "C6H12O7",
            "C8H18O",
            "C8H8O2",
            "C7H6O",
            "C10H14N4O2",
            "C11H12O3",
            "C3H6O3",
            "C2H4O2S",
            "C8H16O2",
            "C18H34O2",
            "C5H6O2",
            "C2H4N4",
            "C7H16O",
            "C25H54ClN",
            "C6H5I",
            "CH3I",
            "C3H2N2",
            "C9H10O",
            "C8H10O",
            "C10H12O2",
            "C62H87N13O16",
            "C13H10N2",
            "CH2O",
            "C6H10O2",
            "C2H2ClN",
            "C10H18",
            "C2H3LiO2",
            "C7H6O2",
            "C5H4O2",
            "C10H12O",
            "C11H17O3P",
            "C7H8O2",
            "C16H34",
            "C7H9N",
            "C8H9Br",
            "C7H5N",
            "C6H8O7",
            "H3BO3",
            "C2H7AsO2",
            "C7H7Cl",
            "C9H10O2",
            "C3H6BrCl",
            "CBrCl3",
            "C4H10O2",
            "C2H3N",
            "C2H3NaO2",
            "C2H7NO2",
            "C3H7ClO2",
            "C12H18O",
            "C10H14O",
            "C12H15N3O6",
            "C24H40O4",
            "C8H12N2O3",
            "C6H14N2",
            "C6H11NO4",
            "C4H7NO4",
            "C4H6O5",
            "C8H14O6",
            "C13H17HgNO6",
            "H2MoO4",
            "C2H4I2",
            "C6H5NO2",
            "H3O4P",
            "C2H2O4",
            "C7H7NO2",
            "HClO4",
            "C6H3N3O7",
            "C3H4O3",
            "C20H28O2",
            "C7H7NO",
            "C4H6O4",
            "C8H9NO",
            "H2O4S",
            "C4H11NO",
            "C76H52O46",
            "C20H22O3",
            "C8H10O3",
            "C4F6O3",
            "C7H9NO",
            "C12H11N",
            "C6H12Cl2O2",
            "C6H11Br",
            "C2H5Br",
            "C8H17Br",
            "C10H7Br",
            "C7H7Br",
            "C2HCl3O2",
            "C3H5NO",
            "C62H86N12O16",
            "C10H13N5O4",
            "C17H12O6",
            "C21H28O5",
            "C39H54N10O14S",
            "C13H10N2O",
            "C9H23NO3Si",
            "H3N",
            "C16H19N3O4S",
            "C27H44N10O6",
            "N3Na",
            "C14H18N4O3",
            "C7H8N2",
            "C12H12N2",
            "CH5NO3",
            "Cr2K2O7",
            "C14H12O2",
            "C8H12N4O5",
            "As2O3",
            "C35H58O9",
            "C34H24N6Na4O14S4",
            "C6H10O5",
            "C2H5BrO",
            "C8H18O3",
            "C9H13N",
            "C4H6O2",
            "CCl4",
            "C21H20BrN3",
            "C4H10O",
            "C8H10N4O2",
            "C20H16N2O4",
            "CH8N2O3",
            "CCaO3",
            "CK2O3",
            "CNa2O3",
            "C21H12O7",
            "C2H3Cl3O2",
            "C11H12Cl2N2O5",
            "CHCl3",
            "H4ClN",
            "C2H5NO",
            "C13H11Cl",
            "BaCl2",
            "C2H5ClO",
            "C8H9NO2",
            "CdCl2",
        ];

        for formula in empirical_formulas {
            assert_eq!(empirical_formula(formula), Ok(formula.to_string()));
        }
    }
}
