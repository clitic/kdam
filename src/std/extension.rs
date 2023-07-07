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
    /// Returns wheter a update was triggered or not depending on constraints.
    fn update(&mut self, n: usize) -> bool;

    /// Set counter position instead of incrementing progress bar through [update](Self::update).
    /// Alternative way to update bar.
    /// Returns wheter a update was triggered or not depending on constraints.
    fn update_to(&mut self, update_to_n: usize) -> bool;

    /// Print a message via bar (without overlap with bars).
    fn write<T: Into<String>>(&mut self, text: T);

    /// Write rendered text to a writer, useful for writing files.
    /// If `n` is supplied then this method behaves like [update](Self::update).
    /// Returns wheter a update was triggered or not depending on constraints.
    ///
    /// # Example
    ///
    /// Using [write_to](Self::write_to) as [update_to](Self::update_to).
    ///
    /// ```
    /// use kdam::{tqdm, BarExt};
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut pb = tqdm!(total = 100);
    /// let mut f = File::create("logs.txt").unwrap();
    /// 
    /// for i in 1..101 {
    ///     pb.set_counter(i);
    ///     pb.write_to(&mut f, Some(0));
    /// }
    /// ```
    fn write_to<T: std::io::Write>(&mut self, writer: &mut T, n: Option<usize>) -> bool;
}

/// Derive [BarExt](crate::std::BarExt) trait for a struct.
/// 
/// # Example
/// 
/// ```
/// use kdam::{derive_bar_ext, Bar};
/// 
/// struct CustomBar {
///     pb: Bar, // Required
/// }
/// 
/// fn render(progress: &mut CustomBar) -> String {
///     format!(
///         "Progress: {}/{}",
///         progress.pb.fmt_counter(),
///         progress.pb.fmt_total(),
///     )
/// }
/// 
/// derive_bar_ext!(CustomBar, render);
/// ```
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
#[macro_export]
macro_rules! derive_bar_ext {
    ($struct: ident, $render: ident) => {
        impl $crate::BarExt for $struct {
            fn clear(&mut self) {
                self.pb.clear();
            }

            fn input<T: Into<String>>(&mut self, text: T) -> Result<String, std::io::Error> {
                self.clear();
                self.pb.writer.print_str(&text.into());

                let mut input_string = String::new();
                std::io::stdin().read_line(&mut input_string)?;

                if self.pb.leave {
                    self.refresh();
                }

                Ok(input_string)
            }

            fn refresh(&mut self) {
                if !self.pb.force_refresh {
                    self.pb.force_refresh = true;
                    self.update(0);
                    self.pb.force_refresh = false;
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

            fn update(&mut self, n: usize) -> bool {
                if self.pb.trigger(n) {
                    let text = self.render();
                    let length = $crate::term::Colorizer::len_ansi(text.as_str()) as i16;

                    if length != self.pb.get_bar_length() {
                        self.pb.clear();
                    }

                    self.pb.set_bar_length(length);
                    self.pb.write_at(text);
                    return true;
                }

                false
            }

            fn update_to(&mut self, update_to_n: usize) -> bool {
                self.pb.set_counter(update_to_n);
                self.update(0)
            }

            fn write<T: Into<String>>(&mut self, text: T) {
                self.pb.clear();
                self.pb.writer.print(format_args!("\r{}\n", text.into()));

                if self.pb.leave {
                    self.refresh();
                }
            }

            fn write_to<T: std::io::Write>(&mut self, writer: &mut T, n: Option<usize>) -> bool {
                let text;

                if let Some(n) = &n {
                    if self.pb.trigger(*n) {
                        text = $crate::term::Colorizer::trim_ansi(self.render().as_str());
                    } else {
                        return false;
                    }
                } else {
                    text = $crate::term::Colorizer::trim_ansi(self.render().as_str());
                }

                self.pb
                    .set_bar_length($crate::term::Colorizer::len_ansi(text.as_str()) as i16);
                $crate::lock::acquire();
                writer.write_fmt(format_args!("{}\n", text)).unwrap();
                writer.flush().unwrap();
                $crate::lock::release();
                true
            }
        }
    };
}
