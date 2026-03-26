#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::casnumber::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_valid_cas_numbers() {
        init_logger();

        let valid_cas_numbers = vec![
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

        for cas_number in valid_cas_numbers {
            assert!(is_cas_number(cas_number).is_ok(), "-> error {cas_number}");
        }
    }

    #[test]
    fn test_invalid_cas_numbers() {
        init_logger();

        let invalid_cas_numbers = vec![
            "0-0-0",         // First group too short
            "0000000-0-0",   // First group too long
            "00-0-0",        // Second group too short
            "00000-0-0",     // Second group too long
            "000-0-00",      // Check digit too long
            "000-0",         // Missing check digit
            "123-ABC-5",     // Non-numeric characters in groups
            "123-45-",       // Missing check digit
            "@@2-34-5",      // Invalid characters
            "12-34-56",      // Check digit too long
            "123-45-678",    // Check digit too long
            "123-A5-7",      // Non-numeric character in group
            "123-4B-5",      // Non-numeric character in group
            "12345678-89-0", // First group too long (7 digits)
            "123456-78-90",  // Check digit too long (2 digits)
            "ABC-12345-6",   // Non-numeric characters in first group
            "12-ABC34-5",    // Non-numeric characters in second group
            "123-45-A",      // Non-numeric character as check digit
            "000-00-0",      // All zeros
            "123-A5-7",      // Letter in group2
            "A23-45-6",      // Letter in group1
            "123-45-A",      // Letter as check digit
            "123-X5-6",      // Letter in group2
            "S23-45-6",      // Letter in group1
            "123-4X-6",      // Letter in group2
            "123-45-S",      // Letter as check digit
            "1!3-45-6",      // Special character in group1
            "123-4#-6",      // Special character in group2
            "123-45-@",      // Special character as check digit
            "123-€5-6",      // Special character in group2
            "¢23-45-6",      // Currency symbol in group1
            "123-4¢-6",      // Currency symbol in group2
            "123-45-¢",      // Currency symbol as check digit
            "100-٠٠-٥",      // Arabic-Indic digits
            "100-𝟎𝟎-𝟕",      // Mathematical bold digits
            "100-⁰⁰-⁵",      // Superscript digits
            "100-⁰⁰-₅",      // Subscript digits
            "100-ℵℵ-ℵ",      // Hebrew aleph symbols (should fail)
            "100-ⰀⰀ-Ⰰ",      // Glagolitic letters (should fail)
            "100-℁℁-℁",      // Latin extended symbols (should fail)
            "100-一一-一",   // CJK unified ideographs (should fail)
            "100-☀☀-☀",      // Miscellaneous symbols (should fail)
            "100-ℂℂ-ℂ",      // Double-struck letters (should fail)
            "",              // Empty string
            "            ",  // Whitespace string
            "1337-420-69",   // Pop culture reference - likely invalid
            "999-99-98",     // Check digit should be 9 (based on the checksum)
            "111-11-10",     // Check digit should be 1 (based on the checksum)
            "123-45-678",    // Check digit too long
            "000-00-1",      // All zeros except check digit - likely invalid
            "999-99-0",      // Check digit should be 9 (based on the checksum)
            "123-456-7",     // Second group too long
            "12345678-9-0",  // First group too long
            "1-23-4",        // First group too short
            "12-3-4",        // First group too short
            "123-4-5",       // Second group too short
            "123-45-",       // Missing check digit
            "123-45-gh",     // Non-numeric check digit
            "123-4a-5",      // Non-numeric character in group2
            "12a-45-6",      // Non-numeric character in group1
            "123 45-6",      // Whitespace instead of hyphen
            "123-45-6 ",     // Trailing whitespace
            " 123-45-6",     // Leading whitespace
            "123-45-6\n",    // Newline character
        ];

        for cas_number in invalid_cas_numbers {
            assert!(is_cas_number(cas_number).is_err(), "-> error {cas_number}");
        }

        // All zeros
    }

    #[test]
    fn test_format_empty_cas_number_error() {
        let error = CasNumberError::EmptyCasNumber;
        assert_eq!(error.to_string(), "empty CAS number");
    }

    #[test]
    fn test_format_digit_groups_capture_error() {
        let error = CasNumberError::DigitGroupsCapture;
        assert_eq!(error.to_string(), "can not capture digit groups");
    }

    #[test]
    fn test_format_char_to_digit_conversion_error() {
        let error = CasNumberError::CharTodigitConversion('a');
        assert_eq!(error.to_string(), "can not convert a into digit");
    }

    #[test]
    fn test_format_no_check_digit_found_error() {
        let error = CasNumberError::NoCheckDigitFound;
        assert_eq!(error.to_string(), "no check digit found");
    }

    #[test]
    fn test_format_check_digit_does_not_match_error() {
        let error = CasNumberError::CheckDigitDoesNotMatch;
        assert_eq!(error.to_string(), "check digit does not match");
    }

    #[test]
    fn test_format_all_zeros_error() {
        let error = CasNumberError::AllZeros;
        assert_eq!(error.to_string(), "all zeros");
    }
}
