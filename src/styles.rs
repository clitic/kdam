pub(crate) static TQDMCHARSET: [&str; 8] = [
    "\u{258F}", "\u{258E}", "\u{258D}", "\u{258C}", "\u{258B}", "\u{258A}", "\u{2589}", "\u{2588}",
];
pub(crate) static TQDMASCIICHARSET: [&str; 10] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "#"];
pub(crate) static FILLUPCHARSET: [&str; 8] = [
    "\u{2581}", "\u{2582}", "\u{2583}", "\u{2584}", "\u{2585}", "\u{2586}", "\u{2587}", "\u{2588}",
];

/// Different types of pre-configured animation styles for `kdam::Bar`.
#[derive(Debug, Clone)]
pub enum Animation {
    Tqdm,
    TqdmAscii,
    FillUp,
    Classic,
    Arrow,
}

/// Different ouput locations of `kdam::Bar`.
#[derive(Debug, Clone)]
pub enum Output {
    Stdout,
    Stderr,
}
