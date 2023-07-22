use super::{
    styles::{Animation, Colour},
    BarExt,
};
use crate::{
    format, term,
    term::{Colorizer, Writer},
};
use std::{
    io::{stdin, Result, Write},
    num::NonZeroU16,
    time::Instant,
};

#[cfg(feature = "spinner")]
use crate::spinner::Spinner;

#[cfg(feature = "template")]
use formatx::Template;

/// Core implemention of console progress bar.
///
/// # Example
///
/// ```
/// use kdam::{tqdm, Bar, BarExt};
///
/// let mut pb = Bar::new(100);
/// // let mut pb = tqdm!(total = 100);
/// // let mut pb = Bar::builder().total(100).build().unwrap();
///
/// for _ in 0..100 {
///     pb.update(1).unwrap();
/// }
/// ```
#[derive(Debug)]
pub struct Bar {
    // Builder Fields
    pub animation: Animation,
    #[cfg(feature = "template")]
    pub bar_format: Option<Template>,
    pub colour: Option<Colour>,
    pub desc: String,
    pub delay: f32,
    pub disable: bool,
    pub dynamic_miniters: bool,
    pub dynamic_ncols: bool,
    pub force_refresh: bool,
    pub inverse_unit: bool,
    pub leave: bool,
    pub mininterval: f32,
    pub miniters: usize,
    pub ncols: Option<u16>,
    pub position: u16,
    pub postfix: String,
    pub total: usize,
    #[cfg(feature = "spinner")]
    pub spinner: Option<Spinner>,
    pub unit: String,
    pub unit_divisor: usize,
    pub unit_scale: bool,
    pub writer: Writer,
    // Non Builder Fields
    pub bar_length: u16,
    pub counter: usize,
    current_ncols: u16,
    elapsed_time: f32,
    timer: Instant,
}

impl Default for Bar {
    fn default() -> Self {
        let mut ncols = None;

        if let Ok(Ok(x)) = std::env::var("KDAM_NCOLS").map(|x| x.parse::<u16>()) {
            ncols = Some(x);
        }

        Self {
            animation: Animation::Tqdm,
            #[cfg(feature = "template")]
            bar_format: None,
            colour: None,
            delay: 0.0,
            desc: "".to_owned(),
            disable: false,
            dynamic_miniters: false,
            dynamic_ncols: false,
            force_refresh: false,
            inverse_unit: false,
            leave: true,
            mininterval: 0.1,
            miniters: 1,
            ncols,
            total: 0,
            position: 0,
            postfix: "".to_string(),
            #[cfg(feature = "spinner")]
            spinner: None,
            unit: "it".to_owned(),
            unit_divisor: 1000,
            unit_scale: false,
            writer: Writer::Stderr,
            bar_length: 0,
            counter: 0,
            current_ncols: 0,
            elapsed_time: 0.0,
            timer: Instant::now(),
        }
    }
}

impl Bar {
    // -----------------------------------------------------------------------------------------
    // Constructors
    // -----------------------------------------------------------------------------------------

    /// Create a new [Bar](Self) with a total value.
    ///
    /// # Example
    ///
    /// ```
    /// use kdam::Bar;
    ///
    /// let pb = Bar::new(100);
    /// ```
    pub fn new(total: usize) -> Self {
        Self {
            total,
            ..Default::default()
        }
    }

    /// Create a new [BarBuilder](crate::BarBuilder).
    ///
    /// # Example
    ///
    /// ```
    /// use kdam::Bar;
    ///
    /// let pb = Bar::builder().total(100).build().unwrap();
    /// ```
    pub fn builder() -> BarBuilder {
        BarBuilder::default()
    }

    // -----------------------------------------------------------------------------------------
    // Setters
    // -----------------------------------------------------------------------------------------

