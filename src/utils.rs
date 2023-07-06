#[cfg(feature = "unicode")]
use unicode_segmentation::UnicodeSegmentation;

#[cfg(feature = "unicode")]
pub(super) fn len(text: &str) -> usize {
    text.graphemes(true).count()
}

#[cfg(not(feature = "unicode"))]
pub(super) fn len(text: &str) -> usize {
    text.chars().count()
}
