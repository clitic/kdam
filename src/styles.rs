// Animation Unicodes
pub(crate) static FILLUPCHARSET: [&str; 8] = [
    "\u{2581}", "\u{2582}", "\u{2583}", "\u{2584}", "\u{2585}", "\u{2586}", "\u{2587}", "\u{2588}",
];
pub(crate) static TQDMCHARSET: [&str; 8] = [
    "\u{258F}", "\u{258E}", "\u{258D}", "\u{258C}", "\u{258B}", "\u{258A}", "\u{2589}", "\u{2588}",
];
pub(crate) static TQDMASCIICHARSET: [&str; 10] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "#"];

// Spinner Unicodes
pub(crate) static CLASSICSPINNER: [&str; 4] = ["\\", "|", "/", "-"];
pub(crate) static FIRACODESPINNER: [&str; 6] = ["\u{EE06}", "\u{EE07}", "\u{EE08}", "\u{EE09}", "\u{EE0A}", "\u{EE0B}"];

/// Different types of pre-configured animation styles for `kdam::Bar`.
#[derive(Debug, Clone)]
pub enum Animation {
    Arrow,
    Classic,
    Custom,
    FillUp,
    FiraCode,
    Tqdm,
    TqdmAscii,
}

/// Different ouput locations of `kdam::Bar`.
#[derive(Debug, Clone)]
pub enum Output {
    Stderr,
    Stdout,
}
