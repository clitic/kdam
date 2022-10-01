/// Comman progress bar functionalities shared between different types of progress bars.
pub trait BarExt {
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

#[macro_export]
#[doc(hidden)]
macro_rules! _impl_bar_methods {
    ($struct: ident, $render: ident) => {
        impl $crate::BarExt for $struct {
            fn clear(&mut self) {
                self.pb.clear();
            }

            fn input<T: Into<String>>(&mut self, text: T) -> Result<String, std::io::Error> {
                self.clear();
                self.pb.get_writer().print_str(&text.into());

                let mut input_string = String::new();
                std::io::stdin().read_line(&mut input_string)?;

                if self.pb.get_leave() {
                    self.refresh();
                }

                Ok(input_string)
            }

            fn refresh(&mut self) {
                if !self.pb.get_force_refresh() {
                    self.pb.set_force_refresh(true);
                    self.update(0);
                    self.pb.set_force_refresh(false);
                } else {
                    self.update(0);
                }
            }

            fn render(&mut self) -> String {
                $render(self)
            }

            fn reset(&mut self, total: Option<usize>) {
                self.pb.reset(total);
            }

            fn update(&mut self, n: usize) {
                if self.pb.trigger(n) {
                    let text = self.render();
                    let length = $crate::term::Colorizer::len_ansi(text.as_str()) as i16;

                    if length != self.pb.get_bar_length() {
                        self.pb.clear();
                    }

                    self.pb.set_bar_length(length);
                    self.pb.write_at(text);
                }
            }

            fn update_to(&mut self, update_to_n: usize) {
                self.pb.set_counter(update_to_n);
                self.update(0);
            }

            fn write<T: Into<String>>(&mut self, text: T) {
                self.pb.clear();
                self.pb
                    .get_writer()
                    .print(format_args!("\r{}\n", text.into()));

                if self.pb.get_leave() {
                    self.refresh();
                }
            }
        }
    };
}
