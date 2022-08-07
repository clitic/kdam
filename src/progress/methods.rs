/// Comman progress bar functionalities shared between different types of progress bars.
pub trait BarMethods {
    /// Clear current bar display.
    fn clear(&mut self);

    /// Take input via bar (without overlap with bars).
    fn input<T: Into<String>>(&mut self, text: T) -> Result<String, std::io::Error>;

    /// Force refresh the display of this bar.
    fn refresh(&mut self);

    /// Render progress bar.
    fn render(&mut self) -> String;

    /// Resets to intial iterations for repeated use.
    /// Consider combining with `leave=true`.
    fn reset(&mut self, total: Option<usize>);

    /// Manually update the progress bar, useful for streams such as reading files.
    fn update(&mut self, n: usize);

    /// Set counter position instead of incrementing progress bar through `self.update`.
    /// Alternative way to update bar.
    fn update_to(&mut self, update_to_n: usize);

    /// Print a message via bar (without overlap with bars).
    fn write<T: Into<String>>(&mut self, text: T);
}
