use crate::lock;
use std::io::{stderr, stdout, Result, Write};

/// Stderr and Stdout writer.
#[derive(Debug, Clone)]
pub enum Writer {
    Stderr,
    Stdout,
}

impl Writer {
    /// Print text buffer in terminal followed by a flush.
    pub fn print(&self, buf: &[u8]) -> Result<()> {
        let mut writer: Box<dyn Write> = match self {
            Self::Stderr => Box::new(stderr()),
            Self::Stdout => Box::new(stdout()),
        };

        lock::acquire();
        writer.write_all(buf)?;
        writer.flush()?;
        lock::release();

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
    /// Writer::Stderr.print_at(1, format!("1 + 1 = {}", 2).as_bytes()).unwrap();
    /// ```
    pub fn print_at(&self, position: u16, buf: &[u8]) -> Result<()> {
        let mut writer: Box<dyn Write> = match self {
            Self::Stderr => Box::new(stderr()),
            Self::Stdout => Box::new(stdout()),
        };

        lock::acquire();

        if position > 0 {
            writer.write_all("\n".repeat(position as usize).as_bytes())?;
            writer.write_all(buf)?;
            writer.write_fmt(format_args!("\x1b[{}A", position))?;
        } else {
            writer.write_all(&[b'\r'])?;
            writer.write_all(buf)?;
        }

        writer.flush()?;
        lock::release();

        Ok(())
    }
}