    /// Set/Modify [bar_format](Self::bar_format) property.
    #[cfg(feature = "template")]
    #[cfg_attr(docsrs, doc(cfg(feature = "template")))]
    pub fn set_bar_format<T: Into<String>>(
        &mut self,
        bar_format: T,
    ) -> ::std::result::Result<(), String> {
        let bar_format = bar_format
            .into()
            .parse::<Template>()
            .map_err(|x| x.message())?;
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
        bar_format_check.text().map_err(|x| x.message())?;
        self.bar_format = Some(bar_format);
        Ok(())
    }

    /// Set/Modify [description](Self::desc) property.
    pub fn set_description<T: Into<String>>(&mut self, description: T) {
        self.desc = description.into();
    }

    /// Set/Modify [postfix](Self::postfix) property.
    pub fn set_postfix<T: Into<String>>(&mut self, postfix: T) {
        self.postfix = ", ".to_owned() + &postfix.into();
    }

    // -----------------------------------------------------------------------------------------
    // Methods
    // -----------------------------------------------------------------------------------------

    /// Returns whether progress is completed or not.
    ///
    /// If `total` is `0`, it always returns `false`.
    pub fn completed(&self) -> bool {
        if self.indefinite() {
            false
        } else {
            self.counter >= self.total
        }
    }

    /// Set and returns progress elapsed time.
    pub fn elapsed_time(&mut self) -> f32 {
        self.elapsed_time = self.timer.elapsed().as_secs_f32();
        self.elapsed_time
    }

    /// Returns formatted counter value.
    pub fn fmt_counter(&self) -> String {
        if self.unit_scale {
            format::size_of(self.counter as f64, self.unit_divisor as f64)
        } else {
            format!("{:1$}", self.counter, self.fmt_total().len())
        }
    }

    /// Returns formatted elapsed time.
    pub fn fmt_elapsed_time(&self) -> String {
        format::interval(self.elapsed_time as usize, false)
    }

    /// Returns formatted progress percentage.
    pub fn fmt_percentage(&self, precision: usize) -> String {
        format!(
            "{:1$.2$}%",
            self.percentage() * 100.0,
            if precision == 0 { 3 } else { precision + 4 },
            precision
        )
    }

