#[derive(Copy, Clone)]
pub enum Transform {
    None,
    Uppercase,
    Lowercase,
}

/// Cleans a string by trimming whitespaces and replacing multiple spaces with a single space.
/// Optionally, it can convert the string to uppercase or lowercase.
#[must_use]
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
#[path = "string_tests.rs"]
mod string_tests;
