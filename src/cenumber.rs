use log::debug;
use regex::Regex;

// https://en.wikipedia.org/wiki/European_Community_number
pub fn is_ce_number(number: &str) -> Result<bool, String> {
    // Build regex.
    let re =
        match Regex::new(r"^(?P<group1>[0-9]{3})-(?P<group2>[0-9]{3})-(?P<checkdigit>[0-9]{1})$") {
            Ok(re) => re,
            Err(e) => return Err(format!("invalid regex: {}", e)),
        };

    // Capture groups and check number.
    let captures = match re.captures(number) {
        Some(captures) => captures,
        None => return Err("can not capture digit groups".to_string()),
    };

    let group1 = &captures["group1"];
    let group2 = &captures["group2"];
    let checkdigit_char = &captures["checkdigit"];
    debug!("group1:{group1} - group2:{group2} - checkdigit_char:{checkdigit_char}");

    // Multiplier that will increase at each operation.
    let mut multiplier = 1;
    // Total sum of each operation.
    let mut total = 0;

    // Processing group1.
    for digit_char in group1.chars() {
        let digit = match digit_char.to_digit(10) {
            Some(digit) => digit,
            None => return Err(format!("can not convert {digit_char} into digit")),
        };
        total += multiplier * digit;
        multiplier += 1;
    }

    // Processing group2.
    for digit_char in group2.chars() {
        let digit = match digit_char.to_digit(10) {
            Some(digit) => digit,
            None => return Err(format!("can not convert {digit_char} into digit")),
        };
        total += multiplier * digit;
        multiplier += 1;
    }

    // Calculating modulo.
    let modulo = total % 11;
    debug!("modulo:{modulo}");

    // Processing checkdigit.
    if let Some(digit_char) = checkdigit_char.chars().next() {
        let digit = match digit_char.to_digit(10) {
            Some(digit) => digit,
            None => return Err(format!("can not convert {digit_char} into digit")),
        };

        Ok(digit.eq(&modulo))
    } else {
        Err("not check digit found".to_string())
    }
}

#[cfg(test)]
mod tests {

    use log::info;

    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_is_ce_number_nok() {
        init_logger();

        // Regex test.
        let ce_numbers = vec![
            "21-480-6",   // first group too short
            "2140-480-6", // first group too long
            "214-48-6",   // second group too short
            "214-4800-6", // second group too long
            "214-480-",   // no check digit
            "ABC-480-5",  // wrong chars
        ];

        for ce_number in ce_numbers {
            info!("processing {ce_number}");
            assert_eq!(
                is_ce_number(ce_number),
                Err("can not capture digit groups".to_string())
            );
        }

        // Check digit test.
        assert_eq!(is_ce_number("214-480-7"), Ok(false));
        assert_eq!(is_ce_number("200-419-1"), Ok(false));
    }

    #[test]
    fn test_is_ce_number_ok() {
        init_logger();

        let ce_numbers = vec![
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
            "200-658-0",
            "200-730-1",
            "233-046-7",
            "203-628-5",
            "203-561-1",
            "203-707-4",
            "228-319-2",
            "204-368-5",
            "203-576-3",
            "207-356-8",
            "218-862-3",
            "202-252-9",
            "215-270-7",
            "232-056-9",
            "202-819-0",
            "200-517-3",
            "201-478-5",
            "238-679-2",
            "231-175-3",
            "212-889-4",
            "214-318-4",
            "215-116-9",
            "219-468-4",
            "242-641-0",
            "231-211-8",
            "212-642-0",
            "215-713-4",
        ];

        for ce_number in ce_numbers {
            info!("processing {ce_number}");
            assert_eq!(is_ce_number(ce_number), Ok(true));
        }
    }
}
