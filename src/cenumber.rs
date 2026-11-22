use chimitheque_defines::{ALL_ZERO_RE, CE_NUMBER_RE};
use log::debug;
use std::fmt::{Display, Formatter};

/// Why a string is not a valid European Community number.
#[derive(Debug, PartialEq)]
pub enum CeNumberError {
    /// The string is empty.
    EmptyCeNumber,
    /// The string is not formatted as a CE number, e.g. `231-791-2`.
    DigitGroupsCaptureError,
    /// A char is not a digit. Not returned: the format check is done before.
    CharTodigitConversionerror(char),
    /// The check digit is missing. Not returned: the format check is done
    /// before.
    NoCheckDigitFound,
    /// The check digit is not the computed one.
    CheckDigitDoesNotMatch,
    /// All the digits before the check digit are zeros.
    AllZeros,
}

impl Display for CeNumberError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self {
            CeNumberError::DigitGroupsCaptureError => write!(f, "can not capture digit groups"),
            CeNumberError::CharTodigitConversionerror(char) => {
                write!(f, "can not convert {char} into digit")
            }
            CeNumberError::NoCheckDigitFound => write!(f, "no check digit found"),
            CeNumberError::CheckDigitDoesNotMatch => write!(f, "check digit does not match"),
            CeNumberError::AllZeros => write!(f, "all zeros"),
            CeNumberError::EmptyCeNumber => write!(f, "empty CE number"),
        }
    }
}

impl std::error::Error for CeNumberError {}

/// Checks that a string is a valid European Community number: two groups of 3
/// digits and a check digit separated by dashes, e.g. `231-791-2`.
///
/// See <https://en.wikipedia.org/wiki/European_Community_number>.
pub fn is_ce_number(number: &str) -> Result<(), CeNumberError> {
    // Rejecting empty numbers.
    if number.is_empty() {
        return Err(CeNumberError::EmptyCeNumber);
    }

    // Capture groups and check number.
    let Some(captures) = CE_NUMBER_RE.captures(number) else {
        return Err(CeNumberError::DigitGroupsCaptureError);
    };

    let group1 = &captures["group1"];
    let group2 = &captures["group2"];
    let checkdigit_char = &captures["checkdigit"];
    debug!("group1:{group1} - group2:{group2} - checkdigit_char:{checkdigit_char}");

    if ALL_ZERO_RE.is_match(group1) && ALL_ZERO_RE.is_match(group2) {
        return Err(CeNumberError::AllZeros);
    }

    // Multiplier that will increase at each operation.
    let mut multiplier = 1;
    // Total sum of each operation.
    let mut total = 0;

    // Processing group1.
    for digit_char in group1.chars() {
        let Some(digit) = digit_char.to_digit(10) else {
            return Err(CeNumberError::CharTodigitConversionerror(digit_char));
        };
        total += multiplier * digit;
        multiplier += 1;
    }

    // Processing group2.
    for digit_char in group2.chars() {
        let Some(digit) = digit_char.to_digit(10) else {
            return Err(CeNumberError::CharTodigitConversionerror(digit_char));
        };
        total += multiplier * digit;
        multiplier += 1;
    }

    // Calculating modulo.
    let modulo = total % 11;
    debug!("modulo:{modulo}");

    // Processing checkdigit.
    if let Some(digit_char) = checkdigit_char.chars().next() {
        let Some(digit) = digit_char.to_digit(10) else {
            return Err(CeNumberError::CharTodigitConversionerror(digit_char));
        };

        if digit.eq(&modulo) {
            Ok(())
        } else {
            Err(CeNumberError::CheckDigitDoesNotMatch)
        }
    } else {
        Err(CeNumberError::NoCheckDigitFound)
    }
}

#[cfg(test)]
#[path = "cenumber_tests.rs"]
mod cenumber_tests;
