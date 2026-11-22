use chimitheque_defines::{ALL_ZERO_RE, CAS_NUMBER_RE};
use log::debug;
use std::fmt::{Display, Formatter};

/// Why a string is not a valid CAS number.
#[derive(Debug, PartialEq)]
pub enum CasNumberError {
    /// The string is empty.
    EmptyCasNumber,
    /// The string is not formatted as a CAS number, e.g. `7732-18-5`.
    DigitGroupsCapture,
    /// A char is not a digit. Not returned: the format check is done before.
    CharTodigitConversion(char),
    /// The check digit is missing. Not returned: the format check is done
    /// before.
    NoCheckDigitFound,
    /// The check digit is not the computed one.
    CheckDigitDoesNotMatch,
    /// All the digits before the check digit are zeros.
    AllZeros,
}

impl Display for CasNumberError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self {
            CasNumberError::DigitGroupsCapture => write!(f, "can not capture digit groups"),
            CasNumberError::CharTodigitConversion(char) => {
                write!(f, "can not convert {char} into digit")
            }
            CasNumberError::NoCheckDigitFound => write!(f, "no check digit found"),
            CasNumberError::CheckDigitDoesNotMatch => write!(f, "check digit does not match"),
            CasNumberError::AllZeros => write!(f, "all zeros"),
            CasNumberError::EmptyCasNumber => write!(f, "empty CAS number"),
        }
    }
}

impl std::error::Error for CasNumberError {}

/// Checks that a string is a valid CAS number: 2 to 7 digits, 2 digits and a
/// check digit separated by dashes, e.g. `7732-18-5`.
///
/// See <https://en.wikipedia.org/wiki/CAS_Registry_Number>.
pub fn is_cas_number(number: &str) -> Result<(), CasNumberError> {
    // Rejecting empty numbers.
    if number.is_empty() {
        return Err(CasNumberError::EmptyCasNumber);
    }

    // Capture groups and check number.
    let Some(captures) = CAS_NUMBER_RE.captures(number) else {
        return Err(CasNumberError::DigitGroupsCapture);
    };

    let group1 = &captures["group1"];
    let group2 = &captures["group2"];
    let checkdigit_char = &captures["checkdigit"];
    debug!("group1:{group1} - group2:{group2} - checkdigit_char:{checkdigit_char}");

    if ALL_ZERO_RE.is_match(group1) && ALL_ZERO_RE.is_match(group2) {
        return Err(CasNumberError::AllZeros);
    }

    // Multiplier that will increase at each operation.
    let mut multiplier = 1;
    // Total sum of each operation.
    let mut total = 0;

    let group2_reversed: String = group2.chars().rev().collect();

    // Processing group2.
    for digit_char in group2_reversed.chars() {
        let Some(digit) = digit_char.to_digit(10) else {
            return Err(CasNumberError::CharTodigitConversion(digit_char));
        };
        total += multiplier * digit;
        multiplier += 1;
    }

    let group1_reversed: String = group1.chars().rev().collect();

    // Processing group1.
    for digit_char in group1_reversed.chars() {
        let Some(digit) = digit_char.to_digit(10) else {
            return Err(CasNumberError::CharTodigitConversion(digit_char));
        };
        total += multiplier * digit;
        multiplier += 1;
    }

    // Calculating modulo.
    let modulo = total % 10;
    debug!("modulo:{modulo}");

    // Processing checkdigit.
    if let Some(digit_char) = checkdigit_char.chars().next() {
        let Some(digit) = digit_char.to_digit(10) else {
            return Err(CasNumberError::CharTodigitConversion(digit_char));
        };

        debug!("digit:{digit}");

        if digit.eq(&modulo) {
            Ok(())
        } else {
            Err(CasNumberError::CheckDigitDoesNotMatch)
        }
    } else {
        Err(CasNumberError::NoCheckDigitFound)
    }
}

#[cfg(test)]
#[path = "casnumber_tests.rs"]
mod casnumber_tests;
