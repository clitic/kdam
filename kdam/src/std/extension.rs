use std::io::{Result, Write};

/// Comman progress bar functionalities shared between different types of progress bars.
pub trait BarExt {
    /// Clear current progress bar display.
    fn clear(&mut self) -> Result<()>;

    /// Take input via progress bar (without overlaping with bar(s)).
    fn input<T: Into<String>>(&mut self, text: T) -> Result<String>;

    /// Force refresh current progress bar display.
    fn refresh(&mut self) -> Result<()>;

    /// Render progress bar text.
    fn render(&mut self) -> String;

    /// Resets counter to 0 for repeated use.
    /// 
    /// Consider combining with `leave = true`.
    fn reset(&mut self, total: Option<usize>);

    /// Manually update the progress bar, useful for streams such as reading files.
    /// 
    /// Returns whether an update was triggered or not depending on constraints.
    fn update(&mut self, n: usize) -> Result<bool>;

    /// Set counter value instead of incrementing counter through [update](Self::update) method.
    /// 
    /// Returns wheter a update was triggered or not depending on constraints.
    fn update_to(&mut self, n: usize) -> Result<bool>;

    /// Print a message via progress bar (without overlaping with bar(s)).
    fn write<T: Into<String>>(&mut self, text: T) -> Result<()>;

    /// Write progress bar rendered text to a writer (useful for writing files).
    /// 
    /// If `n` is supplied then this method behaves like [update](Self::update) method.
    /// 
    /// Returns whether a update was triggered or not depending on constraints.
    ///
    /// # Example
    ///
    /// Using [write_to](Self::write_to) as [update_to](Self::update_to).
    ///
    /// ```
    /// use kdam::{tqdm, BarExt};
    /// use std::{fs::File, io::Write};
    ///
    /// let mut pb = tqdm!(total = 100);
    /// let mut f = File::create("kdam-logs.txt").unwrap();
    /// 
    /// for i in 1..101 {
    ///     pb.counter = i;
    ///     pb.write_to(&mut f, Some(0));
    /// }
    /// ```
    fn write_to<T: Write>(&mut self, writer: &mut T, n: Option<usize>) -> Result<bool>;
}
