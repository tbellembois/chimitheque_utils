#[cfg(test)]
mod tests {
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
        assert_eq!("a b c", clean(" \t\ra\n\nb\rc\t", Transform::None));
        assert_eq!("A B C", clean(" \t\ra\n\nb\rc\t", Transform::Uppercase));

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
}
