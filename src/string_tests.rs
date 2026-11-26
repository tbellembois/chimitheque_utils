#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::string::{Transform, clean};

    #[test]
    fn test_clean() {
        // Test: No transformation (Transform::None)
        assert_eq!("abc", clean(" abc", Transform::None));
        assert_eq!("abc", clean("abc ", Transform::None));
        assert_eq!("abc", clean(" abc ", Transform::None));
        assert_eq!("a bc", clean("a  bc", Transform::None));
        assert_eq!("a b c", clean("a  b    c", Transform::None));

        // Test: With all whitespace characters (not just spaces)
        assert_eq!("abc", clean("   \t  abc  \t\n", Transform::None));
        assert_eq!("abc", clean("\r\n \t abc \r\n \t", Transform::None));

        // Test: Empty string
        assert_eq!("", clean("", Transform::None));
        assert_eq!("", clean("   ", Transform::None));
        assert_eq!("", clean("\t\n\r", Transform::None));

        // Test: Single space with no transformation
        assert_eq!("a b c", clean("a b c", Transform::None));

        // Test: No transformation but with special characters
        assert_eq!("a@b#c$", clean(" a@b#c$ ", Transform::None));
        assert_eq!("a@ b# c$", clean("a@  b#  c$", Transform::None));

        // Test: Uppercase transformation
        assert_eq!("ABC", clean(" abc ", Transform::Uppercase));
        assert_eq!("A B C", clean("a b c", Transform::Uppercase));
        assert_eq!("A@ B# C$", clean(" a@ b# c$ ", Transform::Uppercase));
        assert_eq!("A B C", clean(" a  b    c ", Transform::Uppercase));
        assert_eq!("", clean("", Transform::Uppercase));
        assert_eq!("", clean("\t\n\r", Transform::Uppercase));

        // Test: Lowercase transformation
        assert_eq!("abc", clean(" ABC ", Transform::Lowercase));
        assert_eq!("a b c", clean("A B C", Transform::Lowercase));
        assert_eq!("a@b#c$", clean(" A@B#C$ ", Transform::Lowercase));
        assert_eq!("a b c", clean(" A  B    C ", Transform::Lowercase));
        assert_eq!("", clean("", Transform::Lowercase));
        assert_eq!("", clean("  \t\n\r  ", Transform::Lowercase));

        // Test: Multiple consecutive and various whitespace characters
        assert_eq!("a\n\nb c", clean(" \t\ra\n\nb\rc\t", Transform::None));
        assert_eq!("A\n\nB C", clean(" \t\ra\n\nb\rc\t", Transform::Uppercase));

        // Test edge cases with punctuation and numbers
        assert_eq!(
            "a,,, b... c(...)",
            clean("a,,,  b...    c(...)", Transform::None)
        );
        assert_eq!(
            "(a),,(b) ...(c)",
            clean("(a),,(b)   ...(c)", Transform::None)
        );
        assert_eq!(
            "A,,, B... C(...)",
            clean("a,,,  b...    c(...)", Transform::Uppercase)
        );

        // Test mixed transformations
        assert_eq!("A B C", clean(" a   b     c ", Transform::Uppercase));
        assert_eq!("a b c", clean(" A   B     C ", Transform::Lowercase));

        // Multiple spaces followed by punctuation
        assert_eq!(
            "a , b . c !",
            clean(" a  ,   b  .   c  ! ", Transform::None)
        );
    }

    #[test]
    fn test_clean_keeps_line_breaks() {
        // Each line is trimmed and its whitespaces are collapsed.
        assert_eq!("a b\nc d", clean("  a   b  \n  c \t d  ", Transform::None));
        // Inner empty lines are kept.
        assert_eq!(
            "line 1\n\nline 2",
            clean("line 1\n  \nline 2", Transform::None)
        );
        // Leading and trailing empty lines are dropped.
        assert_eq!("abc", clean("\n \n abc \n\t\n", Transform::None));
        // `\r\n` becomes `\n`.
        assert_eq!("a\nb", clean("a\r\nb\r\n", Transform::None));
        assert_eq!("A\nB", clean(" a \n b ", Transform::Uppercase));
    }
}
