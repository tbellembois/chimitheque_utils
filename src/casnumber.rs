use std::fmt::{Display, Formatter};

use log::debug;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum CasNumberError {
    DigitGroupsCaptureError,
    CharTodigitConversionerror(char),
    NoCheckDigitFound,
}

impl Display for CasNumberError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self {
            CasNumberError::DigitGroupsCaptureError => write!(f, "can not capture digit groups"),
            CasNumberError::CharTodigitConversionerror(char) => {
                write!(f, "can not convert {char} into digit")
            }
            CasNumberError::NoCheckDigitFound => write!(f, "no check digit found"),
        }
    }
}

impl std::error::Error for CasNumberError {}

/// <https://en.wikipedia.org/wiki/CAS_Registry_Number>
/// Check if a string is a valid CAS number.
pub fn is_cas_number(number: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    // Build regex.
    let re = Regex::new(r"^(?P<group1>[0-9]{2,7})-(?P<group2>[0-9]{2})-(?P<checkdigit>[0-9]{1})$")
        .unwrap();

    // Capture groups and check number.
    let captures = match re.captures(number) {
        Some(captures) => captures,
        None => return Err(Box::new(CasNumberError::DigitGroupsCaptureError)),
    };

    let group1 = &captures["group1"];
    let group2 = &captures["group2"];
    let checkdigit_char = &captures["checkdigit"];
    debug!("group1:{group1} - group2:{group2} - checkdigit_char:{checkdigit_char}");

    // Multiplier that will increase at each operation.
    let mut multiplier = 1;
    // Total sum of each operation.
    let mut total = 0;

    let group2_reversed: String = group2.chars().rev().collect();

    // Processing group2.
    for digit_char in group2_reversed.chars() {
        let digit = match digit_char.to_digit(10) {
            Some(digit) => digit,
            None => {
                return Err(Box::new(CasNumberError::CharTodigitConversionerror(
                    digit_char,
                )))
            }
        };
        total += multiplier * digit;
        multiplier += 1;
    }

    let group1_reversed: String = group1.chars().rev().collect();

    // Processing group1.
    for digit_char in group1_reversed.chars() {
        let digit = match digit_char.to_digit(10) {
            Some(digit) => digit,
            None => {
                return Err(Box::new(CasNumberError::CharTodigitConversionerror(
                    digit_char,
                )))
            }
        };
        total += multiplier * digit;
        multiplier += 1;
    }

    // Calculating modulo.
    let modulo = total % 10;
    debug!("modulo:{modulo}");

    // Processing checkdigit.
    if let Some(digit_char) = checkdigit_char.chars().next() {
        let digit = match digit_char.to_digit(10) {
            Some(digit) => digit,
            None => {
                return Err(Box::new(CasNumberError::CharTodigitConversionerror(
                    digit_char,
                )))
            }
        };

        Ok(digit.eq(&modulo))
    } else {
        Err(Box::new(CasNumberError::NoCheckDigitFound))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use dyn_error::*;
    use log::info;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_is_cas_number_nok() {
        init_logger();

        // Regex test.
        let cas_numbers = vec![
            "0-00-5",        // first group too short
            "00000000-00-5", // first group too long
            "100-0-5",       // second group too short
            "100-000-5",     // second group too long
            "100-000-",      // no check digit
            "ABC-000-5",     // wrong chars
        ];

        for cas_number in cas_numbers {
            info!("processing {cas_number}");
            let result = is_cas_number(cas_number);
            assert_err_box!(result, CasNumberError::DigitGroupsCaptureError);
        }

        // Check digit test.
        assert!(!is_cas_number("100-00-6").unwrap());
        assert!(!is_cas_number("123-45-6").unwrap());
    }

    #[test]
    fn test_is_cas_number_ok() {
        init_logger();

        let cas_numbers = vec![
            "100-00-5",
            "100-40-3",
            "100-42-5",
            "100-44-7",
            "100-63-0",
            "10028-18-9",
            "10039-54-0",
            "10043-35-3",
            "10046-00-1",
            "100683-97-4",
            "100683-98-5",
            "100683-99-6",
            "100684-02-4",
            "100684-03-5",
            "100684-04-6",
            "100684-05-7",
            "100684-33-1",
            "100684-37-5",
            "100684-38-6",
            "100684-49-9",
            "100684-51-3",
            "100801-63-6",
            "100801-65-8",
            "100801-66-9",
            "100988-63-4",
            "101-14-4",
            "101-21-3",
            "101-61-1",
            "101-68-8",
            "101-77-9",
            "101-80-4",
            "101-90-6",
            "10101-96-9",
            "10108-64-2",
            "101205-02-1",
            "10124-36-4",
            "10124-43-3",
            "101316-45-4",
            "101316-49-8",
            "101316-56-7",
            "101316-57-8",
            "101316-59-0",
            "101316-62-5",
            "101316-63-6",
            "101316-66-9",
            "101316-67-0",
            "101316-69-2",
            "101316-70-5",
            "101316-71-6",
            "101316-72-7",
            "101316-76-1",
            "101316-83-0",
            "101316-84-1",
            "101316-85-2",
            "101316-86-3",
            "101316-87-4",
            "10141-05-6",
            "101463-69-8",
            "101631-14-5",
            "101631-20-3",
            "101794-74-5",
            "101794-75-6",
            "101794-76-7",
            "101794-90-5",
            "101794-91-6",
            "101794-97-2",
            "101795-01-1",
            "101896-26-8",
            "101896-27-9",
            "101896-28-0",
            "102-06-7",
            "102110-14-5",
            "102110-15-6",
            "102110-55-4",
            "1024-57-3",
            "103-33-3",
            "103112-35-2",
            "103122-66-3",
            "10325-94-7",
            "10332-33-9",
            "103361-09-7",
            "10381-36-9",
            "104-91-6",
            "104653-34-1",
            "10486-00-7",
            "105024-66-6",
            "10588-01-9",
            "106-46-7",
            "106-47-8",
            "106-49-0",
            "106-87-6",
            "106-88-7",
            "106-89-8",
            "106-91-2",
            "106-92-3",
            "106-93-4",
            "106-94-5",
            "106-97-8",
            "106-99-0",
            "10605-21-7",
            "107-05-1",
            "107-06-2",
            "107-13-1",
            "107-20-0",
            "107-22-2",
            "107-30-2",
            "107534-96-3",
            "108-05-4",
            "108-45-2",
            "108-88-3",
            "108-91-8",
            "108-95-2",
            "108225-03-2",
            "109-86-4",
            "109-99-9",
            "110-00-9",
            "110-05-4",
            "110-49-6",
            "110-54-3",
            "110-71-4",
            "110-80-5",
            "110-85-0",
            "110-88-3",
            "110235-47-7",
            "11099-02-8",
            "111-15-9",
            "111-41-1",
            "111-44-4",
            "111-77-3",
            "111-96-6",
            "11113-50-1",
            "11113-74-9",
            "11113-75-0",
            "11132-10-8",
            "11138-47-9",
            "1116-54-7",
            "111988-49-9",
            "112-49-2",
            "1120-71-4",
            "114565-66-1",
            "115-96-8",
            "115662-06-1",
            "117-81-7",
            "117-82-8",
            "118-74-1",
            "118134-30-8",
            "118612-00-3",
            "118658-99-4",
            "119-90-4",
            "119-93-7",
            "119738-06-6",
            "120-32-1",
            "120-71-8",
            "12001-28-4",
            "12001-29-5",
            "12004-35-2",
            "12007-00-0",
            "12007-01-1",
            "12007-02-2",
            "12008-41-2",
            "120187-29-3",
            "12031-65-1",
            "12035-36-8",
            "12035-38-0",
            "12035-39-1",
            "12035-64-2",
            "12035-71-1",
            "12035-72-2",
            "12040-72-1",
            "12054-48-7",
            "12056-51-8",
            "12059-14-2",
            "12068-61-0",
            "121-14-2",
            "121-69-7",
            "121158-58-5",
            "12137-12-1",
        ];

        for cas_number in cas_numbers {
            info!("processing {cas_number}");
            assert!(is_cas_number(cas_number).unwrap());
        }
    }
}
