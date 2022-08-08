use crate::format;
use crate::progress::BarMethods;
use crate::styles::{Animation, Spinner};
use crate::term::{Colorizer, Writer};
use crate::thread::lock;
use formatx::Template;
use std::io::Write;

/// Core implemention of console progress bar.
///
/// # Example
///
/// A clean nice progress bar with a total value.
///
/// ```rust
/// use kdam::prelude::*;
/// use kdam::Bar;
///
/// fn main() {
///     let mut pb = Bar::new(100);
///     // let mut pb = tqdm!(total = 100);
///     // let mut pb = Bar::builder().total(100).build();
///
///     for _ in 0..100 {
///         pb.update(1);
///     }
/// }
/// ```
#[derive(Debug)]
pub struct Bar {
    desc: String,
    pub(crate) total: usize,
    pub(crate) leave: bool,
    file: Option<std::fs::File>,
    pub(crate) ncols: i16,
    mininterval: f32,
    miniters: usize,
    dynamic_miniters: bool,
    pub(crate) disable: bool,
    unit: String,
    unit_scale: bool,
    dynamic_ncols: bool,
    initial: usize,
    bar_format: Option<Template>,
    pub(crate) position: u16,
    postfix: String,
    unit_divisor: usize,
    colour: String,
    delay: f32,
    animation: Animation,
    spinner: Option<Spinner>,
    pub(crate) writer: Writer,
    pub(crate) force_refresh: bool,
    wrap: bool,
    // Internal
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
            desc: "".to_owned(),
            total: 0,
            leave: true,
            file: None,
            ncols: 10,
            mininterval: 0.1,
            miniters: 1,
            dynamic_miniters: false,
            disable: false,
            unit: "it".to_owned(),
            unit_scale: false,
            dynamic_ncols: false,
            initial: 0,
            bar_format: None,
            position: 0,
            postfix: "".to_string(),
            unit_divisor: 1000,
            colour: "default".to_owned(),
            delay: 0.0,
            animation: Animation::Tqdm,
            spinner: None,
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
    /// Create a new instance of [Bar](crate::Bar) with a total value.
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

    /// Create a instance of [BarBuilder](crate::BarBuilder).
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut pb = kdam::Bar::builder().total(100).build();
    /// ```
    pub fn builder() -> BarBuilder {
        BarBuilder::default()
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
        if self.total == 0 {
            f32::INFINITY
        } else {
            (self.total - self.n) as f32 / self.bar_rate()
        }
    }

    /// Format self.n
    pub fn bar_fmt_count(&self) -> String {
        let count = if self.unit_scale {
            format::format_sizeof(self.n as f64, self.unit_divisor as f64)
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
            "{:1$.2$}%",
            self.bar_percentage() * 100.0,
            if precision == 0 { 3 } else { precision + 4 },
            precision
        )
    }

