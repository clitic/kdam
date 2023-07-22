/// Returns floor division and modulus of two values.
pub(super) fn divmod(x: usize, y: usize) -> (usize, usize) {
    (x / y, x % y)
}

/// Returns length of the given text.
#[cfg(feature = "unicode")]
pub(super) fn len(text: &str) -> usize {
    unicode_width::UnicodeWidthStr::width(text)
}

/// Returns length of the given text.
#[cfg(not(feature = "unicode"))]
pub(super) fn len(text: &str) -> usize {
    text.chars().count()
}
