use super::{
    styles::{Animation, Colour},
    BarExt,
};
use crate::{
    format, lock,
    term::{Colorizer, Writer},
};

#[cfg(feature = "spinner")]
use crate::spinner::Spinner;

#[cfg(feature = "template")]
use formatx::Template;

/// Core implemention of console progress bar.
///
/// # Example
///
/// A clean nice progress bar with a total value.
///
/// ```
/// use kdam::{tqdm, BarExt};
/// use kdam::Bar;
///
/// let mut pb = Bar::new(100);
/// // let mut pb = tqdm!(total = 100);
/// // let mut pb = Bar::builder().total(100).build();
///
/// for _ in 0..100 {
///     pb.update(1);
/// }
/// ```
#[derive(Debug)]
pub struct Bar {
    // CUSTOMIZABLE FIELDS
    pub animation: Animation,
    #[cfg(feature = "template")]
    pub bar_format: Option<Template>,
    pub colour: Option<Colour>,
    pub delay: f32,
    pub desc: String,
    pub disable: bool,
    pub dynamic_miniters: bool,
    pub dynamic_ncols: bool,
    pub force_refresh: bool,
    pub initial: usize,
    pub inverse_unit: bool,
    pub leave: bool,
    pub mininterval: f32,
    pub miniters: usize,
    pub ncols: i16,
    pub position: u16,
    pub postfix: String,
    pub total: usize,
    #[cfg(feature = "spinner")]
    pub spinner: Option<Spinner>,
    pub unit: String,
    pub unit_divisor: usize,
    pub unit_scale: bool,
    pub writer: Writer,
    // NON CUSTOMIZABLE FIELDS
    bar_length: i16,
    counter: usize,
    timer: std::time::Instant,
    elapsed_time: f32,
    user_ncols: Option<i16>,
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            desc: "".to_owned(),
            total: 0,
            leave: true,
            ncols: 10,
            mininterval: 0.1,
            miniters: 1,
            dynamic_miniters: false,
            disable: false,
            unit: "it".to_owned(),
            unit_scale: false,
            dynamic_ncols: false,
            initial: 0,
            inverse_unit: false,
            #[cfg(feature = "template")]
            bar_format: None,
            position: 0,
            postfix: "".to_string(),
            unit_divisor: 1000,
            colour: None,
            delay: 0.0,
            animation: Animation::Tqdm,
            #[cfg(feature = "spinner")]
            spinner: None,
            writer: Writer::Stderr,
            force_refresh: false,
            counter: 0,
            timer: std::time::Instant::now(),
            elapsed_time: 0.0,
            user_ncols: None,
            bar_length: 0,
        }
        .init()
    }
}

impl Bar {
    // -----------------------------------------------------------------------------------------
    // CONSTRUCTORS
    // -----------------------------------------------------------------------------------------

    /// Create a new instance of [Bar](crate::Bar) with a total value.
    /// You can also set `total=0` if total is unknown.
    ///
    /// # Example
    ///
    /// ```
    /// let mut pb = kdam::Bar::new(100);
    /// ```
    pub fn new(total: usize) -> Self {
        Self {
            total,
            ..Default::default()
        }
        .init()
    }

    /// Create a instance of [BarBuilder](crate::BarBuilder).
    ///
    /// # Example
    ///
    /// ```
    /// let mut pb = kdam::Bar::builder().total(100).build();
    /// ```
    pub fn builder() -> BarBuilder {
        BarBuilder::default()
    }

    fn init(mut self) -> Self {
        if self.user_ncols.is_none() {
            if let Ok(ncols) = std::env::var("KDAM_NCOLS") {
                self.ncols = ncols
                    .parse::<i16>()
                    .expect("KDAM_NCOLS is not an valid integer (i16).");
                self.user_ncols = Some(self.ncols);
            }
        }

        self.counter = self.initial;
        self.timer = std::time::Instant::now();
        self
    }

    // -----------------------------------------------------------------------------------------
    // GETTERS
    // -----------------------------------------------------------------------------------------

    /// Get bar length value.
    pub fn get_bar_length(&self) -> i16 {
        self.bar_length
    }

    /// Get counter value.
    pub fn get_counter(&self) -> usize {
        self.counter
    }

    // -----------------------------------------------------------------------------------------
    // SETTERS
    // -----------------------------------------------------------------------------------------

