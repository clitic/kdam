/// Get number of columns in current window or default to specified value.
pub fn get_columns_or(width: u16) -> u16 {
    terminal_size::terminal_size()
        .unwrap_or((terminal_size::Width(width), terminal_size::Height(0)))
        .0
         .0
}
