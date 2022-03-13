//! Terminal related functions.

use std::io::Write;

pub(crate) static COLOUR_RESET: &str = "\x1b[0m";

pub(crate) fn write_to_stdout(text: std::fmt::Arguments) {
    let mut stdout = std::io::stdout();
    stdout.write_fmt(text).unwrap();
    stdout.flush().unwrap();
}

pub(crate) fn write_to_stderr(text: std::fmt::Arguments) {
    let mut stderr = std::io::stderr();
    stderr.write_fmt(text).unwrap();
    stderr.flush().unwrap();
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
pub(crate) fn colour(c: &str) -> String {
    if c.starts_with("#") {
        if c.len() == 7 {
            return format!(
                "\x1b[38;2;{};{};{}m",
                isize::from_str_radix(c.get(1..3).unwrap(), 16).unwrap(),
                isize::from_str_radix(c.get(3..5).unwrap(), 16).unwrap(),
                isize::from_str_radix(c.get(5..7).unwrap(), 16).unwrap()
            );
        } else {
            panic!(
                "Unknown hex colour format [{}]; Valid type: [hex (#00ff00)]",
                c
            );
        }
    }

    let compare_colour = c.to_uppercase();
    if compare_colour == "BLACK" {
        "\x1b[30m".to_string()
    } else if compare_colour == "RED" {
        "\x1b[31m".to_string()
    } else if compare_colour == "GREEN" {
        "\x1b[32m".to_string()
    } else if compare_colour == "YELLOW" {
        "\x1b[33m".to_string()
    } else if compare_colour == "BLUE" {
        "\x1b[34m".to_string()
    } else if compare_colour == "MAGENTA" {
        "\x1b[35m".to_string()
    } else if compare_colour == "CYAN" {
        "\x1b[36m".to_string()
    } else if compare_colour == "WHITE" {
        "\x1b[37m".to_string()
    } else {
        panic!("Unknown colour ({}); valid choices: [hex (#00ff00), BLACK, RED, GREEN, YELLOW, BLUE, MAGENTA, CYAN, WHITE]", compare_colour);
    }
}

/// Get number of columns in current window.
pub(crate) fn get_columns() -> u16 {
    terminal_size::terminal_size()
        .unwrap_or((terminal_size::Width(0), terminal_size::Height(0)))
        .0
         .0
}