    /// Set/Modify counter property.
    pub fn set_counter(&mut self, counter: usize) {
        self.counter = counter;
    }

    /// Set/Modify bar length property.
    pub fn set_bar_length(&mut self, bar_length: i16) {
        self.bar_length = bar_length;
    }

    /// Set/Modify bar_format property.
    #[cfg(feature = "template")]
    #[cfg_attr(docsrs, doc(cfg(feature = "template")))]
    pub fn set_bar_format<T: Into<String>>(&mut self, bar_format: T) -> Result<(), formatx::Error> {
        let bar_format = bar_format.into().parse::<Template>()?;
        let mut bar_format_check = bar_format.clone();
        bar_format_check.replace("desc", "");
        bar_format_check.replace("percentage", 0.0);
        bar_format_check.replace("count", 0);
        bar_format_check.replace("total", 0);
        bar_format_check.replace("elapsed", 0);
        bar_format_check.replace("remaining", 0);
        bar_format_check.replace("rate", 0.0);
        bar_format_check.replace("unit", "");
        bar_format_check.replace("postfix", "");
        #[cfg(feature = "spinner")]
        bar_format_check.replace("spinner", "");
        bar_format_check.replace("animation", "");
        bar_format_check.text()?;
        self.bar_format = Some(bar_format);
        Ok(())
    }

    /// Set/Modify description property.
    pub fn set_description<T: Into<String>>(&mut self, description: T) {
        self.desc = description.into();
    }

    /// Set/Modify postfix property.
    pub fn set_postfix<T: Into<String>>(&mut self, postfix: T) {
        self.postfix = ", ".to_owned() + &postfix.into();
    }

    // -----------------------------------------------------------------------------------------
    // BASIC INFORMATION
    // -----------------------------------------------------------------------------------------

    /// Returns progress percentage, like 0.62, 0.262, 1.0.
    /// If total is 0, it returns 1.0.
    pub fn percentage(&self) -> f64 {
        if self.indefinite() {
            1.0
        } else {
            self.counter as f64 / self.total as f64
        }
    }

    /// Set/Returns progress elapsed time.
    pub fn elapsed_time(&mut self) -> f32 {
        self.elapsed_time = self.timer.elapsed().as_secs_f32();
        self.elapsed_time
    }

    /// Returns remaining time (ETA) for progress completion.
    pub fn remaining_time(&self) -> f32 {
        if self.indefinite() {
            f32::INFINITY
        } else {
            (self.total - self.counter) as f32 / self.rate()
        }
    }

    /// Returns progress rate.
    pub fn rate(&self) -> f32 {
        self.counter as f32 / self.elapsed_time
    }

    // -----------------------------------------------------------------------------------------
    // EXTRA FUNCTIONALITIES
    // -----------------------------------------------------------------------------------------

    /// Returns wheter progress is completed or not.
    /// If `total` is `0`, then it always returns `false`.
    pub fn completed(&self) -> bool {
        if self.indefinite() {
            false
        } else {
            self.counter >= self.total
        }
    }

    /// Returns wheter progress is indefinite (total=0) or not.
    pub fn indefinite(&self) -> bool {
        self.total == 0
    }

    /// Returns wheter progress is started (counter=0) or not.
    pub fn started(&self) -> bool {
        self.counter != 0
    }

    // -----------------------------------------------------------------------------------------
    // UPDATE AND PRINTING LOGIC (FOR INTERNAL USE ONLY)
    // -----------------------------------------------------------------------------------------

