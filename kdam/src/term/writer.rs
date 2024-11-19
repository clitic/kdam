use std::{
    fs::{File, OpenOptions},
    io::{stderr, stdout, Result, Write},
};

/// Stderr and Stdout writer.
#[derive(Debug, Clone)]
pub enum Writer {
    Stderr,
    Stdout,
    Tty,
}

impl Writer {
    pub fn init(&self) -> InitializedOutput {
        match self {
            Writer::Stderr => InitializedOutput::Stderr,
            Writer::Stdout => InitializedOutput::Stdout,
            Writer::Tty => match OpenOptions::new().append(true).open(TTY_PATH) {
                Ok(a) => InitializedOutput::Tty(a),
                Err(_) => InitializedOutput::Null,
            },
        }
    }
}

#[cfg(target_os = "windows")]
const TTY_PATH: &str = "CON";

#[cfg(not(target_os = "windows"))]
const TTY_PATH: &str = "/dev/tty";

pub enum InitializedOutput {
    Stderr,
    Stdout,
    Tty(File),
    Null,
}

impl std::fmt::Debug for InitializedOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitializedOutput::Stderr => write!(f, "Stderr"),
            InitializedOutput::Stdout => write!(f, "Stdout"),
            InitializedOutput::Tty(_) => write!(f, "TTY"),
            InitializedOutput::Null => write!(f, "Null"),
        }
    }
}

struct NullWriter;

impl std::io::Write for NullWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl InitializedOutput {
    fn lock(&mut self) -> Box<dyn Write + '_> {
        match self {
            Self::Stderr => Box::new(stderr().lock()),
            Self::Stdout => Box::new(stdout().lock()),
            Self::Tty(f) => Box::new(f),
            InitializedOutput::Null => Box::new(NullWriter),
        }
    }

    /// Print text buffer in terminal followed by a flush.
    pub fn print(&mut self, buf: &[u8]) -> Result<()> {
        let mut writer: Box<dyn Write> = self.lock();
        writer.write_all(buf)?;
        writer.flush()?;
        Ok(())
    }

    /// Print text buffer in terminal followed by a flush at specified position.
    ///
    /// # Note
    ///
    /// Cursor position is restored to original position after buffer is printed.
    ///
    /// # Example
    ///
    /// ```
    /// use kdam::term::Writer;
    ///
    /// Writer::Stderr.init().print_at(1, format!("1 + 1 = {}", 2).as_bytes()).unwrap();
    /// ```
    pub fn print_at(&mut self, position: u16, buf: &[u8]) -> Result<()> {
        let mut writer: Box<dyn Write> = self.lock();

        if position > 0 {
            writer.write_all("\n".repeat(position as usize).as_bytes())?;
            writer.write_all(buf)?;
            writer.write_fmt(format_args!("\x1b[{}A", position))?;
        } else {
            writer.write_all(&[b'\r'])?;
            writer.write_all(buf)?;
        }

        writer.flush()?;
        Ok(())
    }
}
