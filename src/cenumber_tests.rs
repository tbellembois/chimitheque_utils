#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::cenumber::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_valid_ce_numbers() {
        init_logger();

        let valid_ce_numbers = vec![
            "214-480-6",
            "200-419-0",
            "275-117-5",
            "210-085-8",
            "231-448-7",
            "216-542-8",
            "229-347-8",
            "209-536-1",
            "216-700-6",
            "200-139-9",
            "214-195-7",
            "225-834-4",
            "207-634-9",
            "203-593-6",
            "211-843-0",
            "203-489-0",
            "207-637-5",
            "225-362-9",
            "210-521-7",
            "220-711-1",
            "204-651-3",
            "231-845-5",
            "206-698-5",
            "208-945-2",
            "203-309-0",
            "243-229-3",
            "200-675-3",
            "233-826-7",
            "230-907-9",
            "204-423-3",
            "201-345-1",
            "223-090-5",
            "200-842-0",
            "202-713-4",
            "225-403-0",
            "212-903-9",
            "237-124-1",
            "221-141-6",
            "210-935-8",
            "203-809-9",
            "233-253-2",
            "201-997-7",
            "231-962-1",
            "236-944-7",
            "208-745-5",
            "254-705-5",
            "200-729-6",
            "213-638-1",
            "234-717-7",
            "231-391-8",
            "249-313-6",
            "247-781-6",
            "231-829-8",
            "207-838-8",
            "233-058-2",
            "219-989-7",
            "240-795-3",
            "214-787-5",
            "206-851-6",
            "225-266-7",
            "207-622-3",
            "234-118-0",
            "200-193-3",
            "205-281-5",
            "205-769-8",
            "253-149-0",
            "208-401-4",
            "204-125-3",
            "212-782-2",
            "211-402-2",
            "210-155-8",
            "215-040-6",
            "228-393-6",
            "204-135-8",
            "215-268-6",
            "211-284-2",
            "249-622-6",
            "211-798-7",
            "204-402-9",
            "227-663-0",
            "202-548-8",
            "204-152-0",
            "203-611-2",
            "235-060-9",
            "204-602-6",
            "211-923-5",
            "215-160-9",
            "212-668-2",
            "200-718-6",
            "209-608-2",
            "235-186-4",
            "200-578-6",
            "200-441-0",
            "204-469-4",
            "253-248-9",
            "221-284-4",
            "231-853-9",
            "200-536-7",
            "202-729-1",
            "205-043-0",
            "277-730-3",
            "214-910-2",
            "202-701-9",
            "229-642-1",
            "216-483-8",
            "202-876-1",
            "249-373-3",
            "231-832-4",
            "200-844-1",
            "247-081-0",
            "204-593-9",
            "235-852-4",
            "231-957-4",
            "203-473-3",
            "203-541-2",
            "200-806-4",
            "207-123-0",
            "204-823-8",
            "231-299-8",
            "210-064-3",
            "204-504-3",
            "203-813-0",
            "231-820-9",
            "208-221-6",
            "206-007-7",
            "209-875-5",
            "205-745-7",
        ];

        for ce_number in valid_ce_numbers {
            assert!(is_ce_number(ce_number).is_ok(), "-> error {ce_number}");
        }
    }

    #[test]
    fn test_invalid_ce_numbers() {
        init_logger();

        let invalid_ce_numbers = vec![
            "0-0-0",            // First group too short
            "0000-0-0",         // First group too long
            "0-0-0",            // Second group too short (first part)
            "000-000-0",        // Second group too long
            "000-0-0",          // Check digit too short
            "000-00-00",        // Check digit too long
            "123-ABC-5",        // Non-numeric characters
            "123-45-",          // Missing check digit
            "@/장으로#$%^&*()", // Invalid characters
            "12-34-56",         // Check digit too long
            "123-45-678",       // Check digit too long
            "123-A5-7",         // Non-numeric characters
            "123-4B-5",         // Non-numeric characters
            "1234567-89-0",     // First group too long
            "123456-789-0",     // Second group too long
            "123-45678-0",      // Second group too long
            "ABC-12345-6",      // Non-numeric characters
            "12-ABC34-5",       // Non-numeric characters
            "123-45-A",         // Non-numeric check digit
            "000-00-1",         // Valid but really weird case
            "123-45-5",         // Forced invalid checksum (actual check digit should be 7)
            "246-824-3",        // Forced invalid checksum (actual check digit should be 2)
            "12-3-456",         // Incorrect separators
            "12A-45B-6C",       // Mixed alphanumeric
            "123-456",          // Missing separator
            "123-45-6-7",       // Extra group
            " ",                // Whitespace
            "\t",               // Tab
            "\n",               // Newline
            "\r",               // Carriage return
            "123 45-6",         // Space instead of separator
            " 123-45-6",        // Leading whitespace
            "123-45-6 ",        // Trailing whitespace
            "123€456€7",        // ⬇︎ wrong separator for number, €⬆︎
            "123⚡456⚡7",      // ⬇︎ wrong separator for number, ⚡⬆︎
            "123☠456☠7",        // ⬇︎ wrong separator for number, ☠⬆︎
            "123❗456❗7",      // ⬇︎ wrong separator for number, ❗⬆︎
            "123❓456❓7",      // ⬇︎ wrong separator for number, ❓⬆︎
            "123⚠456⚠7",        // ⬇︎ wrong separator for number, ⚠⬆︎
            "123❕456❕7",      // ⬇︎ wrong separator for number, ❕⬆︎
            "123❗456❗7",      // ⬇︎ wrong separator for number, ❗⬆︎
            "123❕456❕7",      // ⬇︎ wrong separator for number, ❕⬆︎
            "123⚕456⚕7",        // ⬇︎ wrong separator for number, ⚕⬆︎
        ];

        for ce_number in invalid_ce_numbers {
            assert!(is_ce_number(ce_number).is_err(), "-> error {ce_number}");
        }
    }

    #[test]
    fn test_format_empty_ce_number_error() {
        let error = CeNumberError::EmptyCeNumber;
        assert_eq!(error.to_string(), "empty CE number");
    }

    #[test]
    fn test_format_digit_groups_capture_error() {
        let error = CeNumberError::DigitGroupsCaptureError;
        assert_eq!(error.to_string(), "can not capture digit groups");
    }

    #[test]
    fn test_format_char_to_digit_conversion_error() {
        let error = CeNumberError::CharTodigitConversionerror('a');
        assert_eq!(error.to_string(), "can not convert a into digit");
    }

    #[test]
    fn test_format_no_check_digit_found_error() {
        let error = CeNumberError::NoCheckDigitFound;
        assert_eq!(error.to_string(), "no check digit found");
    }

    #[test]
    fn test_format_check_digit_does_not_match_error() {
        let error = CeNumberError::CheckDigitDoesNotMatch;
        assert_eq!(error.to_string(), "check digit does not match");
    }

    #[test]
    fn test_format_all_zeros_error() {
        let error = CeNumberError::AllZeros;
        assert_eq!(error.to_string(), "all zeros");
    }
}