    /// Returns formatted progress rate.
    pub fn fmt_rate(&self) -> String {
        if !self.started() {
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

    /// Returns formatted remaining time.
    pub fn fmt_remaining_time(&self) -> String {
        if self.counter == 0 || self.indefinite() {
            "inf".to_owned()
        } else {
            format::interval(self.remaining_time() as usize, false)
        }
    }

    /// Returns formatted total value.
    pub fn fmt_total(&self) -> String {
        if self.unit_scale {
            format::size_of(self.total as f64, self.unit_divisor as f64)
        } else {
            self.total.to_string()
        }
    }

    /// Returns whether progress is indefinite i.e. `total` is `0` or not.
    pub fn indefinite(&self) -> bool {
        self.total == 0
    }

    /// Set and returns number of columns for bar animation with given padding.
    pub fn ncols_for_animation(&mut self, padding: u16) -> u16 {
        if self.dynamic_ncols || ((padding + self.current_ncols) != self.bar_length) {
            if let Some(ncols) = self.ncols {
                self.current_ncols = ncols;
            } else if let Some(width) = term::width() {
                if width >= padding {
                    self.current_ncols = width - padding;
                }
            } else {
                self.current_ncols = 10;
            }
        }

        self.current_ncols
    }

    /// Returns progress percentage, like `0.62`, `0.262`, `1.0`.
    ///
    /// If `total` is `0`, it always returns `1.0`.
    pub fn percentage(&self) -> f32 {
        if self.indefinite() {
            1.0
        } else {
            (self.counter as f64 / self.total as f64) as f32
        }
    }

    /// Returns progress/iterations rate.
    ///
    /// # Note
    ///
    /// Before calling this method, [elapsed_time](crate::Bar::elapsed_time) method should be called.
    pub fn rate(&self) -> f32 {
        self.counter as f32 / self.elapsed_time
    }

    /// Returns remaining time (ETA) for progress completion.
    ///
    /// If `total` is `0`, it always returns infinity.
    pub fn remaining_time(&self) -> f32 {
        if self.indefinite() {
            f32::INFINITY
        } else {
            (self.total - self.counter) as f32 / self.rate()
        }
    }

    /// Returns whether to trigger a display update or not.
    pub fn should_refresh(&mut self) -> bool {
        if !self.disable {
            if self.force_refresh {
                return true;
            }

            let elapsed_time_now = self.timer.elapsed().as_secs_f32();
            let completion_constraint = self.counter == self.total;
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

    /// Returns whether progress is started i.e. `counter` is `0` or not.
    pub fn started(&self) -> bool {
        self.counter > 0
    }
}

impl BarExt for Bar {
    fn clear(&mut self) -> Result<()> {
        self.writer.print_at(
            self.position,
            " ".repeat(term::width().unwrap_or(self.bar_length) as usize)
                .as_bytes(),
        )
    }

    fn input<T: Into<String>>(&mut self, text: T) -> Result<String> {
        self.clear()?;
        self.writer.print(text.into().as_bytes())?;

        let mut buf = String::new();
        stdin().read_line(&mut buf)?;

        if self.leave {
            self.refresh()?;
        }

        Ok(buf)
    }

    fn refresh(&mut self) -> Result<()> {
        self.elapsed_time();

        if self.completed() {
            if !self.leave && self.position > 0 {
                return self.clear();
            }

            self.total = self.counter;
        }

        let text = self.render();
        let bar_length = text.len_ansi() as u16;

        if bar_length > self.bar_length {
            self.clear()?;
            self.bar_length = bar_length;
        }

        self.writer.print_at(self.position, text.as_bytes())?;
        Ok(())
    }

    fn render(&mut self) -> String {
        #[cfg(feature = "template")]
        if let Some(bar_format) = &self.bar_format {
            let mut bar_format = bar_format.clone();

            bar_format.replace_with_callback("desc", &self.desc, |fmtval, placeholder| {
                if self.desc.is_empty() {
                    fmtval
                } else {
                    fmtval
                        + &placeholder
                            .attr("suffix")
                            .unwrap_or_else(|| ": ".to_owned())
                }
            });

            bar_format.replace_from_callback("percentage", |placeholder| {
                placeholder.format_spec.format(self.percentage() * 100.)
            });

            bar_format.replace_from_callback("count", |placeholder| {
                if self.unit_scale {
                    placeholder.format_spec.format(format::size_of(
                        self.counter as f64,
                        self.unit_divisor as f64,
                    ))
                } else {
                    placeholder.format_spec.format(self.counter)
                }
            });

            bar_format.replace_from_callback("total", |placeholder| {
                if self.unit_scale {
                    placeholder
                        .format_spec
                        .format(format::size_of(self.total as f64, self.unit_divisor as f64))
                } else {
                    placeholder.format_spec.format(self.total)
                }
            });

            bar_format.replace_from_callback("elapsed", |placeholder| {
                let human = placeholder
                    .attr("human")
                    .and_then(|x| x.parse::<bool>().ok())
                    .unwrap_or(false);
                placeholder
                    .format_spec
                    .format(format::interval(self.elapsed_time as usize, human))
            });

            bar_format.replace_from_callback("remaining", |placeholder| {
                if self.indefinite() {
                    placeholder.format_spec.format("inf")
                } else {
                    let human = placeholder
                        .attr("human")
                        .and_then(|x| x.parse::<bool>().ok())
                        .unwrap_or(false);
                    placeholder
                        .format_spec
                        .format(format::interval(self.remaining_time() as usize, human))
                }
            });

            // inverse_unit field is not considered here.
            bar_format.replace_from_callback("rate", |placeholder| {
                if self.unit_scale {
                    placeholder.format_spec.format(format::size_of(
                        self.rate() as f64,
                        self.unit_divisor as f64,
                    ))
                } else {
                    placeholder.format_spec.format(self.rate())
                }
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

            let length = bar_format.unchecked_text().len_ansi() as u16;

            if bar_format.contains("animation") && length > 11 {
                let ncols = self.ncols_for_animation(length - 11);

                if ncols > 0 {
                    bar_format.replace_from_callback("animation", |_| {
                        let render = self
                            .animation
                            .render(NonZeroU16::new(ncols).unwrap(), self.percentage());

                        if let Some(colour) = &self.colour {
                            colour.apply(&render)
                        } else {
                            render
                        }
                    });
                }
            } else {
                bar_format.replace("animation", "");
            }

            return bar_format.text().unwrap(); // This should not panic.
        }

        let desc = if self.desc.is_empty() {
            "".to_owned()
        } else {
            self.desc.clone() + ": "
        };

        if self.indefinite() {
            format!(
                "{}{}{} [{}, {}{}]",
                desc,
                self.fmt_counter(),
                self.unit,
                self.fmt_elapsed_time(),
                self.fmt_rate(),
                self.postfix
            )
        } else {
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

            let ncols = self.ncols_for_animation(
                (lbar.len_ansi() + rbar.len_ansi() + self.animation.spaces() as usize) as u16,
            );

            if ncols > 0 {
                lbar + &self.animation.fmt_render(
                    NonZeroU16::new(ncols).unwrap(),
                    self.percentage(),
                    &self.colour,
                ) + &rbar
            } else {
                lbar + &rbar
            }
        }
    }

    fn reset(&mut self, total: Option<usize>) {
        if let Some(x) = total {
            self.total = x;
        }

        self.counter = 0;
        self.timer = Instant::now();
    }

    fn update(&mut self, n: usize) -> Result<bool> {
        self.counter += n;
        let should_refresh = self.should_refresh();

        if should_refresh {
            self.refresh()?;
        }

        Ok(should_refresh)
    }

    fn update_to(&mut self, n: usize) -> Result<bool> {
        self.counter = n;
        self.update(0)
    }

    fn write<T: Into<String>>(&mut self, text: T) -> Result<()> {
        self.clear()?;
        self.writer
            .print(format!("\r{}\n", text.into()).as_bytes())?;

        if self.leave {
            self.refresh()?;
        }

        Ok(())
    }

    fn write_to<T: Write>(&mut self, writer: &mut T, n: Option<usize>) -> Result<bool> {
        let text;

        if let Some(n) = n {
            self.counter += n;

            if self.should_refresh() {
                text = self.render().trim_ansi();
            } else {
                return Ok(false);
            }
        } else {
            text = self.render().trim_ansi();
        }

        self.bar_length = text.len_ansi() as u16;
        writer.write_all((text + "\n").as_bytes())?;
        writer.flush()?;
        Ok(true)
    }
}

/// Create [Bar](crate::Bar) with custom configurations.
///
/// # Example
///
/// ```
/// use kdam::BarBuilder;
///
/// let mut pb = BarBuilder::default().total(100).build().unwrap();
/// ```
#[derive(Default)]
pub struct BarBuilder {
    pb: Bar,
    #[cfg(feature = "template")]
    bar_format: Option<String>,
}

impl BarBuilder {
    /// Prefix for progress bar.
    /// (default: `""`)
    pub fn desc<T: Into<String>>(mut self, desc: T) -> Self {
        self.pb.desc = desc.into();
        self
    }

    /// The number of expected iterations.
    /// If `unspecified`, `iterable.size_hint().0` is used if possible.
    /// If `0`, only basic progress statistics are displayed (no ETA, no progressbar).
    /// (default: `0`)
    pub fn total(mut self, total: usize) -> Self {
        self.pb.total = total;
        self
    }

    /// If `true`, keeps all traces of the progress bar upon termination.
    /// If `false`, will leave only if position is `0`.
    /// (default: `true`)
    pub fn leave(mut self, leave: bool) -> Self {
        self.pb.leave = leave;
        self
    }

    /// The width of the entire output message.
    /// If `specified`, dynamically resizes the progress bar to stay within this bound.
    /// If `unspecified`, attempts to use `KDAM_NCOLS` environment variable or adjust width automatically.
    /// If `0`, will not print any meter (only stats).
    /// The fallback is a meter width of `10` and no limit for the counter and statistics.
    /// (default: `10`)
    pub fn ncols(mut self, ncols: u16) -> Self {
        self.pb.ncols = Some(ncols);
        self
    }

    /// Minimum progress display update interval (in seconds).
    /// (default: `0.1`)
    pub fn mininterval(mut self, mininterval: f32) -> Self {
        self.pb.mininterval = mininterval;
        self
    }

    /// Minimum progress display update interval (in iterations).
    /// If `miniters > 0`, specified number of iterations will be skipped and not displayed.
    /// If your progress is erratic with both fast and slow iterations (network, skipping items, etc.) you should keep it default.
    /// Tweak this and [mininterval](Self::mininterval) to get very efficient loops.
    /// (default: `1`)
    pub fn miniters(mut self, miniters: usize) -> Self {
        self.pb.miniters = miniters;
        self
    }

    /// Automatically adjusts [miniters](Self::miniters) to correspond to [mininterval](Self::mininterval) after long display update lag.
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

    /// Unit that will be used to define the unit of each iteration.
    /// (default: `"it"`)
    pub fn unit<T: Into<String>>(mut self, unit: T) -> Self {
        self.pb.unit = unit.into();
        self
    }

    /// If `true`, the number of iterations will be reduced/scaled automatically
    /// and a metric prefix following the [International System of Units](https://en.wikipedia.org/wiki/Metric_prefix) standard will be added (kilo, mega, etc.).
    /// (default: `false`)
    pub fn unit_scale(mut self, unit_scale: bool) -> Self {
        self.pb.unit_scale = unit_scale;
        self
    }

    /// If `true`, and the number of iterations per second is less than 1
    /// then `s/it` will be displayed instead of `it/s`.
    /// (default: `false`)
    pub fn inverse_unit(mut self, inverse_unit: bool) -> Self {
        self.pb.inverse_unit = inverse_unit;
        self
    }

    /// If `true`, constantly alters [ncols](Self::ncols) to the environment (allowing for window resizes).
    /// (default: `false`)
    pub fn dynamic_ncols(mut self, dynamic_ncols: bool) -> Self {
        self.pb.dynamic_ncols = dynamic_ncols;
        self
    }

    /// The initial counter value.
    /// (default: `0`)
    pub fn initial(mut self, initial: usize) -> Self {
        self.pb.counter = initial;
        self
    }

    /// Specify a custom progress bar format (may impact performance).
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

    /// Specify the line offset to print this progress bar (starting from `0`).
    /// Useful for managing multiple progress bars at once (eg. from threads).
    /// (default: `0`)
    ///
    /// # Platform-specific notes
    ///
    /// On windows, [term::init](crate::term::init) method should be called first.
    pub fn position(mut self, position: u16) -> Self {
        self.pb.position = position;
        self
    }

    /// Specify additional stats to display at the end of the progress bar.
    /// (default: `""`)
    pub fn postfix<T: Into<String>>(mut self, postfix: T) -> Self {
        self.pb.set_postfix(postfix);
        self
    }

    /// Divide values by this [unit_divisor](Self::unit_divisor).
    /// It is ignored unless [unit_scale](Self::unit_scale) is true.
    /// (default: `1000`)
    pub fn unit_divisor(mut self, unit_divisor: usize) -> Self {
        self.pb.unit_divisor = unit_divisor;
        self
    }

    /// Progress bar colour (e.g. "green", "#00ff00").
    /// (default: `None`)
    pub fn colour<T: Into<Colour>>(mut self, colour: T) -> Self {
        self.pb.colour = Some(colour.into());
        self
    }

    /// Don't display progress bar until few seconds have elapsed.
    /// (default: `0`)
    pub fn delay<T: Into<f32>>(mut self, delay: T) -> Self {
        self.pb.delay = delay.into();
        self
    }

    /// Animation style to display progress bar.
    /// (default: [Animation::Tqdm](crate::Animation::Tqdm))
    pub fn animation<T: Into<Animation>>(mut self, animation: T) -> Self {
        self.pb.animation = animation.into();
        self
    }

    /// Spinner to use with progress bar.
    /// Spinner is only used when [bar_format](Self::bar_format) is used.
    /// (default: `None`)
    #[cfg(feature = "spinner")]
    #[cfg_attr(docsrs, doc(cfg(feature = "spinner")))]
    pub fn spinner(mut self, spinner: Spinner) -> Self {
        self.pb.spinner = Some(spinner);
        self
    }

    /// Select writer between `stdout` and `stderr` to display progress bar output.
    /// (default: [Writer::Stderr](crate::term::Writer))
    pub fn writer(mut self, writer: Writer) -> Self {
        self.pb.writer = writer;
        self
    }

    /// If `true`, each progress bar update method call will be displayed.
    /// (default: `false`)
    pub fn force_refresh(mut self, force_refresh: bool) -> Self {
        self.pb.force_refresh = force_refresh;
        self
    }

    /// Build a new [Bar](crate::Bar) with custom configurations.
    ///
    /// # Note
    ///
    /// This method only returns error when `bar_format` is used incorrectly.
    #[allow(unused_mut)]
    pub fn build(mut self) -> ::std::result::Result<Bar, String> {
        #[cfg(feature = "template")]
        if let Some(bar_format) = self.bar_format {
            self.pb.set_bar_format(bar_format)?;
        }

        Ok(self.pb)
    }
}

/// [tqdm](https://github.com/tqdm/tqdm) like macro for creating [Bar](crate::Bar) and [BarIter](crate::BarIter).
///
/// It uses [BarBuilder](crate::BarBuilder) for creating [Bar](crate::Bar).
/// See, all available [methods](crate::BarBuilder).
///
/// # Panics
///
/// This macro will panic if [BarBuilder::build](crate::BarBuilder::build) returns error.
///
/// # Examples
///
/// ```
/// use kdam::tqdm;
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
        $crate::TqdmIterator::tqdm($iterable)
    };

    ($iterable: expr, $($setter_method: ident = $value: expr),*) => {
        $crate::TqdmIterator::tqdm_with_bar($iterable, $crate::BarBuilder::default()$(.$setter_method($value))*.build().unwrap())
    };
}

/// Parallel version of [tqdm](crate::tqdm) macro.
///
/// # Panics
///
/// This macro will panic if [BarBuilder::build](crate::BarBuilder::build) returns error.
///
/// # Examples
///
/// ```
/// use kdam::{rayon::prelude::*, par_tqdm};
///
/// par_tqdm!();
/// par_tqdm!(total = 100);
/// par_tqdm!(total = 100, mininterval = 0.0, colour = "green");
/// par_tqdm!((0..100).into_par_iter());
/// par_tqdm!((0..100).into_par_iter(), desc = "0 to 99");
/// par_tqdm!(["a", "b", "c", "d"].par_iter());
/// ```
#[cfg(feature = "rayon")]
#[cfg_attr(docsrs, doc(cfg(feature = "rayon")))]
#[macro_export]
macro_rules! par_tqdm {
    ($($setter_method: ident = $value: expr),*) => {
        $crate::BarBuilder::default()$(.$setter_method($value))*.build().unwrap()
    };

    ($iterable: expr) => {
        $crate::TqdmParallelIterator::tqdm($iterable)
    };

    ($iterable: expr, $($setter_method: ident = $value: expr),*) => {
        $crate::TqdmParallelIterator::tqdm_with_bar($iterable, $crate::BarBuilder::default()$(.$setter_method($value))*.build().unwrap())
    };
}
