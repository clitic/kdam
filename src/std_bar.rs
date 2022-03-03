use std::io::Write;

use crate::format;
use crate::internal::BarInternal;
use crate::term;
use crate::styles::Animation;

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
///     // In some cases creating struct doesn't set corresponding values for other variables.
///     // To solve this error use `set_defaults` method.
///     // pb.set_defaults();
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
    pub total: u64,
    /// If true, keeps all traces of the progressbar upon termination of iteration.
    /// If false, will leave only if position is 0.
    /// (default: `true`)
    pub leave: bool,
    /// Specifies where to output the progress messages (default: stdout).
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
    pub mininterval: f64,
    /// Minimum progress display update interval, in iterations.
    /// If > 0, will skip display of specified number of iterations. Tweak this and mininterval to get very efficient loops.
    /// If your progress is erratic with both fast and slow iterations (network, skipping items, etc) you should set miniters=1.
    /// (default: `1`)
    pub miniters: u64,
    /// If false, use unicode (smooth blocks) to fill the meter.
    /// If true, use ASCII characters "123456789#" to fill the meter.
    /// You can change ASCII charset using set_charset method.
    /// (default: `false`)
    pub ascii: bool,
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
    /// The initial counter value. Useful when restarting a progress bar.
    /// (default: `0`)
    pub initial: u64,
    /// Specify additional stats to display at the end of the bar.
    /// (default: `""`)
    pub postfix: String,
    /// ignored unless unit_scale is true.
    /// (default: `1024`)
    pub unit_divisor: u64,
    /// Bar colour (e.g. "green", "#00ff00").
    pub colour: String,
    /// Don't display until few seconds have elapsed.
    /// (default: `0`)
    pub delay: f64,
    /// Fill incompleted progress bar with a character.
    /// (default: `" "`)
    pub fill: String,
    /// Defines the animation style to use with progress bar.
    /// For custom type use set_charset method.
    /// (default: `Animation::TqdmAscii`)
    pub animation: Animation,
    /// Counter of progress bar.
    /// (default: `0`)
    pub i: u64,
    /// Variables for internal use.
    pub internal: BarInternal,
}

