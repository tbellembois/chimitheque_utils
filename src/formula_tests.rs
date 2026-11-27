#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::formula::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_empirical_formulas_are_unchanged() {
        init_logger();

        // Already empirical formulas in the Hill order are returned unchanged.
        let empirical_formulas = [
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
            "ClHO3S",
            "C2H2Cl2O2",
            "BF4H",
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
            "BH3O3",
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
            "ClHO4",
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
            "ClH4N",
            "C2H5NO",
            "C13H11Cl",
            "BaCl2",
            "C2H5ClO",
            "C8H9NO2",
            "CdCl2",
        ];

        for formula in empirical_formulas {
            assert_eq!(to_empirical_formula(formula).unwrap(), formula.to_string());
        }
    }

    #[test]
    fn test_linear_formulas_are_parsed() {
        init_logger();

        // Linear formulas from a product database: they must all be parsed.
        // Their results are checked in `test_linear_formulas`.
        let linear_formulas = [
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
            assert!(to_empirical_formula(formula).is_ok(), "{formula}");
        }
    }

    #[test]
    fn test_linear_formulas() {
        init_logger();

        for (formula, expected) in [
            ("CH3OH", "CH4O"),                       // methanol
            ("HCHO", "CH2O"),                        // formaldehyde
            ("CH3CO2H", "C2H4O2"),                   // acetic acid
            ("CH3CH2CH2Br", "C3H7Br"),               // 1-bromopropane
            ("Cl(CH2)3Br", "C3H6BrCl"),              // 1-bromo-3-chloropropane
            ("CH3(CH2)5CH3", "C7H16"),               // heptane
            ("C6H5CH=CH2", "C8H8"),                  // styrene
            ("C6H5CN", "C7H5N"),                     // benzonitrile
            ("2-(HO)C6H4CHO", "C7H6O2"),             // salicylaldehyde
            ("(CH3)2CHCH2COOH", "C5H10O2"),          // isovaleric acid
            ("H2N(CH2)4CH(NH2)CO2H", "C6H14N2O2"),   // lysine
            ("HOC(COOH)(CH2COOH)2", "C6H8O7"),       // citric acid
            ("[-CH(OH)CO2C2H5]2", "C8H14O6"),        // diethyl tartrate
            ("C6H4-1,2-(CO2CH3)2", "C10H10O4"),      // dimethyl phthalate
            ("CH3(CH2)4CH(OH)C≡CH", "C8H14O"),       // 1-octyn-3-ol
            ("C6H11N=C=NC6H11", "C13H22N2"),         // DCC
            ("(CF3CO)2O", "C4F6O3"),                 // trifluoroacetic anhydride
            ("(CH3)3SiCHN2", "C4H10N2Si"),           // TMS-diazomethane
            ("(CH3CO2)2Zn", "C4H6O4Zn"),             // zinc acetate
            ("Ni(C5H7O2)2", "C10H14NiO4"),           // nickel acetylacetonate
            ("CH3(CH2)11N(CH3)3Br", "C15H34BrN"),    // DTAB
            ("(CH3CH2CH2CH2)4N(PF6)", "C16H36F6NP"), // TBAPF6
            ("Li[CH(CH3)CH2CH3]3BH", "C12H28BLi"),   // L-selectride
            ("[CH2=C(CH3)CH2PdCl]2", "C8H14Cl2Pd2"), // methallylpalladium chloride dimer
            ("NH2SO3H", "H3NO3S"),                   // sulfamic acid
            ("GdCl3.6H2O", "Cl3GdH12O6"),            // gadolinium chloride hexahydrate
            ("NH2CH2COOCH3 · HCl", "C3H8ClNO2"),     // glycine methyl ester hydrochloride
            ("C5H5N·ClCrO3H", "C5H6ClCrNO3"),        // pyridinium chlorochromate
            ("C8 H17 N3 . H Cl", "C8H18ClN3"),       // EDC hydrochloride
        ] {
            assert_eq!(
                to_empirical_formula(formula).unwrap(),
                expected,
                "{formula}"
            );
        }
    }

    #[test]
    fn test_two_letter_atoms() {
        init_logger();

        // A lowercase letter after an uppercase one makes a two-letter atom.
        for (formula, expected) in [
            ("Co", "Co"),
            ("CO", "CO"),
            ("CoCl2", "Cl2Co"),
            ("NaCl", "ClNa"),
            ("CHCl3", "CHCl3"),
        ] {
            assert_eq!(
                to_empirical_formula(formula).unwrap(),
                expected,
                "{formula}"
            );
        }
    }

    #[test]
    fn test_ignored_chars() {
        init_logger();

        // Bonds, dashes, commas, spaces and lowercase letters are ignored, and
        // so is a number after them.
        for (formula, expected) in [
            ("CH2=CH2", "C2H4"),
            ("HC≡CH", "C2H2"),
            ("2-(HO)C6H4CHO", "C7H6O2"),
            ("C6H4-1,2-(CO2CH3)2", "C10H10O4"),
            ("C8 H17 N3", "C8H17N3"),
            ("0H2O", "H2O"),
        ] {
            assert_eq!(
                to_empirical_formula(formula).unwrap(),
                expected,
                "{formula}"
            );
        }
    }

    #[test]
    fn test_formula_without_atom() {
        init_logger();

        // Only ignored chars: the empirical formula is empty.
        for formula in ["()", "[]", ".", "2", "x", "co", "-"] {
            assert_eq!(to_empirical_formula(formula).unwrap(), "", "{formula}");
        }
    }

    #[test]
    fn test_errors() {
        init_logger();

        assert_eq!(
            to_empirical_formula(""),
            Err(ToEmpiricalFormulaError::EmptyFormula)
        );

        for (formula, atom) in [("Xy", "Xy"), ("CXy", "Xy"), ("CH3Qq", "Qq"), ("J", "J")] {
            assert_eq!(
                to_empirical_formula(formula),
                Err(ToEmpiricalFormulaError::UnknownAtom(atom.to_string())),
                "{formula}"
            );
        }

        for formula in [
            "(CH3", "C[H3", "((CH3)2", "CH3)", "H2O)(", "(CH3]2", "[CH3)2", "([CH3)]2",
        ] {
            assert_eq!(
                to_empirical_formula(formula),
                Err(ToEmpiricalFormulaError::UnbalancedParenthesis),
                "{formula}"
            );
        }

        assert!(matches!(
            to_empirical_formula("C99999999999999999999999"),
            Err(ToEmpiricalFormulaError::CanNotParseNumber(_))
        ));

        // A count or a multiplier of 0.
        for formula in [
            "C0H4",
            "C00",
            "H2O0",
            "(H2O)0",
            "CuSO4.0H2O",
            "CuSO4 · 0H2O",
        ] {
            assert_eq!(
                to_empirical_formula(formula),
                Err(ToEmpiricalFormulaError::ZeroCount),
                "{formula}"
            );
        }
    }

    #[test]
    fn test_display() {
        for (error, message) in [
            (
                ToEmpiricalFormulaError::UnbalancedParenthesis,
                "unbalanced parenthesis",
            ),
            (
                ToEmpiricalFormulaError::UnknownAtom("Xy".to_string()),
                "unknown atom Xy",
            ),
            (
                ToEmpiricalFormulaError::CanNotParseNumber("abc".parse::<usize>().unwrap_err()),
                "can not parse number: invalid digit found in string",
            ),
            (
                ToEmpiricalFormulaError::NumberAfterUnknownAtom,
                "found a number after no known atom",
            ),
            (
                ToEmpiricalFormulaError::UnexpectedNoneAtomCount("Xy".to_string()),
                "unexpected empty atom_count_map value for key Xy",
            ),
            (ToEmpiricalFormulaError::EmptyFormula, "empty formula"),
            (ToEmpiricalFormulaError::ZeroCount, "count of 0"),
        ] {
            assert_eq!(error.to_string(), message);
        }
    }

    #[test]
    fn test_block_multiplier_applies_to_its_block_only() {
        init_logger();

        assert_eq!(to_empirical_formula("(CH3)2(OH)3").unwrap(), "C2H9O3");
        assert_eq!(
            to_empirical_formula("Cl(CaC2(NaCl)3)2").unwrap(),
            "C4Ca2Cl7Na6"
        );
        assert_eq!(to_empirical_formula("[(CH3)2N]2CO").unwrap(), "C5H12N2O");
    }

    #[test]
    fn test_counts_of_more_than_two_digits() {
        init_logger();

        assert_eq!(to_empirical_formula("C100H202").unwrap(), "C100H202");
        assert_eq!(
            to_empirical_formula("C254H377N65O75S6").unwrap(),
            "C254H377N65O75S6"
        );
        assert_eq!(to_empirical_formula("(CH2)120").unwrap(), "C120H240");
    }

    #[test]
    fn test_part_multiplier() {
        init_logger();

        for (formula, expected) in [
            ("CuSO4.5H2O", "CuH10O9S"),
            ("CuSO4·5H2O", "CuH10O9S"),
            ("CuSO4 · 5H2O", "CuH10O9S"),
            ("CaCl2.2H2O.2NH3", "CaCl2H10N2O2"),
            // A part without a number counts once; a number after a letter is ignored.
            ("CuSO4.H2O", "CuH2O5S"),
            ("C15H10O7 · xH2O", "C15H12O8"),
            ("CuSO4. x5H2O", "CuH2O5S"),
        ] {
            assert_eq!(
                to_empirical_formula(formula).unwrap(),
                expected,
                "{formula}"
            );
        }
    }

    #[test]
    fn test_hill_order() {
        init_logger();

        for (formula, expected) in [
            // With C: C, H, then alphabetical.
            ("CH3CH2OH", "C2H6O"),
            ("NaCH3COO", "C2H3NaO2"),
            ("CCl4", "CCl4"),
            // Without C: all alphabetical.
            ("Ca(OH)2", "CaH2O2"),
            ("NaCl", "ClNa"),
            ("H2O", "H2O"),
            ("H2SO4", "H2O4S"),
            ("NH4Cl", "ClH4N"),
        ] {
            assert_eq!(
                to_empirical_formula(formula).unwrap(),
                expected,
                "{formula}"
            );
        }
    }
}