    /// Format rate / iterations of update method calls.
    pub fn bar_fmt_rate(&self) -> String {
        let rate = if self.unit_scale {
            format::format_sizeof(self.bar_rate() as f64, self.unit_divisor as f64)
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
        if self.total == 0 {
            "inf".to_owned()
        } else {
            format::format_interval(self.bar_remaining_time() as usize)
        }
    }

    /// Format self.total
    pub fn bar_fmt_total(&self) -> String {
        let total = if self.unit_scale {
            format::format_sizeof(self.total as f64, self.unit_divisor as f64)
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

    /// Set/Modify bar_format of the progress bar.
    pub fn set_bar_format(&mut self, bar_format: Template) {
        self.bar_format = Some(bar_format);
    }

    /// Set/Modify colour of the progress bar.
    pub fn set_colour<T: Into<String>>(&mut self, colour: T) {
        self.colour = colour.into();
    }

    /// Set/Modify description of the progress bar.
    pub fn set_description<T: Into<String>>(&mut self, desc: T) {
        self.desc = desc.into();
    }

    /// Set/Modify visibility of the progress bar.
    pub fn set_disable(&mut self, disable: bool) {
        self.disable = disable;
    }

    /// Set/Modify leave property of the progress bar.
    pub fn set_leave(&mut self, leave: bool) {
        self.leave = leave;
    }

    /// Set/Modify position of the progress bar.
    pub fn set_position(&mut self, position: u16) {
        self.position = position;
    }

    /// Set/Modify postfix (additional stats) with automatic formatting based on datatype.
    pub fn set_postfix<T: Into<String>>(&mut self, postfix: T) {
        self.postfix = ", ".to_owned() + &postfix.into();
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
            if self.user_ncols.is_none() {
                if let Ok(kdam_ncols) = std::env::var("KDAM_NCOLS") {
                    self.ncols = kdam_ncols
                        .parse::<i16>()
                        .expect("KDAM_NCOLS is not an valid integer (i16).");
                    self.user_ncols = Some(self.ncols);
                }
            }

            self.n = self.initial;
            self.timer = std::time::Instant::now();
            self.started = true;
        }
    }

    /// Adjust number of columns for bar animation using length of remanining bar.
    pub(crate) fn set_ncols(&mut self, lbar_rbar_len: i16) {
        if self.dynamic_ncols || (lbar_rbar_len + self.ncols != self.bar_length) {
            if let Some(ncols) = self.user_ncols {
                self.ncols = ncols;
            } else {
                let columns = crate::term::get_columns_or(0);

                if columns != 0 {
                    let new_ncols = columns as i16 - lbar_rbar_len;
                    self.ncols = if new_ncols > 0 { new_ncols } else { 0 };
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
            let columns = crate::term::get_columns_or(0) as usize;

            if self.bar_length as usize > columns {
                text = text[..columns].to_owned();
            }
        }

        if self.file.is_some() {
            lock::acquire();
            let mut file = self.file.as_ref().unwrap();
            file.write_fmt(format_args!("{}\n", text.as_str())).unwrap();
            file.flush().unwrap();
            lock::release();
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
    }
}

impl BarMethods for Bar {
    fn clear(&mut self) {
        if self.file.is_none() {
            self.write_at(format!(
                "\r{}",
                " ".repeat(crate::term::get_columns_or(self.bar_length as u16) as usize)
            ));
        }
    }

    fn input<T: Into<String>>(&mut self, text: T) -> Result<String, std::io::Error> {
        self.clear();
        self.writer.print_str(&text.into());

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

        if self.bar_format.is_none() {
            let desc = if self.desc == "" {
                "".to_owned()
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

                if !self.leave && self.position != 0 {
                    return format!(
                        "{}\r",
                        " ".repeat(crate::term::get_columns_or(self.bar_length as u16) as usize)
                    );
                }

                return bar;
            }

            let progress = self.bar_percentage() as f32;

            if progress >= 1.0 {
                self.total = self.n;

                if !self.leave && self.position != 0 {
                    return format!(
                        "{}\r",
                        " ".repeat(crate::term::get_columns_or(self.bar_length as u16) as usize)
                    );
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

            self.set_ncols(
                (format!("{}{}", lbar, rbar).len_ansi() + self.animation.spaces() as usize) as i16,
            );

            if self.ncols <= 0 {
                return lbar + &rbar;
            }

            lbar + &self
                .animation
                .fmt_progress(progress, self.ncols.clone(), &self.colour)
                + &rbar
        } else {
            let mut bar_format = self.bar_format.as_ref().unwrap().clone();

            bar_format.replace_from_callback("desc", |placeholder| {
                if self.desc != "" {
                    self.desc.clone() + &placeholder.attr("suffix").unwrap_or(": ".to_owned())
                } else {
                    self.desc.clone()
                }
            });

            bar_format.replace_with_callback("@desc", &self.desc, |fmtval, placeholder| {
                if self.desc != "" {
                    fmtval + &placeholder.attr("suffix").unwrap_or(": ".to_owned())
                } else {
                    fmtval
                }
            });

            bar_format.replace_from_callback("percentage", |placeholder| {
                self.bar_fmt_percentage(
                    placeholder
                        .attr("precision")
                        .unwrap_or("0".to_owned())
                        .parse::<usize>()
                        .unwrap(),
                )
            });

            if bar_format.contains("@percentage") {
                bar_format.replace("@percentage", self.bar_percentage() * 100.0);
            }

            bar_format.replace_from_callback("count", |_| self.bar_fmt_count());
            bar_format.replace("@count", &self.n);

            bar_format.replace_from_callback("total", |_| self.bar_fmt_total());
            bar_format.replace("@total", &self.total);

            bar_format.replace_from_callback("elapsed", |placeholder| {
                if placeholder.attr("human").unwrap_or("false".to_owned()) == "false" {
                    self.bar_fmt_elapsed_time()
                } else {
                    crate::format::format_interval_human(self.elapsed_time as usize)
                }
            });

            bar_format.replace_from_callback("remaining", |placeholder| {
                if placeholder.attr("human").unwrap_or("false".to_owned()) == "false" {
                    self.bar_fmt_remaining_time()
                } else {
                    if self.total == 0 {
                        "0s".to_owned()
                    } else {
                        crate::format::format_interval_human(self.bar_remaining_time() as usize)
                    }
                }
            });

            bar_format.replace_from_callback("rate", |_| self.bar_fmt_rate());

            if bar_format.contains("@rate") {
                bar_format.replace("@rate", self.bar_rate());
            }

            bar_format.replace("postfix", &self.postfix);
            bar_format.replace("unit", &self.unit);

            bar_format.replace_from_callback("spinner", |_| {
                if let Some(spinner) = &self.spinner {
                    spinner.render_frame(self.elapsed_time)
                } else {
                    "".to_owned()
                }
            });

            let length = bar_format.unchecked_text().len_ansi() as i16;
            self.set_ncols(length - 11);

            bar_format.replace_from_callback("animation", |_| {
                let fmtval = self
                    .animation
                    .progress(self.bar_percentage() as f32, self.ncols.clone());

                if self.colour.to_uppercase().starts_with("GRADIENT(") {
                    if !cfg!(feature = "gradient") {
                        panic!("Enable cargo feature `gradient` to use gradient colours.");
                    }

                    #[cfg(feature = "gradient")]
                    return fmtval.gradient(
                        &self
                            .colour
                            .to_uppercase()
                            .trim_start_matches("GRADIENT(")
                            .trim_end_matches(')')
                            .split(",")
                            .collect::<Vec<&str>>(),
                        self.ncols as usize,
                    );
                }

                if self.colour != "default" {
                    return fmtval.colorize(&self.colour);
                }

                fmtval
            });

            bar_format.text().unwrap()
        }
    }

    fn reset(&mut self, total: Option<usize>) {
        if let Some(x) = total {
            self.total = x;
        }

        self.n = self.initial;
        self.timer = std::time::Instant::now();
    }

    fn update(&mut self, n: usize) {
        self.init();

        if self.trigger(n) {
            let text = self.render();
            let length = text.len_ansi() as i16;

            if length != self.bar_length {
                self.clear();
            }

            self.bar_length = length;
            self.write_at(text);
        }
    }

    fn update_to(&mut self, update_to_n: usize) {
        self.n = update_to_n;
        self.update(0);
    }

    fn write<T: Into<String>>(&mut self, text: T) {
        self.clear();
        self.writer.print(format_args!("\r{}\n", text.into()));

        if self.leave {
            self.refresh();
        }
    }
}

/// Create `kdam::Bar` with custom configurations.
///
/// # Example
///
/// ```rust
/// use kdam::BarBuilder;
///
/// let mut pb = BarBuilder::default().total(100).build();
/// ```
#[derive(Default)]
pub struct BarBuilder {
    pb: Bar,
}

impl BarBuilder {
    /// Prefix for the progress bar.
    /// (default: `""`)
    pub fn desc<T: Into<String>>(mut self, desc: T) -> Self {
        self.pb.desc = desc.into();
        self
    }

    /// The number of expected iterations.
    /// If unspecified, iterable.size_hint().0 is used if possible.
    /// If 0, only basic progress statistics are displayed (no ETA, no progressbar).
    /// (default: `0`)
    pub fn total(mut self, total: usize) -> Self {
        self.pb.total = total;
        self
    }

    /// If true, keeps all traces of the progressbar upon termination of iteration.
    /// If false, will leave only if position is 0.
    /// (default: `true`)
    pub fn leave(mut self, leave: bool) -> Self {
        self.pb.leave = leave;
        self
    }

    /// Specifies where to output the progress messages (default: stderr).
    /// Uses file.write_fmt and file.flush methods.
    /// (default: `None`)
    pub fn file(mut self, file: Option<std::fs::File>) -> Self {
        self.pb.file = file.into();
        self
    }

    /// The width of the entire output message.
    /// If specified, dynamically resizes the progressbar to stay within this bound.
    /// If unspecified, attempts to use KDAM_NCOLS environment variable or adjust width automatically.
    /// The fallback is a meter width of 10 and no limit for the counter and statistics.
    /// If 0, will not print any meter (only stats).
    /// (default: `10`)
    pub fn ncols<T: Into<i16>>(mut self, ncols: T) -> Self {
        self.pb.ncols = ncols.into();
        self.pb.user_ncols = Some(self.pb.ncols);
        self
    }

    /// Minimum progress display update interval (in seconds).
    /// (default: `0.1`)
    pub fn mininterval<T: Into<f32>>(mut self, mininterval: T) -> Self {
        self.pb.mininterval = mininterval.into();
        self
    }

    /// Minimum progress display update interval, in iterations.
    /// If > 0, will skip display of specified number of iterations. Tweak this and mininterval to get very efficient loops.
    /// If your progress is erratic with both fast and slow iterations (network, skipping items, etc) you should set miniters=1.
    /// (default: `1`)
    pub fn miniters(mut self, miniters: usize) -> Self {
        self.pb.miniters = miniters;
        self
    }

    /// Automatically adjusts miniters to correspond to mininterval after long display update lag.
    /// (default: `false`)
    pub fn dynamic_miniters(mut self, dynamic_miniters: bool) -> Self {
        self.pb.dynamic_miniters = dynamic_miniters;
        self
    }

    /// Whether to disable the entire progress bar wrapper.
    /// (default: `false`)
    pub fn disable(mut self, disable: bool) -> Self {
        self.pb.disable = disable.into();
        self
    }

    /// String that will be used to define the unit of each iteration.
    /// (default: `"it"`)
    pub fn unit<T: Into<String>>(mut self, unit: T) -> Self {
        self.pb.unit = unit.into();
        self
    }

    /// If true, the number of iterations will be reduced/scaled automatically
    /// and a metric prefix following the International System of Units standard will be added (kilo, mega, etc.).
    /// (default: `false`)
    pub fn unit_scale(mut self, unit_scale: bool) -> Self {
        self.pb.unit_scale = unit_scale;
        self
    }

    /// If true, constantly alters ncols to the environment (allowing for window resizes).
    /// (default: `false`)
    pub fn dynamic_ncols(mut self, dynamic_ncols: bool) -> Self {
        self.pb.dynamic_ncols = dynamic_ncols;
        self
    }

    /// The initial counter value. Useful when restarting a progress bar.
    /// (default: 0)
    pub fn initial(mut self, initial: usize) -> Self {
        self.pb.initial = initial;
        self
    }

    /// Specify a custom bar string formatting. May impact performance.
    /// [default: '{l_bar}{bar}{r_bar}'], where
    /// l_bar='{desc}: {percentage:3.0f}%|' and
    /// r_bar='| {n_fmt}/{total_fmt} [{elapsed}<{remaining}, '
    ///   '{rate_fmt}{postfix}]'
    /// Possible vars: l_bar, bar, r_bar, n, n_fmt, total, total_fmt,
    ///   percentage, elapsed, elapsed_s, ncols, nrows, desc, unit,
    ///   rate, rate_fmt, rate_noinv, rate_noinv_fmt,
    ///   rate_inv, rate_inv_fmt, postfix, unit_divisor,
    ///   remaining, remaining_s, eta.
    /// Note that a trailing ": " is automatically removed after {desc}
    /// if the latter is empty.
    ///
    /// ```text
    /// {desc} {percentage} {animation} {count}/{total} [{elapsed}<{remaining}, {rate}{postfix}]
    /// ```
    /// (default: `None`)
    pub fn bar_format(mut self, bar_format: Template) -> Self {
        self.pb.bar_format = Some(bar_format);
        self
    }

    /// Specify the line offset to print this bar (starting from 0).
    /// Useful to manage multiple bars at once (eg, from threads).
    /// (default: `0`)
    pub fn position(mut self, position: u16) -> Self {
        self.pb.position = position;
        self
    }

    /// Specify additional stats to display at the end of the bar.
    /// (default: `""`)
    pub fn postfix<T: Into<String>>(mut self, postfix: T) -> Self {
        self.pb.set_postfix(postfix.into());
        self
    }

    /// Divide values by this unit_divisor.
    /// Ignored unless `unit_scale` is true.
    /// (default: `1024`)
    pub fn unit_divisor(mut self, unit_divisor: usize) -> Self {
        self.pb.unit_divisor = unit_divisor;
        self
    }

    /// Bar colour (e.g. "green", "#00ff00").
    pub fn colour<T: Into<String>>(mut self, colour: T) -> Self {
        self.pb.colour = colour.into();
        self
    }

    /// Don't display until few seconds have elapsed.
    /// (default: `0`)
    pub fn delay<T: Into<f32>>(mut self, delay: T) -> Self {
        self.pb.delay = delay.into();
        self
    }

    /// Defines the animation style to use with progress bar.
    /// (default: `kdam::Animation::Tqdm`)
    pub fn animation<T: Into<Animation>>(mut self, animation: T) -> Self {
        self.pb.animation = animation.into();
        self
    }

    /// Defines the spinner to use with progress bar.
    /// (default: `None`)
    pub fn spinner(mut self, spinner: Spinner) -> Self {
        self.pb.spinner = Some(spinner);
        self
    }

    /// Select writer type to display progress bar output between stdout and stderr.
    /// (default: `kdam::Output::Stderr`)
    pub fn writer<T: Into<Writer>>(mut self, writer: T) -> Self {
        self.pb.writer = writer.into();
        self
    }
    /// If true, each update method call will be rendered.
    /// (default: `false`)
    pub fn force_refresh(mut self, force_refresh: bool) -> Self {
        self.pb.force_refresh = force_refresh;
        self
    }

    /// If true, progress bar of more length than terminal will be trimmed at end.
    /// (default: `false`)
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.pb.wrap = wrap;
        self
    }

    /// Build `kdam::Bar`
    pub fn build(self) -> Bar {
        self.pb
    }
}

/// [tqdm](https://github.com/tqdm/tqdm) like macro for constructing [BarIterator](crate::BarIterator) if iterable is given else [Bar](crate::Bar).
///
/// This macro use [BarBuilder](crate::BarBuilder) for creating [Bar](crate::Bar).
/// See all available [methods](https://docs.rs/kdam/latest/kdam/struct.BarBuilder.html).
///
/// # Examples
///
/// ```rust
/// use kdam::prelude::*;
///
/// tqdm!();
/// tqdm!(total = 100);
/// tqdm!(total = 100, mininterval = 0.0, colour = "green");
/// tqdm!(0..100);
/// tqdm!(0..100, desc = "0 to 99");
/// tqdm!(["a", "b", "c", "d"].iter());
/// ```
#[macro_export]
macro_rules! tqdm {
    ($($setter_method: ident = $value: expr),*) => {
        $crate::BarBuilder::default()$(.$setter_method($value))*.build()
    };

    ($iterable: expr) => {
        $crate::BarIterator::new_with_bar($iterable, kdam::Bar::default())
    };

    ($iterable: expr, $($setter_method: ident = $value: expr),*) => {
        $crate::BarIterator::new_with_bar($iterable, kdam::BarBuilder::default()$(.$setter_method($value))*.build())
    };
}
