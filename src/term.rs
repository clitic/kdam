//! Terminal related functions.

use std::io::Write;

pub(crate) static COLOUR_RESET: &str = "\x1b[0m";
static COLOUR_BLACK: &str = "\x1b[30m";
static COLOUR_RED: &str = "\x1b[31m";
static COLOUR_GREEN: &str = "\x1b[32m";
static COLOUR_YELLOW: &str = "\x1b[33m";
static COLOUR_BLUE: &str = "\x1b[34m";
static COLOUR_MAGENTA: &str = "\x1b[35m";
static COLOUR_CYAN: &str = "\x1b[36m";
static COLOUR_WHITE: &str = "\x1b[37m";

/// Do some platform specific terminal initialization.
pub fn init() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/c", "color"])
            .spawn()
            .unwrap();
    }
}

/// Create a colour escape code from hex colour code.
pub fn colour(c: &str) -> String {
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
        COLOUR_BLACK.to_string()
    } else if compare_colour == "RED" {
        COLOUR_RED.to_string()
    } else if compare_colour == "GREEN" {
        COLOUR_GREEN.to_string()
    } else if compare_colour == "YELLOW" {
        COLOUR_YELLOW.to_string()
    } else if compare_colour == "BLUE" {
        COLOUR_BLUE.to_string()
    } else if compare_colour == "MAGENTA" {
        COLOUR_MAGENTA.to_string()
    } else if compare_colour == "CYAN" {
        COLOUR_CYAN.to_string()
    } else if compare_colour == "WHITE" {
        COLOUR_WHITE.to_string()
    } else {
        panic!("Unknown colour ({}); valid choices: [hex (#00ff00), BLACK, RED, GREEN, YELLOW, BLUE, MAGENTA, CYAN, WHITE]", compare_colour);
    }
}

/// Move the cursor n times up.
pub fn move_up(n: u16) {
    print!("\x1b[{}A", n);
}

/// Get number of columns in current window.
pub(crate) fn get_columns() -> u16 {
    terminal_size::terminal_size().unwrap_or((terminal_size::Width(0), terminal_size::Height(0))).0 .0
}

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
