use std::fmt::{Display, Formatter};

use log::debug;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum CeNumberError {
    DigitGroupsCaptureError,
    CharTodigitConversionerror(char),
    NoCheckDigitFound,
    CheckDigitDoesNotMatch,
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
        }
    }
}

impl std::error::Error for CeNumberError {}

/// <https://en.wikipedia.org/wiki/European_Community_number>
/// Check if a string is a valid European Community number.
pub fn is_ce_number(number: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Build regex.
    let ce_number_re =
        Regex::new(r"^(?P<group1>[0-9]{3})-(?P<group2>[0-9]{3})-(?P<checkdigit>[0-9]{1})$")
            .unwrap();
    let all_zeros_re = Regex::new(r"^0+$").unwrap();

    // Capture groups and check number.
    let Some(captures) = ce_number_re.captures(number) else {
        return Err(Box::new(CeNumberError::DigitGroupsCaptureError));
    };

    let group1 = &captures["group1"];
    let group2 = &captures["group2"];
    let checkdigit_char = &captures["checkdigit"];
    debug!("group1:{group1} - group2:{group2} - checkdigit_char:{checkdigit_char}");

    if all_zeros_re.is_match(group1) && all_zeros_re.is_match(group2) {
        return Err(Box::new(CeNumberError::AllZeros));
    }

    // Multiplier that will increase at each operation.
    let mut multiplier = 1;
    // Total sum of each operation.
    let mut total = 0;

    // Processing group1.
    for digit_char in group1.chars() {
        let Some(digit) = digit_char.to_digit(10) else {
            return Err(Box::new(CeNumberError::CharTodigitConversionerror(
                digit_char,
            )));
        };
        total += multiplier * digit;
        multiplier += 1;
    }

    // Processing group2.
    for digit_char in group2.chars() {
        let Some(digit) = digit_char.to_digit(10) else {
            return Err(Box::new(CeNumberError::CharTodigitConversionerror(
                digit_char,
            )));
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
            return Err(Box::new(CeNumberError::CharTodigitConversionerror(
                digit_char,
            )));
        };

        if digit.eq(&modulo) {
            Ok(())
        } else {
            Err(Box::new(CeNumberError::CheckDigitDoesNotMatch))
        }
    } else {
        Err(Box::new(CeNumberError::NoCheckDigitFound))
    }
}

#[cfg(test)]
#[path = "cenumber_tests.rs"]
mod cenumber_tests;
