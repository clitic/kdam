use std::io::Write;

use crate::format;
use crate::styles::Animation;
use crate::term;
use crate::term::Writer;

/// Standard struct implemention of progress bar.
///
/// # Examples
///
/// A clean nice progress bar with a total value.
///
/// ```rust
/// use kdam::Bar;
///
/// fn main() {
///     let mut pb = Bar::new(100);
///
///     for _ in 0..100 {
///         pb.update(1);
///     }
/// }
/// ```
///
/// If total is unknown then `kdam::Bar` could be constructed with total as 0 i.e. `Bar::new(0)`
#[derive(Debug)]
pub struct Bar {
    /// Prefix for the progress bar.
    /// (default: `""`)
    pub desc: String,
    /// The number of expected iterations.
    /// If unspecified, iterable.size_hint().0 is used if possible.
    /// If 0, only basic progress statistics are displayed (no ETA, no progressbar).
    /// (default: `0`)
    pub total: usize,
    /// If true, keeps all traces of the progressbar upon termination of iteration.
    /// If false, will leave only if position is 0.
    /// (default: `true`)
    pub leave: bool,
    /// Specifies where to output the progress messages (default: stderr).
    /// Uses file.write_fmt and file.flush methods.
    /// (default: `None`)
    pub file: Option<std::fs::File>,
    /// The width of the entire output message.
    /// If specified, dynamically resizes the progressbar to stay within this bound.
    /// If unspecified, attempts to use environment width.
    /// The fallback is a meter width of 10 and no limit for the counter and statistics.
    /// If 0, will not print any meter (only stats).
    /// (default: `10`)
    pub ncols: i16,
    /// Minimum progress display update interval (in seconds).
    /// (default: `0.1`)
    pub mininterval: f32,
    /// Minimum progress display update interval, in iterations.
    /// If > 0, will skip display of specified number of iterations. Tweak this and mininterval to get very efficient loops.
    /// If your progress is erratic with both fast and slow iterations (network, skipping items, etc) you should set miniters=1.
    /// (default: `1`)
    pub miniters: usize,
    /// Automatically adjusts miniters to correspond to mininterval after long display update lag.
    /// (default: `false`)
    pub dynamic_miniters: bool,
    /// Whether to disable the entire progress bar wrapper.
    /// (default: `false`)
    pub disable: bool,
    /// String that will be used to define the unit of each iteration.
    /// (default: `"it"`)
    pub unit: String,
    /// If true, the number of iterations will be reduced/scaled automatically
    /// and a metric prefix following the International System of Units standard will be added (kilo, mega, etc.).
    /// (default: `false`)
    pub unit_scale: bool,
    /// If true, constantly alters ncols to the environment (allowing for window resizes).
    /// (default: `false`)
    pub dynamic_ncols: bool,
    /// Specify the line offset to print this bar (starting from 0).
    /// Useful to manage multiple bars at once (eg, from threads).
    /// (default: `0`)
    pub position: u8,
    /// Specify additional stats to display at the end of the bar.
    /// (default: `""`)
    pub postfix: String,
    /// ignored unless unit_scale is true.
    /// (default: `1024`)
    pub unit_divisor: usize,
    /// Bar colour (e.g. "green", "#00ff00").
    pub colour: String,
    /// Don't display until few seconds have elapsed.
    /// (default: `0`)
    pub delay: f32,
    /// Defines the animation style to use with progress bar.
    /// For custom type use set_charset method.
    /// (default: `kdam::Animation::TqdmAscii`)
    pub animation: Animation,
    /// Select writer type to display progress bar output between stdout and stderr.
    /// (default: `kdam::Output::Stderr`)
    pub writer: Writer,
    /// If true, each update method call will be rendered.
    /// (default: `false`)
    pub force_refresh: bool,
    /// If true, progress bar of more length than terminal will be trimmed at end.
    /// (default: `false`)
    pub wrap: bool,
    pub(crate) n: usize,
    started: bool,
    timer: std::time::Instant,
    pub(crate) elapsed_time: f32,
    user_ncols: Option<i16>,
    pub(crate) bar_length: i16,
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            desc: "".to_string(),
            total: 0,
            leave: true,
            file: None,
            ncols: 10,
            mininterval: 0.1,
            miniters: 1,
            dynamic_miniters: false,
            disable: false,
            unit: "it".to_string(),
            unit_scale: false,
            dynamic_ncols: false,
            position: 0,
            postfix: "".to_string(),
            unit_divisor: 1000,
            colour: "default".to_string(),
            delay: 0.0,
            animation: Animation::Tqdm,
            writer: Writer::Stderr,
            force_refresh: false,
            wrap: false,
            n: 0,
            started: false,
            timer: std::time::Instant::now(),
            elapsed_time: 0.0,
            user_ncols: None,
            bar_length: 0,
        }
    }
}

impl Bar {
    /// Create a new instance of `kdam::Bar` with a total value.
    /// You can also set `total=0` if total is unknown.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut pb = kdam::Bar::new(100);
    /// ```
    pub fn new(total: usize) -> Self {
        Self {
            total: total,
            ..Default::default()
        }
    }

