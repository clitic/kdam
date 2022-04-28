use std::io::Write;

/// Terminal colour reset escape code.
pub(crate) static COLOUR_RESET: &str = "\x1b[0m";

/// Write to stdout followed by a flush.
pub(crate) fn write_to_stdout(text: std::fmt::Arguments) {
    let mut stdout = std::io::stdout();
    stdout.write_fmt(text).unwrap();
    stdout.flush().unwrap();
}

/// Write to stderr followed by a flush.
pub(crate) fn write_to_stderr(text: std::fmt::Arguments) {
    let mut stderr = std::io::stderr();
    stderr.write_fmt(text).unwrap();
    stderr.flush().unwrap();
}

/// Get number of columns in current window.
/// On encountering error returns 0.
pub(crate) fn get_columns() -> u16 {
    terminal_size::terminal_size()
        .unwrap_or((terminal_size::Width(0), terminal_size::Height(0)))
        .0
         .0
}

/// Do some platform specific terminal initialization.
pub(crate) fn init() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/c", "color"])
            .spawn()
            .unwrap();
    }
}

/// Create a colour escape code from hex colour code.
pub(crate) fn colour(code: &str) -> String {
    if code.starts_with("#") {
        if code.len() == 7 {
            return format!(
                "\x1b[38;2;{};{};{}m",
                isize::from_str_radix(code.get(1..3).unwrap(), 16).unwrap(),
                isize::from_str_radix(code.get(3..5).unwrap(), 16).unwrap(),
                isize::from_str_radix(code.get(5..7).unwrap(), 16).unwrap()
            );
        } else {
            panic!(
                "Unknown hex colour format [{}]; Valid type: [hex (#00ff00)]",
                code
            );
        }
    }

    let compare_colour = code.to_uppercase();
    
    if compare_colour == "BLACK" {
        return "\x1b[30m".to_string();
    } else if compare_colour == "RED" {
        return "\x1b[31m".to_string();
    } else if compare_colour == "GREEN" {
        return "\x1b[32m".to_string();
    } else if compare_colour == "YELLOW" {
        return "\x1b[33m".to_string();
    } else if compare_colour == "BLUE" {
        return "\x1b[34m".to_string();
    } else if compare_colour == "MAGENTA" {
        return "\x1b[35m".to_string();
    } else if compare_colour == "CYAN" {
        return "\x1b[36m".to_string();
    } else if compare_colour == "WHITE" {
        return "\x1b[37m".to_string();
    } else {
        panic!("Unknown colour ({}); valid choices: [hex (#00ff00), BLACK, RED, GREEN, YELLOW, BLUE, MAGENTA, CYAN, WHITE]", compare_colour);
    }
}
