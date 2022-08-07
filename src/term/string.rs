use unicode_segmentation::UnicodeSegmentation;

/// Get terminal display length of string.
pub fn strdisplen(mut text: String) -> usize {
    text = text.replace("\x1b[0m", "");

    while let Some(start) = text.find("\x1b[") {
        text = text.replace(
            &text[start..(start + text[start..].find("m").unwrap() + 1)],
            "",
        );
    }

    text.graphemes(true).count()
}
