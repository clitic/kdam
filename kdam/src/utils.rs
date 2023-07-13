#[cfg(feature = "unicode")]
use unicode_segmentation::UnicodeSegmentation;

/// Returns floor division and modulus of two values.
pub(super) fn divmod(x: usize, y: usize) -> (usize, usize) {
    (x / y, x % y)
}

/// Get number of columns in current window or default to specified value.
pub(super) fn get_terminal_width() -> Option<u16> {
    terminal_size::terminal_size().map(|(w, _)| w.0)
}

/// Returns length of the given text.
#[cfg(feature = "unicode")]
pub(super) fn len(text: &str) -> usize {
    text.graphemes(true).count()
}

/// Returns length of the given text.
#[cfg(not(feature = "unicode"))]
pub(super) fn len(text: &str) -> usize {
    text.chars().count()
}
