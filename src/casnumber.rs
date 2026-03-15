use std::fmt::{Display, Formatter};

use log::debug;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum CasNumberError {
    DigitGroupsCapture,
    CharTodigitConversion(char),
    NoCheckDigitFound,
    CheckDigitDoesNotMatch,
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
        }
    }
}

impl std::error::Error for CasNumberError {}

/// <https://en.wikipedia.org/wiki/CAS_Registry_Number>
/// Check if a string is a valid CAS number.
pub fn is_cas_number(number: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Build regex.
    let cas_number_re =
        Regex::new(r"^(?P<group1>[0-9]{2,7})-(?P<group2>[0-9]{2})-(?P<checkdigit>[0-9]{1})$")
            .unwrap();
    let all_zeros_re = Regex::new(r"^0+$").unwrap();

    // Capture groups and check number.
    let Some(captures) = cas_number_re.captures(number) else {
        return Err(Box::new(CasNumberError::DigitGroupsCapture));
    };

    let group1 = &captures["group1"];
    let group2 = &captures["group2"];
    let checkdigit_char = &captures["checkdigit"];
    debug!("group1:{group1} - group2:{group2} - checkdigit_char:{checkdigit_char}");

    if all_zeros_re.is_match(group1) && all_zeros_re.is_match(group2) {
        return Err(Box::new(CasNumberError::AllZeros));
    }

    // Multiplier that will increase at each operation.
    let mut multiplier = 1;
    // Total sum of each operation.
    let mut total = 0;

    let group2_reversed: String = group2.chars().rev().collect();

    // Processing group2.
    for digit_char in group2_reversed.chars() {
        let Some(digit) = digit_char.to_digit(10) else {
            return Err(Box::new(CasNumberError::CharTodigitConversion(digit_char)));
        };
        total += multiplier * digit;
        multiplier += 1;
    }

    let group1_reversed: String = group1.chars().rev().collect();

    // Processing group1.
    for digit_char in group1_reversed.chars() {
        let Some(digit) = digit_char.to_digit(10) else {
            return Err(Box::new(CasNumberError::CharTodigitConversion(digit_char)));
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
            return Err(Box::new(CasNumberError::CharTodigitConversion(digit_char)));
        };

        debug!("digit:{digit}");

        if digit.eq(&modulo) {
            Ok(())
        } else {
            Err(Box::new(CasNumberError::CheckDigitDoesNotMatch))
        }
    } else {
        Err(Box::new(CasNumberError::NoCheckDigitFound))
    }
}

#[cfg(test)]
#[path = "casnumber_tests.rs"]
mod casnumber_tests;
