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
            SortEmpiricalFormulaError::CanNotParseNumber(ref e) => e.fmt(f),
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
                    Err(e) => {
                        return Err(Box::new(SortEmpiricalFormulaError::CanNotParseNumber(e)))
                    }
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
                                return Err(Box::new(
                                    SortEmpiricalFormulaError::NumberAfterUnknowAtom,
                                ));
                            }
                        };

                        last_atom_count.count = count.unwrap();
                    }
                    None => (),
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

    for atom_block in atom_blocks.iter() {
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

    use std::vec;

    use log::info;

    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_sort_empirical_formula() {
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
            assert_eq!(
                sort_empirical_formula(formula).unwrap(),
                formula.to_string()
            );
        }

        let linear_formulas = vec![
            "C6H5CH=CHCHO",
            "CH3(CH2)5CH3",
            "H2N(CH2)4CH(NH2)CO2H",
            "CH3CH2CH2Br",
            "CH3OH",
            "C6H5CH=CH2",
            "CH3CO2H",
            "CH3COO(CH2)3CH3",
            "CH3N(NO)C(=NH)NHNO2",
            "(CH3)2CHCH2COOH",
            "CH3C6H4OCH3",
            "C6H5CH=CHBr",
            "HCHO",
            "2-(HO)C6H4CHO",
            "CH3CH=CHC6H4OCH3",
            "C6H5CH2P(O)(OC2H5)2",
            "(CH3O)C6H4OH",
            "CH3(CH2)14CH3",
            "CH3COCH2CH2COCH3",
            "C6H5CH2NH2",
            "C2H5C6H4Br",
            "C6H5CN",
            "202-853-6",
            "C6H5CH2Cl",
            "CH3COOCH2C6H5",
            "Cl(CH2)3Br",
            "BrCCl3",
            "CH3CH(OH)CH(OH)CH3",
            "HO(CH2)4OH",
            "ClCH2CH(OH)CH2OH",
            "(CH3)2C=CHCH2CH2CH(CH3)CH2CHO",
            "C6H10(NH2)2",
            "[-CH(OH)CO2C2H5]2",
            "ICH2CH2I",
            "H2NC6H4COCH3",
            "C2H5CH(NH2)CH2OH",
            "[CH3CH2CH(C6H5)CO]2O",
            "(CH3CH=CHCO)2O",
            "(CF3CO)2O",
            "CH3OC6H4NH2",
            "ClCH2CH2OCH2CH2OCH2CH2Cl",
            "CH3(CH2)7Br",
            "CH3C6H4Br",
            "C6H5CH(CH3)Br",
            "BrCH2CH2OH",
            "CH3(CH2)3OCH2CH2OCH2CH2OH",
            "Cl3CCH(OH)2",
            "CH3CONH2",
            "(C6H5)2CHCl",
            "ClCH2CH2OH",
            "CH3CONHC6H4OH",
            "CH3CONHC6H5",
            "(CH3)2C=NOH",
            "Cl(CH2)3COCH3",
            "Ni(C5H7O2)2",
            "2-(CH3CO2)C6H4CO2H",
            "CH3CO2CH(C6H5)CO2H",
            "CH3CONHC6H4CO2H",
            "HOOC(CH2)4COOH",
            "NH2CH2COOH",
            "NH2SO3H",
            "H2NC10H6SO3H",
            "NH2(CH2)10CO2H",
            "CH3OC6H4CO2H",
            "HO2CCH2CH(NH2)CO2H",
            "HOOCCH2CH(NH2)COOH",
            "(CH3)3SiCHN2",
            "CH3CH(OH)CH2OH",
            "2-(H2N)C6H4CO2H",
            "H2NC6H4CO2H",
            "C6H5CONHOH",
            "C6H5COOH",
            "C6H5COC6H4CO2H",
            "BrCH2COOH",
            "BrC6H4CO2H",
            "CH3C6H4OH",
            "BrCH2CH(Br)CH2OH",
            "NC(CH2)4CN",
            "HN(CH2CH2OH)2",
            "(C2H5)2NC6H5",
            "C2H5OCO(CH2)4COOC2H5",
            "(C2H5O)2P(O)Cl",
            "C2H5OCOCH=CHCOOC2H5",
            "(HOCH2CH2)2O",
            "C2H5OCOCOOC2H5",
            "C6H4-1,2-(CO2C2H5)2",
            "(C2H5O)2SO2",
            "C6H4(OCH3)2",
            "CH3(CH2)4CH(OH)C≡CH",
            "CH3(CH2)12CH3",
            "(CH3)2C6H3NH2",
            "C6H5N(CH3)2",
            "CH3OCOCH=CHCOOCH3",
            "C6H4-1,2-(CO2CH3)2",
            "(O2N)2C6H3F",
            "2-(HO)C6H4CO2CH3",
            "(C6H5)2O",
            "(CH3)2SO",
            "(C6H5)2CH2",
            "CH3OCH2CH2OCH3",
            "(CH3OCH2CH2)2O",
            "CH3O(CH2CH2O)3CH3",
            "C2H5OCH2CH2OH",
            "CH3COCH(CH2C6H5)CO2C2H5",
            "CH3COCH2COOC2H5",
            "C6H5COOC2H5",
            "ClCH2CO2C2H5",
            "C6H5CH=CHCOOC2H5",
            "CH3CH=CHCH=CHCO2C2H5",
            "HCONH2",
            "(CH3)2C=CHCH2CH2C(CH3)=CHCH2OH",
            "HOCH2CH(OH)CH2OH",
            "(C6H5CH2)2CO",
            "(CH3)2C(OH)CH2COCH3",
            "CH3OC6H3(CH=CHCH3)OH",
            "(CH3)2C=CHCH2OH",
            "C6H5CH2COCH3",
            "CH3C6H9(=O)",
            "C6H5COOCH3",
            "CH3(CH2)5CH=CHCO2CH3",
            "4-(HO)C6H3-3-(OCH3)CHO",
            "HOC6H4COCH3",
            "C6H5NHCH3",
            "CH3C6H10OH",
            "HCONHCH3",
            "CH3(CH2)5C≡CCO2CH3",
            "CH3(CH2)4CH(OH)CH=CH2",
            "CH3N(CH2CH2OH)2",
            "CH3C6H4NO2",
            "CH3(CH2)5COCH3",
            "CH3C6H4SO3CH3",
            "C10H7CHO",
            "HO(CH2)5OH",
            "C6H5CH2COCl",
            "CH3COOC6H5",
            "C6H5Si(CH3)3",
            "GdCl3.6H2O",
            "HO(CH2)3OH",
            "CCl2=CCl2",
            "(CH3)2NC(=NH)N(CH3)2",
            "CH3C6H4NH2",
            "[(C6H5)2PC10H6-]2",
            "(C2H5O)2P(O)CH2CO2C2H5",
            "NH2CH2COOCH3 · HCl",
            "(CH3)2NCH2CH2CN",
            "Cl3CCH3",
            "C6H5CO2CH2CH2CH3",
            "NH2(CH2)4NH2",
            "C6H5NHNH2",
            "Br(CH2)4Br",
            "(CH3(CH2)3O)3PO",
            "HO(CH2CH2O)2CH2CH2OH",
            "NH2CH2CH2(NHCH2CH2)2NH2",
            "(C2H5O)3PO",
            "(CH3)3C6H2COCH3",
            "C6H5CO(CH2)3CH3",
            "CH3(CH2)10OH",
            "Br2CHCHBr2",
            "CF3COOH",
            "C6H5OCH2CH=CH2",
            "C6H5CH2Br",
            "BrC6H4NH2",
            "BrCH2C6H4CO2H",
            "BrC6H4OCH3",
            "Br(CH2)10COOH",
            "(CH3)2C=CHCH2CH2CH(CH3)CH2CH2OH",
            "CH3(CH2)3CH(CO2H)2",
            "CH3(CH2)3C6H4N=CHC6H4OCH3",
            "C10H16O4S",
            "CH3(CH2)8COOH",
            "(HOCH2CH2)3N",
            "ClCH2COOH",
            "(CH3)3SiO[(CH3)HSiO]nSi(CH3)3",
            "[CH3C6H4CO2CH(CO2H)-]2",
            "CH3(CH2)11N(CH3)3Br",
            "(CH3)3SiNHSi(CH3)3",
            "(CH3CH2CH2CH2)4N(PF6)",
            "C8 H17 N3 . H Cl",
            "HOC10H6C10H6OH",
            "ClC6H4CO2H",
            "[-CH(OH)CO2CH3]2",
            "[C6H5CO2CH(CO2H)]2",
            "C6H5CH2CH(NH2)COOH",
            "H2NC10H6C10H6NH2",
            "CF3COCH2COCF3",
            "C10H15BrO4S · NH3",
            "BrCH2CH2Br",
            "HOC(COOH)(CH2COOH)2",
            "C6H5CH2CH(NH2)COOC2H5 · HCl",
            "C6H11N=C=NC6H11",
            "C6H5CH2CH(NH2)COOCH3 · HC",
            "C6H5CH2CH(NH2)COOCH3 · HCl",
            "C6H5CH2CH(NH2)CO2H",
            "C6H5CO2C6H5",
            "C10H13ClO3",
            "CHCl2CHCl2",
            "C6H5CH(OH)CH(CH3)NH2",
            "C6H5CH[CH(NHCH3)CH3]OH · 1/2H2O",
            "[CH3(CH2)5]4N(HSO4)",
            "(CH3CH2CH2CH2)4N(HSO4)",
            "CH3(CH2)3CH3",
            "(CH3CH2)2O",
            "C6H5CH3",
            "(CH3)3N(Br3)C6H5",
            "C21H14N3NaO3S3",
            "Li[CH(CH3)CH2CH3]3BH",
            "(CH3)2CHCH(NH2)CO2H",
            "NH2CH2CH2SO3H",
            "C5H5N·ClCrO3H",
            "C6H5CH(OH)CH(NH2)CH2OH",
            "[CH2=C(CH3)CH2PdCl]2",
            "C6H5CH2N(CH3)2",
            "(CH3)2CHCH2CH2OH",
            "CH3COCH3",
            "CH3COOC2H5",
            "CH2OCH2O",
            "(CH3CO2)2Zn",
            "C6H11OH",
            "[-CH2OCH2CH2N(CH2CO2H)2]2",
            "HOC6H4CO2CH3",
            "C15H10O7 · xH2O",
            "(CH3)2NCH2CH2N(CH3)2",
            "S(CH2CH2OH)2",
            "NH2C(CH2OH)3",
        ];

        for formula in linear_formulas {
            let maybe_empirical_formula = sort_empirical_formula(formula);
            info!("{formula} -> {:?}", maybe_empirical_formula);

            assert!(maybe_empirical_formula.is_ok());
        }
    }
}
