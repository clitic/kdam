use std::io::Write;

use crate::format;
use crate::styles::{Animation, Output};
use crate::term;

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
///     let mut pb = Bar {
///         total: 100,
///         ..Default::default()
///     };
///
///     for _ in 0..100 {
///         pb.update(1);
///     }
/// }
/// ```
///
/// Another example without a total value. This only shows basic stats.
///
/// ```rust
/// use kdam::Bar;
///
/// fn main() {
///     let mut pb = Bar::default();
///
///     for _ in 0..100 {
///         pb.update(1);
///     }
/// }
/// ```
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
    /// Select where to display progress bar output between stdout and stderr.
    /// (default: `kdam::Output::Stderr`)
    pub output: Output,
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
    timer: std::time::Instant,
    elapsed_time: f32,
    user_ncols: Option<i16>,
    bar_length: i16,
    force_refresh: bool,
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
            output: Output::Stderr,
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
    pub fn new(total: usize) -> Bar {
        Bar {
            total: total,
            ..Default::default()
        }
    }

    /// Initialize struct values.
    fn init(&mut self) {
        self.set_colour(&self.colour.clone());

        if self.ncols != 10 {
            self.user_ncols = Some(self.ncols);
        }

        if self.max_fps {
            self.force_refresh = true;
        }
    }

    fn render_unknown(&mut self, i: usize) -> String {
        let desc_spacing = if self.desc == "" { "" } else { ": " };
        self.elapsed_time = self.timer.elapsed().as_secs_f32();
        let its_per = i as f64 / self.elapsed_time as f64;
        let elapsed_time_fmt = format::format_interval(self.elapsed_time as usize);

        let count = if self.unit_scale {
            format::format_sizeof(i, self.unit_divisor)
        } else {
            format!("{}", i)
        };

        let rate_fmt = if self.unit_scale {
            format::format_sizeof(its_per as usize, self.unit_divisor)
        } else {
            format!("{:.2}", its_per)
        };

        return format!(
            "{}{}{} [{}, {}{}/s{}]",
            self.desc, desc_spacing, count, elapsed_time_fmt, rate_fmt, self.unit, self.postfix
        );
    }

    fn render_lbar(&mut self, n: usize) -> (f32, String) {
        let mut progress = (n as f64) / (self.total as f64);

        if progress >= 1.0 {
            progress = 1.0;
        }

        let desc_spacing = if self.desc == "" { "" } else { ": " };
        let percentage = (progress * 100.0) as u8;
        let mut spacing = if percentage >= 10 { " " } else { "  " };

        if progress >= 1.0 {
            spacing = "";
        }

        return (
            progress as f32,
            format!("{}{}{}{}%", self.desc, desc_spacing, spacing, percentage),
        );
    }

    fn render_rbar(&mut self, n: usize) -> String {
        let count = if self.unit_scale {
            format::format_sizeof(n, self.unit_divisor)
        } else {
            format!("{}", n)
        };

        let total = if self.unit_scale {
            format::format_sizeof(self.total, self.unit_divisor)
        } else {
            format!("{}", self.total)
        };

        self.elapsed_time = self.timer.elapsed().as_secs_f32();
        let its_per = n as f64 / self.elapsed_time as f64;

        let remaning_time = (self.total - n) as f64 / its_per;

        let elapsed_time_fmt = format::format_interval(self.elapsed_time as usize);
        let mut remaning_time_fmt = format::format_interval(remaning_time as usize);
        let mut rate_fmt = if self.unit_scale {
            format::format_sizeof(its_per as usize, self.unit_divisor)
        } else {
            format!("{:.2}", its_per).to_string()
        };

        if n == 0 {
            remaning_time_fmt = "00:00".to_string();
            rate_fmt = "?".to_string();
        }

        return format!(
            " {}/{} [{}<{}, {}{}/s{}]",
            count, total, elapsed_time_fmt, remaning_time_fmt, rate_fmt, self.unit, self.postfix,
        );
    }

    fn render_mbar(&mut self, progress: f32) -> String {
        let (bar_open, bar_close, bar_animation);

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
                (bar_open, bar_close) = ("", "");
                bar_animation =
                    crate::styles::simple(progress, self.ncols.clone(), self.animation.clone());
            }
        }

        if self.colour == "default" {
            return format!("{}{}{}", bar_open, bar_animation, bar_close);
        } else {
            return format!(
                "{}{}{}{}{}",
                bar_open,
                self.colour,
                bar_animation,
                term::COLOUR_RESET,
                bar_close
            );
        }
    }

    fn set_ncols(&mut self, lbar_rbar_len: i16) {
        if self.dynamic_ncols || (lbar_rbar_len + self.ncols + 2 - self.bar_length) > 0 {
            if self.user_ncols.is_some() {
                self.ncols = self.user_ncols.unwrap();
            } else {
                let columns = term::get_columns();

                if columns != 0 {
                    let new_ncols = columns as i16 - lbar_rbar_len - 3;
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

    /// Render progress bar text using given value.
    fn render(&mut self, mut n: usize) -> (String, String, String) {
        let (progress, lbar) = self.render_lbar(n);

        if progress >= 1.0 {
            n = self.total;

            if !self.leave {
                return (
                    " ".repeat(self.bar_length as usize).to_string(),
                    "".to_string(),
                    "\r".to_string(),
                );
            }
        }

        let rbar = self.render_rbar(n);

        self.set_ncols(format!("\r{}{}", lbar, rbar).chars().count() as i16);

        if self.ncols <= 0 {
            return (lbar, "".to_string(), rbar);
        }

        let mbar = self.render_mbar(progress);

        return (lbar, mbar, rbar);
    }

    /// Manually update the progress bar, useful for streams such as reading files.
    pub fn update(&mut self, n: usize) {
        if !self.started {
            term::init();
            self.init();
            self.timer = std::time::Instant::now();
            self.started = true;
        }

        self.n += n;

        if !self.disable {
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
                || self.force_refresh
            {
                if self.dynamic_miniters {
                    self.miniters = 0;
                }

                if self.total != 0 {
                    let (lbar, mbar, rbar) = self.render(self.n);
                    self.bar_length =
                        ((lbar.chars().count() + rbar.chars().count()) as i16) + self.ncols + 2;

                    if !self.wrap {
                        self.write_at(format!("{}{}{}", lbar, mbar, rbar));
                    } else {
                        let columns = term::get_columns() as usize;
                        if self.bar_length as usize > columns {
                            self.write_at(
                                format!("{}{}{}", lbar, mbar, rbar)[..columns].to_string(),
                            );
                        } else {
                            self.write_at(format!("{}{}{}", lbar, mbar, rbar));
                        }
                    }
                } else {
                    let text = self.render_unknown(self.n);
                    self.bar_length = text.chars().count() as i16;

                    if !self.wrap {
                        self.write_at(text);
                    } else {
                        let columns = term::get_columns() as usize;
                        if self.bar_length as usize > columns {
                            self.write_at(text[..columns].to_string());
                        } else {
                            self.write_at(text);
                        }
                    }
                }
            }
        }
    }

    fn write_at(&self, text: String) {
        if self.file.is_none() {
            crate::lock::acquire();

            if self.position == 0 {
                match self.output {
                    Output::Stderr => term::write_to_stderr(format_args!("\r{}", text)),
                    Output::Stdout => term::write_to_stdout(format_args!("\r{}", text)),
                }
            } else {
                match self.output {
                    Output::Stderr => term::write_to_stderr(format_args!(
                        "{}{}{}",
                        "\n".repeat(self.position as usize),
                        text,
                        format!("\x1b[{}A", self.position)
                    )),
                    Output::Stdout => term::write_to_stdout(format_args!(
                        "{}{}{}",
                        "\n".repeat(self.position as usize),
                        text,
                        format!("\x1b[{}A", self.position)
                    )),
                }
            }

            crate::lock::release();
        } else {
            let mut file = self.file.as_ref().unwrap();
            file.write_fmt(format_args!("{}\n", text.as_str())).unwrap();
            file.flush().unwrap();
        }
    }

    /// Clear current bar display.
    pub fn clear(&mut self) {
        if self.file.is_none() {
            let mut columns = term::get_columns() as usize;

            if columns == 0 {
                columns = self.bar_length as usize;
            }

            match self.output {
                Output::Stderr => {
                    term::write_to_stderr(format_args!("\r{}\r", " ".repeat(columns)))
                }
                Output::Stdout => {
                    term::write_to_stdout(format_args!("\r{}\r", " ".repeat(columns)))
                }
            }
        }
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

    /// Print a message via bar (without overlap with bars).
    /// This message is printed to stdout.
    pub fn write(&mut self, text: String) {
        self.clear();

        term::write_to_stdout(format_args!("{}\n", text));

        if self.leave {
            self.refresh();
        }
    }

    /// Take input via bar (without overlap with bars).
    /// The input message is printed to stdout.
    pub fn input(&mut self, text: &str) -> Result<String, std::io::Error> {
        self.clear();

        term::write_to_stdout(format_args!("{}", text));

        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string)?;

        if self.leave {
            self.refresh();
        }

        Ok(input_string)
    }

    /// Set/Modify position of the progress bar.
    pub fn set_position(&mut self, position: usize) {
        self.n = position;
        self.update(0);
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
        self.animation = Animation::Custom(Some(charset));
    }
}
