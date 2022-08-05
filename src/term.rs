//! Terminal related functions for printing and text colorization.

use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};

static COLOURS_ENABLED: AtomicBool = AtomicBool::new(false);

/// Stderr and Stdout writer for [Bar](crate::Bar).
#[derive(Debug, Clone)]
pub enum Writer {
    Stderr,
    Stdout,
}

impl From<&str> for Writer {
    fn from(output: &str) -> Self {
        match output.to_lowercase().as_str() {
            "stdout" => Self::Stdout,
            _ => Self::Stderr,
        }
    }
}

impl Writer {
    /// Print [Arguments](std::fmt::Arguments) in terminal followed by a flush.
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

    /// Prints to the standard error at specified position.
    ///
    /// Also cursor position is restored to original position after print.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kdam::term::Writer;
    ///
    /// Writer::Stderr.print_at(1, format!("1 + 1 = {}", 2));
    /// ```
    pub fn print_at<T: Into<String>>(&self, position: usize, text: T) {
        match self {
            Self::Stderr => {
                let mut writer = std::io::stderr();

                crate::lock::acquire();

                if position > 0 {
                    writer
                        .write_fmt(format_args!(
                            "{}{}\x1b[{}A",
                            "\n".repeat(position),
                            text.into(),
                            position
                        ))
                        .unwrap();
                } else {
                    writer.write_fmt(format_args!("{}", text.into())).unwrap();
                }

                writer.flush().unwrap();
                crate::lock::release();
            }
            Self::Stdout => {
                let mut writer = std::io::stdout();

                crate::lock::acquire();

                if position > 0 {
                    writer
                        .write_fmt(format_args!(
                            "{}{}\x1b[{}A",
                            "\n".repeat(position),
                            text.into(),
                            position
                        ))
                        .unwrap();
                } else {
                    writer.write_fmt(format_args!("{}", text.into())).unwrap();
                }

                writer.flush().unwrap();
                crate::lock::release();
            }
        }
    }
}

/// Get number of columns in current window or default to specified value.
pub fn get_columns_or(width: u16) -> u16 {
    terminal_size::terminal_size()
        .unwrap_or((terminal_size::Width(width), terminal_size::Height(0)))
        .0
         .0
}

/// Get length of string.
pub fn string_length(mut text: String) -> usize {
    while let Some(start) = text.find("\x1b[") {
        text = text.replace(&text[start..(start + text[start..].find("m").unwrap() + 1)], "");
    }

    text.chars().count()
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

    let mut color = colour_code.to_uppercase();
    let mut code = "\x1b[".to_string();

    let bg = if color.contains("ON BLACK") {
        color = color.replace("ON BLACK", "");
        ";40"
    } else if color.contains("ON RED") {
        color = color.replace("ON RED", "");
        ";41"
    } else if color.contains("ON GREEN") {
        color = color.replace("ON GREEN", "");
        ";42"
    } else if color.contains("ON YELLOW") {
        color = color.replace("ON YELLOW", "");
        ";43"
    } else if color.contains("ON BLUE") {
        color = color.replace("ON BLUE", "");
        ";44"
    } else if color.contains("ON MAGENTA") {
        color = color.replace("ON MAGENTA", "");
        ";45"
    } else if color.contains("ON CYAN") {
        color = color.replace("ON CYAN", "");
        ";46"
    } else if color.contains("ON WHITE") {
        color = color.replace("ON WHITE", "");
        ";47"
    } else {
        ""
    };

    // rgb(175,0,255)
    if let Some(hex_code) = colour_code.find("#") {
        code += &format!(
            "38;2;{};{};{}",
            i16::from_str_radix(&colour_code[(hex_code + 1)..(hex_code + 3)], 16).unwrap(),
            i16::from_str_radix(&colour_code[(hex_code + 3)..(hex_code + 5)], 16).unwrap(),
            i16::from_str_radix(&colour_code[(hex_code + 5)..(hex_code + 7)], 16).unwrap()
        );
    } else {
        if color.contains("BRIGHT") {
            if color.contains("BLACK") {
                code += "90";
            } else if color.contains("RED") {
                code += "91";
            } else if color.contains("GREEN") {
                code += "92";
            } else if color.contains("YELLOW") {
                code += "93";
            } else if color.contains("BLUE") {
                code += "94";
            } else if color.contains("MAGENTA") {
                code += "95";
            } else if color.contains("CYAN") {
                code += "96";
            } else if color.contains("WHITE") {
                code += "97";
            } else {
                return "".to_string();
            }
        } else {
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
        }
    }

    code += bg;

    if color.contains("BOLD") {
        code += ";1";
    }

    if color.contains("DIM") {
        code += ";2";
    }

    if color.contains("ITALIC") {
        code += ";3";
    }

    if color.contains("UNDERLINE") {
        code += ";4";
    }

    if color.contains("BLINK") {
        code += ";5";
    }

    if color.contains("REVERSED") {
        code += ";7";
    }

    if color.contains("HIDDEN") {
        code += ";8";
    }

    if color.contains("STRIKETHROUGH") {
        code += ";9";
    }

    code += "m";
    code
}

/// Add colour escape codes to the given text for printing coloured text in terminal.
/// This trait is only implemented for `&str`.
pub trait Colorizer {
    /// Add colour escape codes to the given text.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kdam::prelude::*;
    ///
    /// println!("{}", "hello world!".colorize("bold red"));
    /// println!("{}", "hello world!".colorize("bright white on blue"));
    /// ```
    fn colorize<'a>(&'a self, code: &str) -> String;
}

impl Colorizer for str {
    fn colorize(&self, code: &str) -> String {
        let esc_code = colour(code);

        if esc_code == "" {
            self.to_owned()
        } else {
            esc_code + self + "\x1b[0m"
        }
    }
}
