/// A case change applied by [`clean_required_text`].
#[derive(Copy, Clone)]
pub enum Transform {
    /// Keeps the case.
    None,
    /// Converts to uppercase.
    Uppercase,
    /// Converts to lowercase.
    Lowercase,
}

/// Cleans a string line by line: each line is trimmed and its multiple
/// whitespaces are replaced with a single space. Line breaks are kept,
/// except at the start and the end of the string. `\r\n` line breaks become
/// `\n`.
/// Optionally, it can convert the string to uppercase or lowercase.
///
/// Private: other crates call [`clean_required_text`] or
/// [`clean_optional_text`], which also handle an empty result.
#[must_use]
fn clean(s: &str, transform: Transform) -> String {
    // `split_whitespace` drops the leading and trailing whitespaces of each
    // line; `lines` also removes the `\r` of `\r\n`.
    let lines: Vec<String> = s
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect();

    // Drop the empty lines at the start and the end.
    let start = lines
        .iter()
        .position(|line| !line.is_empty())
        .unwrap_or(lines.len());
    let end = lines
        .iter()
        .rposition(|line| !line.is_empty())
        .map_or(start, |last| last + 1);
    let result = lines[start..end].join("\n");

    match transform {
        Transform::None => result,
        Transform::Uppercase => result.to_uppercase(),
        Transform::Lowercase => result.to_lowercase(),
    }
}

/// Cleans a required text and returns it, or `None` if the cleaned text is
/// empty.
///
/// Each line is trimmed and its multiple whitespaces are replaced with a
/// single space. Line breaks are kept, except at the start and the end of the
/// text, and `\r\n` becomes `\n`. `transform` can also convert the text to
/// uppercase or lowercase.
#[must_use]
pub fn clean_required_text(s: &str, transform: Transform) -> Option<String> {
    let cleaned = clean(s, transform);
    (!cleaned.is_empty()).then_some(cleaned)
}

/// Cleans an optional text in place like [`clean_required_text`], keeping its
/// case. An empty text becomes `None`.
pub fn clean_optional_text(value: &mut Option<String>) {
    *value = value
        .as_deref()
        .and_then(|text| clean_required_text(text, Transform::None));
}

#[cfg(test)]
#[path = "string_tests.rs"]
mod string_tests;
