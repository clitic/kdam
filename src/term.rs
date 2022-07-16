//! Terminal related functions for printing and text colorization.

use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};

static COLOURS_ENABLED: AtomicBool = AtomicBool::new(false);

/// Stderr and Stdout writer for `kdam::Bar`.
#[derive(Debug)]
pub enum Writer {
    Stderr,
    Stdout,
}

impl Writer {
    /// Print `std::fmt::Arguments` in terminal followed by a flush.
    pub fn print(&self, args: std::fmt::Arguments) {
        match self {
            Self::Stderr => {
                let mut writer = std::io::stderr();
                writer.write_fmt(args).unwrap();
                writer.flush().unwrap();
            }
            Self::Stdout => {
                let mut writer = std::io::stdout();
                writer.write_fmt(args).unwrap();
                writer.flush().unwrap();
            }
        }
    }

    /// Print `&str` in terminal followed by a flush.
    pub fn print_str(&self, text: &str) {
        match self {
            Self::Stderr => {
                let mut writer = std::io::stderr();
                writer.write(text.as_bytes()).unwrap();
                writer.flush().unwrap();
            }
            Self::Stdout => {
                let mut writer = std::io::stdout();
                writer.write(text.as_bytes()).unwrap();
                writer.flush().unwrap();
            }
        }
    }
}

/// Get number of columns in current window.
/// On encountering error returns 0.
pub fn get_columns() -> u16 {
    terminal_size::terminal_size()
        .unwrap_or((terminal_size::Width(0), terminal_size::Height(0)))
        .0
         .0
}

/// Create colour escape code from primary colours or hex colour code.
///
/// # Example
///
/// ```rust
/// use kdam::term::colour;
///
/// assert_eq!(colour("bold red"), "\x1b[31;1m");
/// assert_eq!(colour("blue on white"), "\x1b[34;47m");
/// ```
pub fn colour(colour_code: &str) -> String {
    if !COLOURS_ENABLED.load(Ordering::Acquire) {
        if cfg!(target_os = "windows") {
            std::process::Command::new("cmd")
                .args(["/c", "color"])
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }

        COLOURS_ENABLED.store(true, Ordering::SeqCst);
    }

    if colour_code.starts_with("#") {
        if colour_code.len() == 7 {
            return format!(
                "\x1b[38;2;{};{};{}m",
                i16::from_str_radix(colour_code.get(1..3).unwrap(), 16).unwrap(),
                i16::from_str_radix(colour_code.get(3..5).unwrap(), 16).unwrap(),
                i16::from_str_radix(colour_code.get(5..7).unwrap(), 16).unwrap()
            );
        } else {
            panic!(
                "Unknown hex colour format [{}]; Valid type: [hex (#00ff00)]",
                colour_code
            );
        }
    }

    let color = colour_code.to_uppercase();
    let mut code = "\x1b[".to_string();

    if color.contains("BLACK") {
        code += "30";
    } else if color.contains("RED") {
        code += "31";
    } else if color.contains("GREEN") {
        code += "32";
    } else if color.contains("YELLOW") {
        code += "33";
    } else if color.contains("BLUE") {
        code += "34";
    } else if color.contains("MAGENTA") {
        code += "35";
    } else if color.contains("CYAN") {
        code += "36";
    } else if color.contains("WHITE") {
        code += "37";
    } else {
        return "".to_string();
    }

    if color.contains("ON BLACK") {
        code += ";40";
    } else if color.contains("ON RED") {
        code += ";41";
    } else if color.contains("ON GREEN") {
        code += ";42";
    } else if color.contains("ON YELLOW") {
        code += ";43";
    } else if color.contains("ON BLUE") {
        code += ";44";
    } else if color.contains("ON MAGENTA") {
        code += ";45";
    } else if color.contains("ON CYAN") {
        code += ";46";
    } else if color.contains("ON WHITE") {
        code += ";47";
    }

    if color.contains("BOLD") {
        code += ";1";
    }

    if color.contains("ITALIC") {
        code += ";3";
    }

    if color.contains("UNDERLINE") {
        code += ";4";
    }

    if color.contains("STRIKETHROUGH") {
        code += ";9";
    }

    code += "m";
    code
}

/// Add colour escape codes to the given text for printing coloured text in terminal.
/// This trait is only implemented for `&str` and `String`.
pub trait Colorizer {
    /// Add colour escape codes to the given text.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kdam::term::Colorizer;
    ///
    /// println!("{}", "hello world!".colorize("bold red"));
    /// println!("{}", "hello world!".colorize("blue on white"));
    /// ```
    fn colorize<'a>(&'a self, code: &str) -> String;
}

impl Colorizer for str {
    fn colorize(&self, code: &str) -> String {
        colour(code) + self + "\x1b[0m"
    }
}

impl Colorizer for String {
    fn colorize(&self, code: &str) -> String {
        colour(code) + self + "\x1b[0m"
    }
}
