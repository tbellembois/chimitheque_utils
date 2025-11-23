pub enum Transform {
    None,
    Uppercase,
    Lowercase,
}

/// Cleans a string by trimming whitespaces and replacing multiple spaces with a single space.
/// Optionally, it can convert the string to uppercase or lowercase.
pub fn clean(s: &str, transform: Transform) -> String {
    let mut result: String;

    // Trim whitespaces.
    result = s.trim().to_string();

    // Replace more than one space.
    let words: Vec<_> = result.split_whitespace().collect();
    result = words.join(" ");

    match transform {
        Transform::None => result,
        Transform::Uppercase => result.to_uppercase(),
        Transform::Lowercase => result.to_lowercase(),
    }
}

#[cfg(test)]
mod tests {
    use crate::string::{clean, Transform};

    #[test]
    fn test_clean() {
        assert_eq!("abc", clean(" abc", Transform::None));
        assert_eq!("abc", clean("abc ", Transform::None));
        assert_eq!("abc", clean(" abc ", Transform::None));
        assert_eq!("a bc", clean("a  bc", Transform::None));
        assert_eq!("a b c", clean("a  b    c", Transform::None));
        assert_eq!("ABC", clean(" abc ", Transform::Uppercase));
        assert_eq!("abc", clean(" ABC ", Transform::Lowercase));
    }
}
