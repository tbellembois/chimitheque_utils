/// A case change applied by [`clean`].
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
#[must_use]
pub fn clean(s: &str, transform: Transform) -> String {
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

#[cfg(test)]
#[path = "string_tests.rs"]
mod string_tests;