    /// Set and returns elapsed time of progress bar.
    pub fn bar_elapsed_time(&mut self) -> f32 {
        self.elapsed_time = self.timer.elapsed().as_secs_f32();
        self.elapsed_time
    }

    /// Returns progress percentage, like 0.62, 0.262
    pub fn bar_percentage(&self) -> f64 {
        self.n as f64 / self.total as f64
    }

    /// Returns rate / iterations of update method calls.
    pub fn bar_rate(&self) -> f32 {
        self.n as f32 / self.elapsed_time
    }

    /// Returns remaining time / ETA of progress completion.
    pub fn bar_remaining_time(&self) -> f32 {
        if self.total != 0 {
            (self.total - self.n) as f32 / self.bar_rate()
        } else {
            f32::INFINITY
        }
    }

    /// Format self.n
    pub fn bar_fmt_count(&self) -> String {
        let count = if self.unit_scale {
            format::format_sizeof(self.n, self.unit_divisor)
        } else {
            format!("{}", self.n)
        };

        if self.unit_divisor == 1024 {
            format!("{}{}", count, self.unit)
        } else {
            count
        }
    }

    /// Format elapsed time of progress bar.
    pub fn bar_fmt_elapsed_time(&self) -> String {
        format::format_interval(self.elapsed_time as usize)
    }

    /// Format pogress percentage.
    pub fn bar_fmt_percentage(&self, precision: usize) -> String {
        format!(
            "{:#1$.2$}%",
            self.bar_percentage() * 100.0,
            precision + 3,
            precision
        )
    }

    /// Format rate / iterations of update method calls.
    pub fn bar_fmt_rate(&self) -> String {
        let rate = if self.unit_scale {
            format::format_sizeof(self.bar_rate() as usize, self.unit_divisor)
        } else {
            format!("{:.2}", self.bar_rate())
        };

        if self.n == 0 {
            format!("?{}/s", self.unit)
        } else {
            format!("{}{}/s", rate, self.unit)
        }
    }

    /// Format remaining time / ETA of progress completion.
    pub fn bar_fmt_remaining_time(&self) -> String {
        if self.n == 0 {
            "00:00".to_string()
        } else {
            format::format_interval(self.bar_remaining_time() as usize)
        }
    }

    /// Format self.total
    pub fn bar_fmt_total(&self) -> String {
        let total = if self.unit_scale {
            format::format_sizeof(self.total, self.unit_divisor)
        } else {
            format!("{}", self.total)
        };

        if self.unit_divisor == 1024 {
            format!("{}{}", total, self.unit)
        } else {
            total
        }
    }

    /// Returns counter value
    pub fn counter(&self) -> usize {
        self.n
    }

    /// Set/Modify colour of the progress bar.
    pub fn set_colour(&mut self, colour: &str) {
        if colour != "default" {
            self.colour = term::colour(colour);
        } else {
            self.colour = "default".to_string();
        }
    }

    /// Set/Modify description of the progress bar.
    pub fn set_description(&mut self, desc: &str) {
        self.desc = desc.to_owned();
    }

    /// Set/Modify postfix (additional stats) with automatic formatting based on datatype.
    pub fn set_postfix(&mut self, postfix: &str) {
        self.postfix = ", ".to_owned() + postfix;
    }

    /// Returns wheter bar is started or not.
    pub fn started(&self) -> bool {
        self.started
    }

    /// Checks wheter to trigger a display update or not.
    /// This method will increment `self.n`.
    pub fn trigger(&mut self, n: usize) -> bool {
        self.n += n;

        if !self.disable {
            if self.force_refresh {
                return true;
            }

            let completion_constraint = self.n == self.total;

            let elapsed_time_now = self.timer.elapsed().as_secs_f32();
            let delay_constraint = self.delay <= elapsed_time_now;
            let mininterval_constraint = self.mininterval <= (elapsed_time_now - self.elapsed_time);

            if self.dynamic_miniters && !mininterval_constraint {
                self.miniters += self.n;
            }

            let miniters_constraint = if self.miniters <= 1 {
                true
            } else {
                self.n % self.miniters == 0
            };

            if (mininterval_constraint && miniters_constraint && delay_constraint)
                || completion_constraint
            {
                if self.dynamic_miniters {
                    self.miniters = 0;
                }

                return true;
            }
        }

        false
    }

    /// Intialize some values and starts the timer.
    pub(crate) fn init(&mut self) {
        if !self.started {
            self.set_colour(&self.colour.clone());

            if self.ncols != 10 {
                self.user_ncols = Some(self.ncols);
            }

            self.timer = std::time::Instant::now();
            self.started = true;
        }
    }