impl Default for Bar {
    fn default() -> Bar {
        Bar {
            desc: "".to_string(),
            total: 0,
            leave: true,
            file: None,
            ncols: 10,
            mininterval: 0.1,
            miniters: 1,
            ascii: false,
            disable: false,
            unit: "it".to_string(),
            unit_scale: false,
            dynamic_ncols: false,
            initial: 0,
            postfix: "".to_string(),
            unit_divisor: 1000,
            colour: "default".to_string(),
            delay: 0.0,
            fill: " ".to_string(),
            animation: Animation::TqdmAscii,
            i: 0,
            internal: BarInternal::default(),
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
    pub fn new(total: u64) -> Bar {
        Bar {
            total: total,
            ..Default::default()
        }
    }

    /// Set default values to some internal values.
    pub fn set_defaults(&mut self) {
        self.i = self.initial;

        if self.ncols != 10 {
            self.internal.user_ncols = self.ncols;
        }
        
        self.set_colour(&self.colour.clone());

        if self.ascii {
            self.set_charset(&crate::styles::TQDMASCIICHARSET);
        } else if matches!(self.animation, Animation::Tqdm) {
            self.set_charset(&crate::styles::TQDMCHARSET);
        } else if matches!(self.animation, Animation::FillUp) {
            self.set_charset(&crate::styles::FILLUPCHARSET);
        } else if matches!(self.animation, Animation::Classic) {
            self.internal.charset = "#".to_string();
            self.fill = ".".to_string();
        } else if matches!(self.animation, Animation::Arrow) {
            self.internal.charset = "=".to_string();
        }
    }

    fn render_unknown(&mut self, i: u64) -> String {
        let desc_spacing = if self.desc == "" { "" } else { ": " };
        self.internal.elapsed_time = self.internal.timer.elapsed().as_secs_f64();
        self.internal.its_per = i as f64 / self.internal.elapsed_time;
        let elapsed_time_fmt = format::format_interval(self.internal.elapsed_time as u64);

        let count = if self.unit_scale {
            format::format_sizeof(i, self.unit_divisor)
        } else {
            format!("{}", i)
        };

        let rate_fmt = if self.unit_scale {
            format::format_sizeof(self.internal.its_per as u64, self.unit_divisor)
        } else {
            format!("{:.2}", self.internal.its_per).to_string()
        };

        return format!(
            "{}{}{} [{}, {}{}/s{}]",
            self.desc, desc_spacing, count, elapsed_time_fmt, rate_fmt, self.unit, self.postfix
        );
    }

    fn render_lbar(&mut self, i: u64) -> (f64, String) {
        let mut progress = (i as f64) / (self.total as f64);

        if progress >= 1.0 {
            progress = 1.0;
        }

        let desc_spacing = if self.desc == "" { "" } else { ": " };
        let percentage = (progress * 100.0) as u64;
        let mut spacing = if percentage >= 10 { " " } else { "  " };

        if progress >= 1.0 {
            spacing = "";
        }

        return (
            progress,
            format!("{}{}{}{}%", self.desc, desc_spacing, spacing, percentage),
        );
    }

    fn render_rbar(&mut self, i: u64) -> String {
        let count = if self.unit_scale {
            format::format_sizeof(i, self.unit_divisor)
        } else {
            format!("{}", i)
        };

        let total = if self.unit_scale {
            format::format_sizeof(self.total, self.unit_divisor)
        } else {
            format!("{}", self.total)
        };

        self.internal.elapsed_time = self.internal.timer.elapsed().as_secs_f64();
        self.internal.its_per = i as f64 / self.internal.elapsed_time;

        let remaning_time = (self.total - i) as f64 / self.internal.its_per;

        let elapsed_time_fmt = format::format_interval(self.internal.elapsed_time as u64);
        let mut remaning_time_fmt = format::format_interval(remaning_time as u64);
        let mut rate_fmt = if self.unit_scale {
            format::format_sizeof(self.internal.its_per as u64, self.unit_divisor)
        } else {
            format!("{:.2}", self.internal.its_per).to_string()
        };

        if i == 0 {
            remaning_time_fmt = "00:00".to_string();
            rate_fmt = "?".to_string();
        }

        return format!(
            " {}/{} [{}<{}, {}{}/s{}]",
            count, total, elapsed_time_fmt, remaning_time_fmt, rate_fmt, self.unit, self.postfix,
        );
    }

    fn set_ncols(&mut self, lbar_rbar_len: i16) {
        if self.dynamic_ncols || (lbar_rbar_len + self.ncols + 2 - self.internal.bar_length) > 0 {
            if self.internal.user_ncols != -1 {
                self.ncols = self.internal.user_ncols;
            } else {
                let columns = term::get_columns();

                if columns != 0 {
                    let new_ncols = columns as i16 - lbar_rbar_len - 3;
                    if new_ncols >= 0 {
                        self.ncols = new_ncols;
                    }
                } else {
                    self.ncols = 10;
                }
            }
        }
    }

    fn render_mbar(&mut self, progress: f64) -> String {
        let mut bar_animation: String;

        if matches!(self.animation, Animation::TqdmAscii) {
            let nsyms = self.internal.charset_len - 1;
            let (bar_length, frac_bar_length) = format::divmod(
                (progress * self.ncols as f64 * nsyms as f64) as u64,
                nsyms as u64,
            );
            bar_animation = self
                .internal
                .charset
                .chars()
                .nth_back(0)
                .unwrap()
                .to_string()
                .repeat(bar_length as usize);

            if bar_length < self.ncols as u64 {
                bar_animation += &self
                    .internal
                    .charset
                    .chars()
                    .nth((frac_bar_length as usize) + 1)
                    .unwrap()
                    .to_string();
                bar_animation += &self
                    .fill
                    .repeat((self.ncols - (bar_length as i16) - 1) as usize);
            }
        } else {
            let block = (self.ncols as f64 * progress) as i16;
            bar_animation = self.internal.charset.repeat(block as usize);
            if matches!(self.animation, Animation::Classic) {
                bar_animation += &self.fill.repeat((self.ncols - block) as usize);
            } else if matches!(self.animation, Animation::Arrow) {
                let x = self.ncols - block - 1;
                if x > 0 {
                    bar_animation += ">";
                    bar_animation += &self.fill.repeat(x as usize);
                }
            }
        }

        if self.colour != "default" {
            bar_animation = format!("{}{}{}", self.colour, bar_animation, term::COLOUR_RESET);
        }

        if matches!(self.animation, Animation::TqdmAscii) {
            return format!("|{}|", bar_animation);
        } else {
            return format!("[{}]", bar_animation);
        }
    }

    /// Render progress bar text using given value.
    fn render(&mut self, mut i: u64) -> (String, String, String) {
        let (progress, lbar) = self.render_lbar(i);

        if progress == 1.0 {
            i = self.total;

            if !self.leave {
                return (
                    " ".repeat(self.internal.bar_length as usize).to_string(),
                    "".to_string(),
                    "\r".to_string(),
                );
            }
        }

        let rbar = self.render_rbar(i);

        self.set_ncols(format!("\r{}{}", lbar, rbar).len() as i16);

        if self.ncols <= 0 {
            return (lbar, "".to_string(), rbar);
        }

        let mbar = self.render_mbar(progress);

        return (lbar, mbar, rbar);
    }

    /// Manually update the progress bar, useful for streams such as reading files.
    pub fn update(&mut self, i: u64) {
        self.i += i;

        if !self.disable {
            if !self.internal.started {
                term::init();
                self.internal.timer = std::time::Instant::now();
                self.internal.started = true;
            }

            let mut force_refresh = false;

            if self.i == self.total || i == 0 {
                force_refresh = true;
            }

            if ((self.mininterval
                <= (self.internal.timer.elapsed().as_secs_f64() - self.internal.elapsed_time))
                && (self.delay <= self.internal.timer.elapsed().as_secs_f64())
                && (self.i % self.miniters == 0))
                || force_refresh
            {
                let text: String;

                if self.total != 0 {
                    let (lbar, mbar, rbar) = self.render(self.i);
                    text = format!("{}{}{}", lbar, mbar, rbar);
                    self.internal.bar_length =
                        format!("\r{}{}", lbar, rbar).len() as i16 + self.ncols + 2;
                } else {
                    text = self.render_unknown(self.i);
                    self.internal.bar_length = text.len() as i16;
                }

                if self.file.is_none() {
                    if self.internal.nrows == -1 {
                        self.internal
                            .stdout
                            .write_fmt(format_args!("\r{}", text.as_str()))
                            .unwrap();
                        self.internal.stdout.flush().unwrap();
                    } else {
                        if self.internal.tx.is_some() {
                            self.internal
                                .tx
                                .as_ref()
                                .unwrap()
                                .send((self.internal.nrows, text, self.i == self.total))
                                .unwrap();
                        }
                    }
                } else {
                    self.file
                        .as_ref()
                        .unwrap()
                        .write_fmt(format_args!("\r{}", text.as_str()))
                        .unwrap();
                    self.file.as_ref().unwrap().flush().unwrap();
                }
            }
        }
    }

    /// Restart bar timer from last print time.
    pub fn unpause(&mut self) {
        self.internal.elapsed_time = self
            .internal
            .timer
            .duration_since(self.internal.timer)
            .as_secs_f64()
            - self.internal.elapsed_time;
    }

    /// Clear current bar display.
    pub fn clear(&mut self) {
        self.internal
            .stdout
            .write_fmt(format_args!(
                "\r{}\r",
                " ".repeat(self.internal.bar_length as usize)
            ))
            .unwrap();
        self.internal.stdout.flush().unwrap();
    }

    /// Force refresh the display of this bar.
    pub fn refresh(&mut self) {
        self.update(0);
    }

    /// Resets to 0 iterations for repeated use.
    /// Consider combining with `leave=true`.
    pub fn reset(&mut self, total: Option<u64>) {
        self.internal.started = false;
        self.i = self.initial;

        if total.is_some() {
            self.total = total.unwrap();
        }
    }

    /// Print a message via bar (without overlap with bars).
    pub fn write(&mut self, text: &str) {
        if self.file.is_none() {
            self.clear();
            println!("{}", text);
            if self.leave {
                self.refresh();
            }
        } else {
            self.file
                .as_ref()
                .unwrap()
                .write_fmt(format_args!("{}\n", text))
                .unwrap();
            self.file.as_ref().unwrap().flush().unwrap();
        }
    }

    /// Set/Modify description of the progress bar.
    pub fn set_description(&mut self, desc: &str, refresh: bool) {
        self.desc = String::from(desc);
        if refresh {
            self.refresh();
        }
    }

    /// Set/Modify postfix (additional stats) with automatic formatting based on datatype.
    pub fn set_postfix(&mut self, postfix: &str, refresh: bool) {
        self.postfix = format!(", {}", postfix);
        if refresh {
            self.refresh();
        }
    }

    /// Set/Modify colour of the progress bar.
    pub fn set_colour(&mut self, colour: &str) {
        if self.colour != "default" {
            self.colour = term::colour(colour);
        }
    }
    
    /// Set/Modify charset of the progress bar.
    pub fn set_charset(&mut self, charset: &[&str]) {
        self.internal.charset = charset.join("");
        self.internal.charset_len = charset.len() as u64;
        self.animation = Animation::TqdmAscii;
    }

    /// EXPERIMENTAL - monitor mode support.
    pub fn monitor(&mut self, maxinterval: f32) {
        let mut n = self.i;

        while self.i != self.total {
            std::thread::sleep(std::time::Duration::from_secs_f32(maxinterval));
            if self.i == n {
                self.refresh();
            } else {
                n = self.i
            }
        }
    }
}

unsafe impl Sync for Bar {}
