use std::io::Write;
use crate::lock;

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
                writer.write_all(text.as_bytes()).unwrap();
                writer.flush().unwrap();
            }
            Self::Stdout => {
                let mut writer = std::io::stdout();
                writer.write_all(text.as_bytes()).unwrap();
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
    /// ```
    /// use kdam::term::Writer;
    ///
    /// Writer::Stderr.print_at(1, format!("1 + 1 = {}", 2));
    /// ```
    pub fn print_at<T: Into<String>>(&self, position: usize, text: T) {
        match self {
            Self::Stderr => {
                let mut writer = std::io::stderr();

                lock::acquire();

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
                lock::release();
            }
            Self::Stdout => {
                let mut writer = std::io::stdout();

                lock::acquire();

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
                lock::release();
            }
        }
    }
}
