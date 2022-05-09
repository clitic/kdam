//! Terminal related functions for printing and colorization.
use std::io::Write;

/// Terminal colour reset escape code.
pub static COLOUR_RESET: &str = "\x1b[0m";

/// Do some platform specific terminal initialization.
pub fn init() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/c", "color"])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}

/// Create a colour escape code from primary colours or hex colour code.
pub fn colour(code: &str) -> String {
    if code.starts_with("#") {
        if code.len() == 7 {
            return format!(
                "\x1b[38;2;{};{};{}m",
                i16::from_str_radix(code.get(1..3).unwrap(), 16).unwrap(),
                i16::from_str_radix(code.get(3..5).unwrap(), 16).unwrap(),
                i16::from_str_radix(code.get(5..7).unwrap(), 16).unwrap()
            );
        } else {
            panic!(
                "Unknown hex colour format [{}]; Valid type: [hex (#00ff00)]",
                code
            );
        }
    }

    let mut new_colour = "\x1b[".to_string();
    let compare_colour = code.to_uppercase();

    if compare_colour.contains("BLACK") {
        new_colour.push_str("30");
    } else if compare_colour.contains("RED") {
        new_colour.push_str("31");
    } else if compare_colour.contains("GREEN") {
        new_colour.push_str("32");
    } else if compare_colour.contains("YELLOW") {
        new_colour.push_str("33");
    } else if compare_colour.contains("BLUE") {
        new_colour.push_str("34");
    } else if compare_colour.contains("MAGENTA") {
        new_colour.push_str("35");
    } else if compare_colour.contains("CYAN") {
        new_colour.push_str("36");
    } else if compare_colour.contains("WHITE") {
        new_colour.push_str("37");
    } else {
        return "".to_string();
    }

    if compare_colour.contains("ON BLACK") {
        new_colour.push_str(";40");
    } else if compare_colour.contains("ON RED") {
        new_colour.push_str(";41");
    } else if compare_colour.contains("ON GREEN") {
        new_colour.push_str(";42");
    } else if compare_colour.contains("ON YELLOW") {
        new_colour.push_str(";43");
    } else if compare_colour.contains("ON BLUE") {
        new_colour.push_str(";44");
    } else if compare_colour.contains("ON MAGENTA") {
        new_colour.push_str(";45");
    } else if compare_colour.contains("ON CYAN") {
        new_colour.push_str(";46");
    } else if compare_colour.contains("ON WHITE") {
        new_colour.push_str(";47");
    }

    if compare_colour.contains("BOLD") {
        new_colour.push_str(";1")
    }

    if compare_colour.contains("ITALIC") {
        new_colour.push_str(";3")
    }

    if compare_colour.contains("UNDERLINE") {
        new_colour.push_str(";4")
    }

    if compare_colour.contains("STRIKETHROUGH") {
        new_colour.push_str(";9")
    }

    new_colour.push_str("m");
    new_colour
}

/// Terminal colorization.
pub trait Colorizer {
    /// Add colour to the given text.
    ///
    /// # Example
    /// ```rust
    /// use kdam::term::Colorizer;
    ///
    /// println!("{}", "hello world!".colorize("blue on white"));
    /// ```
    fn colorize(self, code: &str) -> String;
}

impl<T: ToString> Colorizer for T {
    fn colorize(self, code: &str) -> String {
        format!("{}{}{}", colour(code), self.to_string(), COLOUR_RESET)
    }
}

/// Different ouput locations of `kdam::Bar`.
#[derive(Debug, Clone)]
pub enum Output {
    Stderr,
    Stdout,
}

/// Write to stdout followed by a flush.
pub fn write_to_stdout(text: std::fmt::Arguments) {
    let mut stdout = std::io::stdout();
    stdout.write_fmt(text).unwrap();
    stdout.flush().unwrap();
}

/// Write to stderr followed by a flush.
pub fn write_to_stderr(text: std::fmt::Arguments) {
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
