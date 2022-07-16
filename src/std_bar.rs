use std::io::Write;

use crate::format;
use crate::styles::Animation;
use crate::term;
use crate::term::{Colorizer, Writer};

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
    pub max_fps: bool,
    /// If true, progress bar of more length than terminal will be trimmed at end.
    /// (default: `false`)
    pub wrap: bool,
    /// Counter of progress bar.
    /// (default: `0`)
    pub n: usize,
    pub(crate) started: bool,
    pub timer: std::time::Instant,
    pub elapsed_time: f32,
    pub user_ncols: Option<i16>,
    pub bar_length: i16,
    pub force_refresh: bool,
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
            max_fps: false,
            wrap: false,
            n: 0,
            started: false,
            timer: std::time::Instant::now(),
            elapsed_time: 0.0,
            user_ncols: None,
            bar_length: 0,
            force_refresh: false,
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

    /// Checks wheter to trigger a display update or not.
    /// This method will increment self.n
    pub fn trigger(&mut self, n: usize) -> bool {
        if !self.started {
            self.set_colour(&self.colour.clone());

            if self.ncols != 10 {
                self.user_ncols = Some(self.ncols);
            }

            if self.max_fps {
                self.force_refresh = true;
            }

            self.timer = std::time::Instant::now();
            self.started = true;
        }

        self.n += n;

        if !self.disable {
            if self.force_refresh {
                return true;
            }

            let elapsed_time_now = self.timer.elapsed().as_secs_f32();
            let mininterval_constraint = self.mininterval <= (elapsed_time_now - self.elapsed_time);

            if self.dynamic_miniters && !mininterval_constraint {
                self.miniters += n;
            }

            let miniters_constraint;

            if self.miniters <= 1 {
                miniters_constraint = true;
            } else {
                miniters_constraint = self.n % self.miniters == 0;
            }

            if (mininterval_constraint && miniters_constraint && (self.delay <= elapsed_time_now))
                || self.n == self.total
            {
                if self.dynamic_miniters {
                    self.miniters = 0;
                }

                return true;
            }
        }

        return false;
    }

    /// Set and returns elapsed time of progress bar.
    pub fn elapsed_time(&mut self) -> f32 {
        self.elapsed_time = self.timer.elapsed().as_secs_f32();
        self.elapsed_time
    }

    /// Format elapsed time of progress bar.
    pub fn elapsed_time_fmt(&self) -> String {
        format::format_interval(self.elapsed_time as usize)
    }

    /// Returns rate/iterations of update method calls.
    pub fn rate(&self) -> f32 {
        self.n as f32 / self.elapsed_time
    }

    /// Format rate/iterations of update method calls.
    pub fn rate_fmt(&self) -> String {
        let rate = if self.unit_scale {
            format::format_sizeof(self.rate() as usize, self.unit_divisor)
        } else {
            format!("{:.2}", self.rate())
        };

        if self.n == 0 {
            format!("?{}/s", self.unit)
        } else {
            format!("{}{}/s", rate, self.unit)
        }
    }

    /// Returns ETA / remaining time of progress completion.
    pub fn eta(&self) -> f32 {
        if self.total != 0 {
            (self.total - self.n) as f32 / self.rate()
        } else {
            0.0
        }
    }

    /// Format remaining time / ETA of progress completion.
    pub fn eta_fmt(&self) -> String {
        if self.n == 0 {
            "00:00".to_string()
        } else {
            format::format_interval(self.eta() as usize)
        }
    }

    /// Returns progress percentage, like 0.62, 0.262
    pub fn percentage(&self) -> f64 {
        self.n as f64 / self.total as f64
    }

    /// Format pogress percentage.
    pub fn percentage_fmt(&self, precision: usize) -> String {
        format!(
            "{:#1$.2$}%",
            self.percentage() * 100.0,
            precision + 3,
            precision
        )
    }

    /// Format self.n
    pub fn count_fmt(&self) -> String {
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

    /// Format self.total
    pub fn total_fmt(&self) -> String {
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

    /// Render progress bar.
    fn render(&mut self) -> String {
        self.elapsed_time();

        let desc = if self.desc == "" {
            "".to_string()
        } else {
            format!("{}: ", self.desc)
        };

        if self.total == 0 {
            let count = if self.unit_scale {
                format::format_sizeof(self.n, self.unit_divisor)
            } else {
                format!("{}", self.n)
            };

            let bar = format!(
                "{}{}{} [{}, {}{}]",
                desc,
                count,
                self.unit,
                self.elapsed_time_fmt(),
                self.rate_fmt(),
                self.postfix
            );

            self.bar_length = bar.chars().count() as i16;
            return bar;
        }

        let progress = self.percentage() as f32;

        if progress >= 1.0 {
            self.total = self.n;

            if !self.leave {
                return format!("{}\r", " ".repeat(self.bar_length as usize));
            }
        }

        let lbar = format!("{}{}", desc, self.percentage_fmt(0));

        let rbar = format!(
            " {}/{} [{}<{}, {}{}]",
            self.count_fmt(),
            self.total_fmt(),
            self.elapsed_time_fmt(),
            self.eta_fmt(),
            self.rate_fmt(),
            self.postfix,
        );

        let spaces = match self.animation {
            Animation::FiraCode => 3,
            _ => 2,
        };

        let lbar_rbar_len = (lbar.chars().count() + rbar.chars().count() + spaces) as i16;
        self.set_ncols(lbar_rbar_len);

        if self.ncols <= 0 {
            return format!("{}{}", lbar, rbar);
        } else {
            self.bar_length = lbar_rbar_len + self.ncols;
        }

        let (bar_open, bar_close, bar_animation, mbar);

        match self.animation {
            Animation::Tqdm | Animation::TqdmAscii | Animation::FillUp | Animation::Custom(_) => {
                (bar_open, bar_close) = ("|", "|");
                bar_animation = crate::styles::progressive(
                    progress,
                    self.ncols.clone(),
                    self.animation.clone(),
                );
            }

            Animation::Classic | Animation::Arrow => {
                (bar_open, bar_close) = ("[", "]");
                bar_animation =
                    crate::styles::simple(progress, self.ncols.clone(), self.animation.clone());
            }

            Animation::FiraCode => {
                (bar_open, bar_close) = (" ", "");
                bar_animation =
                    crate::styles::simple(progress, self.ncols.clone(), self.animation.clone());
            }
        }

        if self.colour == "default" {
            mbar = format!("{}{}{}", bar_open, bar_animation, bar_close);
        } else {
            mbar = format!(
                "{}{}{}",
                bar_open,
                bar_animation.colorize(&self.colour),
                bar_close
            );
        }

        format!("{}{}{}", lbar, mbar, rbar)
    }

    /// Manually update the progress bar, useful for streams such as reading files.
    pub fn update(&mut self, n: usize) {
        if self.trigger(n) {
            let text = self.render();
            self.write_at(text);
        }
    }

    /// Set position of the progress bar.
    /// Alternative way to update bar.
    pub fn set_position(&mut self, position: usize) {
        self.n = position;
        self.update(0);
    }

    /// Force refresh the display of this bar.
    pub fn refresh(&mut self) {
        if !self.max_fps {
            self.force_refresh = true;
            self.update(0);
            self.force_refresh = false;
        } else {
            self.update(0);
        }
    }

    /// Resets to intial iterations for repeated use.
    /// Consider combining with `leave=true`.
    pub fn reset(&mut self, total: Option<usize>) {
        self.started = false;

        if total.is_some() {
            self.total = total.unwrap();
        }
    }

    /// Clear current bar display.
    pub fn clear(&mut self) {
        if self.file.is_none() {
            let mut columns = term::get_columns() as usize;

            if columns == 0 {
                columns = self.bar_length as usize;
            }

            self.writer
                .print(format_args!("\r{}\r", " ".repeat(columns)));
        }
    }

    /// Print a message via bar (without overlap with bars).
    pub fn write(&mut self, text: String) {
        self.clear();
        self.writer.print(format_args!("{}\n", text));

        if self.leave {
            self.refresh();
        }
    }

    /// Take input via bar (without overlap with bars).
    pub fn input(&mut self, text: &str) -> Result<String, std::io::Error> {
        self.clear();
        self.writer.print_str(text);

        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string)?;

        if self.leave {
            self.refresh();
        }

        Ok(input_string)
    }

    /// Set/Modify description of the progress bar.
    pub fn set_description(&mut self, desc: String) {
        self.desc = desc;
    }

    /// Set/Modify postfix (additional stats) with automatic formatting based on datatype.
    pub fn set_postfix(&mut self, postfix: String) {
        self.postfix = format!(", {}", postfix);
    }

    /// Set/Modify colour of the progress bar.
    pub fn set_colour(&mut self, colour: &str) {
        if colour != "default" {
            self.colour = term::colour(colour);
        } else {
            self.colour = "default".to_string();
        }
    }

    /// Set/Modify charset of the progress bar.
    pub fn set_charset(&mut self, charset: &'static [&'static str]) {
        self.animation = Animation::Custom(charset);
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
    pub fn write_at(&self, mut text: String) {
        if self.wrap {
            let columns = crate::term::get_columns() as usize;

            if self.bar_length as usize > columns {
                text = text[..columns].to_string();
            }
        }

        if self.file.is_none() {
            crate::lock::acquire();

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

            crate::lock::release();
        } else {
            let mut file = self.file.as_ref().unwrap();
            file.write_fmt(format_args!("{}\n", text.as_str())).unwrap();
            file.flush().unwrap();
        }
    }
}
