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
    fn test_is_ce_number() {
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
    fn test_is_ce_number_errors() {
        init_logger();

        assert_eq!(is_ce_number(""), Err(CeNumberError::EmptyCeNumber));

        for ce_number in [
            " ",                // Whitespace only
            "\t",               // Tab
            "\n",               // Newline
            "0-0-0",            // Groups too short
            "0000-000-0",       // First group too long
            "12-345-6",         // First group too short
            "123-45-6",         // Second group too short
            "123-4567-8",       // Second group too long
            "123-456-",         // Missing check digit
            "123-456-78",       // Check digit too long
            "123-456",          // Missing group
            "123-456-7-8",      // Extra group
            "123-ABC-5",        // Letters in the second group
            "12A-456-7",        // Letter in the first group
            "123-456-A",        // Letter as check digit
            "@/장으로#$%^&*()", // Invalid chars
            "123 456-7",        // Space instead of a dash
            " 123-456-7",       // Leading whitespace
            "123-456-7 ",       // Trailing whitespace
            "123€456€7",        // Wrong separator
            "123⚠456⚠7",        // Wrong separator
        ] {
            assert_eq!(
                is_ce_number(ce_number),
                Err(CeNumberError::DigitGroupsCapture),
                "{ce_number:?}"
            );
        }

        // All the digits before the check digit are zeros, whatever the check
        // digit.
        for ce_number in ["000-000-0", "000-000-1"] {
            assert_eq!(
                is_ce_number(ce_number),
                Err(CeNumberError::AllZeros),
                "{ce_number}"
            );
        }

        for ce_number in [
            "214-480-5", // Should be 6
            "246-824-3", // Should be 6
            "123-456-7", // Should be 3
        ] {
            assert_eq!(
                is_ce_number(ce_number),
                Err(CeNumberError::CheckDigitDoesNotMatch),
                "{ce_number}"
            );
        }
    }

    #[test]
    fn test_display() {
        for (error, message) in [
            (CeNumberError::EmptyCeNumber, "empty CE number"),
            (
                CeNumberError::DigitGroupsCapture,
                "can not capture digit groups",
            ),
            (
                CeNumberError::CharToDigitConversion('a'),
                "can not convert a into digit",
            ),
            (CeNumberError::NoCheckDigitFound, "no check digit found"),
            (
                CeNumberError::CheckDigitDoesNotMatch,
                "check digit does not match",
            ),
            (CeNumberError::AllZeros, "all zeros"),
        ] {
            assert_eq!(error.to_string(), message);
        }
    }
}