    /// Adjust number of columns for bar animation using length of remanining bar.
    pub(crate) fn set_ncols(&mut self, lbar_rbar_len: i16) {
        if self.dynamic_ncols || (lbar_rbar_len + self.ncols != self.bar_length) {
            if self.user_ncols.is_some() {
                self.ncols = self.user_ncols.unwrap();
            } else {
                let columns = term::get_columns();

                if columns != 0 {
                    let new_ncols = columns as i16 - lbar_rbar_len;
                    if new_ncols >= 0 {
                        self.ncols = new_ncols;
                    }
                } else {
                    self.ncols = 10;

                    if !self.dynamic_ncols {
                        self.user_ncols = Some(10);
                    }
                }
            }
        }
    }

    /// Print a string in position of bar.
    pub(crate) fn write_at(&self, mut text: String) {
        if self.wrap {
            let columns = crate::term::get_columns() as usize;

            if self.bar_length as usize > columns {
                text = text[..columns].to_string();
            }
        }

        crate::lock::acquire();

        if self.file.is_some() {
            let mut file = self.file.as_ref().unwrap();
            file.write_fmt(format_args!("{}\n", text.as_str())).unwrap();
            file.flush().unwrap();
        } else {
            if self.position == 0 {
                self.writer.print(format_args!("\r{}", text));
            } else {
                self.writer.print(format_args!(
                    "{}{}{}",
                    "\n".repeat(self.position as usize),
                    text,
                    format!("\x1b[{}A", self.position)
                ));
            }
        }

        crate::lock::release();
    }
}

pub trait BarMethods {
    /// Clear current bar display.
    fn clear(&mut self);

    /// Take input via bar (without overlap with bars).
    fn input(&mut self, text: &str) -> Result<String, std::io::Error>;

    /// Force refresh the display of this bar.
    fn refresh(&mut self);

    /// Render progress bar.
    fn render(&mut self) -> String;

    /// Manually update the progress bar, useful for streams such as reading files.
    fn update(&mut self, n: usize);

    /// Set counter position instead of incrementing progress bar through `self.update`.
    /// Alternative way to update bar.
    fn update_to(&mut self, update_to_n: usize);

    /// Print a message via bar (without overlap with bars).
    fn write(&mut self, text: &str);

    // /// Resets to intial iterations for repeated use.
    // /// Consider combining with `leave=true`.
    // pub fn reset(&mut self, total: Option<usize>) {
    //     self.started = false;

    //     if total.is_some() {
    //         self.total = total.unwrap();
    //     }
    // }
}

impl BarMethods for Bar {
    fn clear(&mut self) {
        if self.file.is_none() {
            let mut columns = term::get_columns() as usize;

            if columns == 0 {
                columns = self.bar_length as usize;
            }

            self.writer
                .print(format_args!("\r{}\r", " ".repeat(columns)));
        }
    }

    fn input(&mut self, text: &str) -> Result<String, std::io::Error> {
        self.clear();
        self.writer.print_str(text);

        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string)?;

        if self.leave {
            self.refresh();
        }

        Ok(input_string)
    }

    fn refresh(&mut self) {
        if !self.force_refresh {
            self.force_refresh = true;
            self.update(0);
            self.force_refresh = false;
        } else {
            self.update(0);
        }
    }

    fn render(&mut self) -> String {
        self.bar_elapsed_time();

        let desc = if self.desc == "" {
            "".to_string()
        } else {
            format!("{}: ", self.desc)
        };

        if self.total == 0 {
            let bar = format!(
                "{}{}{} [{}, {}{}]",
                desc,
                self.bar_fmt_count(),
                self.unit,
                self.bar_fmt_elapsed_time(),
                self.bar_fmt_rate(),
                self.postfix
            );

            self.bar_length = bar.chars().count() as i16;
            return bar;
        }

        let progress = self.bar_percentage() as f32;

        if progress >= 1.0 {
            self.total = self.n;

            if !self.leave {
                return format!("{}\r", " ".repeat(self.bar_length as usize));
            }
        }

        let lbar = desc + &self.bar_fmt_percentage(0);
        let rbar = format!(
            " {}/{} [{}<{}, {}{}]",
            self.bar_fmt_count(),
            self.bar_fmt_total(),
            self.bar_fmt_elapsed_time(),
            self.bar_fmt_remaining_time(),
            self.bar_fmt_rate(),
            self.postfix,
        );

        let lbar_rbar_len =
            (lbar.chars().count() + rbar.chars().count() + self.animation.spaces() as usize) as i16;
        self.set_ncols(lbar_rbar_len);

        if self.ncols <= 0 {
            return lbar + &rbar;
        } else {
            self.bar_length = lbar_rbar_len + self.ncols;
        }

        lbar + &self
            .animation
            .fmt_progress(progress, self.ncols.clone(), &self.colour)
            + &rbar
    }

    fn update(&mut self, n: usize) {
        self.init();

        if self.trigger(n) {
            let text = self.render();
            self.write_at(text);
        }
    }

    fn update_to(&mut self, update_to_n: usize) {
        self.n = update_to_n;
        self.update(0);
    }

    fn write(&mut self, text: &str) {
        self.clear();
        self.writer.print(format_args!("{}\n", text));

        if self.leave {
            self.refresh();
        }
    }
}