    /// Checks wheter to trigger a display update or not.
    /// This method will increment internal counter.
    pub fn trigger(&mut self, n: usize) -> bool {
        self.counter += n;

        if !self.disable {
            if self.force_refresh {
                return true;
            }

            let completion_constraint = self.counter == self.total;

            let elapsed_time_now = self.timer.elapsed().as_secs_f32();
            let delay_constraint = self.delay <= elapsed_time_now;
            let mininterval_constraint = self.mininterval <= (elapsed_time_now - self.elapsed_time);

            if self.dynamic_miniters && !mininterval_constraint {
                self.miniters += self.counter;
            }

            let miniters_constraint = if self.miniters <= 1 {
                true
            } else {
                self.counter % self.miniters == 0
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

    /// Adjust number of columns for bar animation using length of remanining bar.
    pub fn adjust_ncols(&mut self, lbar_rbar_len: i16) {
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
    pub fn write_at(&self, text: String) {
        if self.position == 0 {
            self.writer.print(format_args!("\r{}", text));
        } else {
            self.writer.print(format_args!(
                "{}{}\x1b[{}A",
                "\n".repeat(self.position as usize),
                text,
                self.position
            ));
        }
    }

    // -----------------------------------------------------------------------------------------
    // FORMATTING (FOR INTERNAL USE ONLY)
    // -----------------------------------------------------------------------------------------

    pub fn fmt_percentage(&self, precision: usize) -> String {
        format!(
            "{:1$.2$}%",
            self.percentage() * 100.0,
            if precision == 0 { 3 } else { precision + 4 },
            precision
        )
    }

    pub fn fmt_counter(&self) -> String {
        if self.unit_scale {
            format::size_of(self.counter as f64, self.unit_divisor as f64)
        } else {
            format!("{:1$}", self.counter, self.fmt_total().len())
        }
    }

    pub fn fmt_total(&self) -> String {
        if self.unit_scale {
            format::size_of(self.total as f64, self.unit_divisor as f64)
        } else {
            self.total.to_string()
        }
    }

    pub fn fmt_elapsed_time(&self) -> String {
        format::interval(self.elapsed_time as usize, false)
    }

    pub fn fmt_remaining_time(&self) -> String {
        if self.counter == 0 || self.indefinite() {
            "inf".to_owned()
        } else {
            format::interval(self.remaining_time() as usize, false)
        }
    }

    pub fn fmt_rate(&self) -> String {
        if self.counter == 0 {
            format!("?{}/s", self.unit)
        } else {
            let rate = self.rate();

            if rate < 1. && self.inverse_unit {
                format!(
                    "{}/{}",
                    if self.unit_scale {
                        format::time(1. / (rate as f64))
                    } else {
                        format!("{:.2}s", 1. / rate)
                    },
                    self.unit
                )
            } else {
                format!(
                    "{}{}/s",
                    if self.unit_scale {
                        format::size_of(rate as f64, self.unit_divisor as f64)
                    } else {
                        format!("{:.2}", rate)
                    },
                    self.unit
                )
            }
        }
    }
}

impl BarExt for Bar {
    fn clear(&mut self) {
        self.write_at(format!(
            "\r{}",
            " ".repeat(crate::term::get_columns_or(self.bar_length as u16) as usize)
        ));
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
        self.elapsed_time();

        #[cfg(feature = "template")]
        if self.bar_format.is_some() {
            let mut bar_format = self.bar_format.as_ref().unwrap().clone();

            bar_format.replace_with_callback("desc", &self.desc, |fmtval, placeholder| {
                if !self.desc.is_empty() {
                    fmtval
                        + &placeholder
                            .attr("suffix")
                            .unwrap_or_else(|| ": ".to_owned())
                } else {
                    fmtval
                }
            });

            bar_format.replace_from_callback("percentage", |placeholder| {
                placeholder.format_spec.format(self.percentage() * 100.0)
            });

            bar_format.replace_from_callback("count", |placeholder| {
                if self.unit_scale {
                    placeholder.format_spec.format(format::size_of(
                        self.counter as f64,
                        self.unit_divisor as f64,
                    ))
                } else {
                    placeholder.format_spec.format(&self.counter)
                }
            });

            bar_format.replace_from_callback("total", |placeholder| {
                if self.unit_scale {
                    placeholder
                        .format_spec
                        .format(format::size_of(self.total as f64, self.unit_divisor as f64))
                } else {
                    placeholder.format_spec.format(&self.total)
                }
            });

            bar_format.replace_from_callback("elapsed", |placeholder| {
                let human = placeholder
                    .attr("human")
                    .unwrap_or_else(|| "false".to_owned())
                    .parse::<bool>()
                    .unwrap_or(false);
                placeholder
                    .format_spec
                    .format(crate::format::interval(self.elapsed_time as usize, human))
            });

            bar_format.replace_from_callback("remaining", |placeholder| {
                if self.indefinite() {
                    placeholder.format_spec.format("inf")
                } else {
                    let human = placeholder
                        .attr("human")
                        .unwrap_or_else(|| "false".to_owned())
                        .parse::<bool>()
                        .unwrap_or(false);
                    placeholder.format_spec.format(crate::format::interval(
                        self.remaining_time() as usize,
                        human,
                    ))
                }
            });

            bar_format.replace_from_callback("rate", |placeholder| {
                placeholder.format_spec.format(self.rate())
            });

            bar_format.replace("unit", &self.unit);
            bar_format.replace("postfix", &self.postfix);

            #[cfg(feature = "spinner")]
            bar_format.replace_from_callback("spinner", |_| {
                if let Some(spinner) = &self.spinner {
                    spinner.render_frame(self.elapsed_time)
                } else {
                    "".to_owned()
                }
            });

            let length = bar_format.unchecked_text().len_ansi() as i16;
            self.adjust_ncols(length - 11);

            bar_format.replace_from_callback("animation", |_| {
                let render = self.animation.render(self.ncols, self.percentage() as f32);

                if let Some(colour) = &self.colour {
                    return colour.apply(&render);
                }

                render
            });

            return bar_format.text().unwrap();
        }

        let desc = if self.desc.is_empty() {
            "".to_owned()
        } else {
            format!("{}: ", self.desc)
        };

        if self.indefinite() {
            let bar = format!(
                "{}{}{} [{}, {}{}]",
                desc,
                self.fmt_counter(),
                self.unit,
                self.fmt_elapsed_time(),
                self.fmt_rate(),
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

        let progress = self.percentage() as f32;

        if progress >= 1.0 {
            self.total = self.counter;

            if !self.leave && self.position != 0 {
                return format!(
                    "{}\r",
                    " ".repeat(crate::term::get_columns_or(self.bar_length as u16) as usize)
                );
            }
        }

        let lbar = desc + &self.fmt_percentage(0);
        let rbar = format!(
            " {}/{} [{}<{}, {}{}]",
            self.fmt_counter(),
            self.fmt_total(),
            self.fmt_elapsed_time(),
            self.fmt_remaining_time(),
            self.fmt_rate(),
            self.postfix,
        );

        self.adjust_ncols(
            (format!("{}{}", lbar, rbar).len_ansi() + self.animation.spaces() as usize) as i16,
        );

        if self.ncols <= 0 {
            return lbar + &rbar;
        }

        lbar + &self
            .animation
            .fmt_render(self.ncols, progress, &self.colour)
            + &rbar
    }

    fn reset(&mut self, total: Option<usize>) {
        if let Some(x) = total {
            self.total = x;
        }

        self.counter = self.initial;
        self.timer = std::time::Instant::now();
    }

    fn update(&mut self, n: usize) -> bool {
        if self.trigger(n) {
            let text = self.render();
            let length = text.len_ansi() as i16;

            if length != self.bar_length {
                self.clear();
            }

            self.bar_length = length;
            self.write_at(text);
            return true;
        }

        false
    }

    fn update_to(&mut self, update_to_n: usize) -> bool {
        self.counter = update_to_n;
        self.update(0)
    }

    fn write<T: Into<String>>(&mut self, text: T) {
        self.clear();
        self.writer.print(format_args!("\r{}\n", text.into()));

        if self.leave {
            self.refresh();
        }
    }

    fn write_to<T: std::io::Write>(&mut self, writer: &mut T, n: Option<usize>) -> bool {
        let text;

        if let Some(n) = &n {
            if self.trigger(*n) {
                text = self.render().trim_ansi();
            } else {
                return false;
            }
        } else {
            text = self.render().trim_ansi();
        }

        self.bar_length = text.len_ansi() as i16;
        lock::acquire();
        writer.write_fmt(format_args!("{}\n", text)).unwrap();
        writer.flush().unwrap();
        lock::release();
        true
    }
}

/// Create [Bar](crate::Bar) with custom configurations.
///
/// # Example
///
/// ```
/// use kdam::BarBuilder;
///
/// let mut pb = BarBuilder::default().total(100).build();
/// ```
#[derive(Default)]
pub struct BarBuilder {
    pb: Bar,
    #[cfg(feature = "template")]
    bar_format: Option<String>,
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
        self.pb.disable = disable;
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

    /// If true, and the number of iterations per second is less than 1,
    /// then s/it will be displayed instead of it/s.
    /// (default: `false`)
    pub fn inverse_unit(mut self, inverse_unit: bool) -> Self {
        self.pb.inverse_unit = inverse_unit;
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
    /// (default: `None`)
    ///
    /// ## Default Style
    ///
    /// ```text
    /// {desc}{percentage:3.0}%|{animation}| {count}/{total} [{elapsed}<{remaining}, {rate:.2}{unit}/s{postfix}]
    /// ```
    ///
    /// ## Placeholders
    ///
    /// | Placeholder | Attributes                                              | Formatting      |
    /// |-------------|---------------------------------------------------------|-----------------|
    /// | desc        | name: **suffix** <br>                                   | &#10004;        |
    /// |             | description: attaches suffix to desc if desc == "" <br> |                 |
    /// |             | type: string <br>                                       |                 |
    /// |             | default: ": " <br>                                      |                 |
    /// | percentage  |                                                         | &#10004; (true) |
    /// | count       |                                                         | &#10004;        |
    /// | total       |                                                         | &#10004;        |
    /// | elapsed     | name: **human** <br>                                    | &#10004;        |
    /// |             | description: alternative way to display time. <br>      |                 |
    /// |             | type: bool <br>                                         |                 |
    /// |             | default: false <br>                                     |                 |
    /// | remaining   | name: **human** <br>                                    | &#10004;        |
    /// |             | description: alternative way to display time. <br>      |                 |
    /// |             | type: bool <br>                                         |                 |
    /// |             | default: false <br>                                     |                 |
    /// | rate        |                                                         | &#10004; (true) |
    /// | unit        |                                                         | &#10004; (true) |
    /// | postfix     |                                                         | &#10004; (true) |
    /// | spinner     |                                                         | &#10060;        |
    /// | animation   |                                                         | &#10060;        |
    #[cfg(feature = "template")]
    #[cfg_attr(docsrs, doc(cfg(feature = "template")))]
    pub fn bar_format<T: Into<String>>(mut self, bar_format: T) -> Self {
        self.bar_format = Some(bar_format.into());
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
        self.pb.set_postfix(postfix);
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
    pub fn colour<T: Into<Colour>>(mut self, colour: T) -> Self {
        self.pb.colour = Some(colour.into());
        self
    }

    /// Don't display until few seconds have elapsed.
    /// (default: `0`)
    pub fn delay<T: Into<f32>>(mut self, delay: T) -> Self {
        self.pb.delay = delay.into();
        self
    }

    /// Animation style to use with progress bar.
    /// (default: [tqdm](crate::Animation::Tqdm))
    pub fn animation<T: Into<Animation>>(mut self, animation: T) -> Self {
        self.pb.animation = animation.into();
        self
    }

    /// Spinner to use with progress bar.
    /// Spinner is only used when `bar_format` is used.
    /// (default: `None`)
    #[cfg(feature = "spinner")]
    #[cfg_attr(docsrs, doc(cfg(feature = "spinner")))]
    pub fn spinner(mut self, spinner: Spinner) -> Self {
        self.pb.spinner = Some(spinner);
        self
    }

    /// Select writer type to display progress bar output between `stdout` and `stderr`.
    /// (default: [stderr](crate::term::Writer))
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

    /// Build [Bar](crate::Bar), this method only returns error when `bar_format` is used incorrectly.
    #[allow(unused_mut)]
    pub fn build(mut self) -> Result<Bar, String> {
        #[cfg(feature = "template")]
        if let Some(bar_format) = self.bar_format {
            self.pb
                .set_bar_format(bar_format)
                .map_err(|e| e.message())?;
        }

        Ok(self.pb.init())
    }
}

/// [tqdm](https://github.com/tqdm/tqdm) like macro for constructing [BarIterator](crate::BarIterator) if iterable is given else [Bar](crate::Bar).
///
/// This macro use [BarBuilder](crate::BarBuilder) for creating [Bar](crate::Bar).
/// See all available [methods](crate::BarBuilder).
///
/// # Panics
///
/// This macro will panic if [BarBuilder::build](crate::BarBuilder::build) returns error.
///
/// # Examples
///
/// ```
/// use kdam::{tqdm, BarExt};
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
        $crate::BarBuilder::default()$(.$setter_method($value))*.build().unwrap()
    };

    ($iterable: expr) => {
        $crate::BarIterator::new_with_bar($iterable, kdam::Bar::default())
    };

    ($iterable: expr, $($setter_method: ident = $value: expr),*) => {
        $crate::BarIterator::new_with_bar($iterable, kdam::BarBuilder::default()$(.$setter_method($value))*.build().unwrap())
    };
}
