use std::io::Write;

use crate::format;
use crate::internal::BarInternal;
use crate::term;
use crate::Animation;

#[derive(Debug)]
pub struct Bar {
    pub desc: String,
    pub total: u64,
    pub leave: bool,
    pub file: Option<std::fs::File>,
    pub ncols: i16,
    pub mininterval: f64,
    pub miniters: u64,
    pub ascii: bool,
    pub disable: bool,
    pub unit: String,
    pub unit_scale: bool,
    pub dynamic_ncols: bool,
    pub initial: u64,
    pub postfix: String,
    pub unit_divisor: u64,
    pub colour: String,
    pub delay: f64,
    pub fill: String,
    pub animation: Animation,
    // internal
    pub i: u64,
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
    pub fn new(total: u64) -> Self {
        Bar {
            total: total,
            ..Default::default()
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
            spacing = ""
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

    /// render progress bar text using given value.
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

    /// manually update the progress bar, useful for streams such as reading files.
    pub fn update(&mut self, i: u64) {
        if !self.internal.started {
            term::init();
            self.internal.timer = std::time::Instant::now();
            self.internal.started = true;
        }

        let mut force_refresh = false;
        let interval = self.internal.timer.elapsed().as_secs_f64() - self.internal.elapsed_time;
        self.i += i;

        if self.i == self.total || i == 0 {
            force_refresh = true;
        }

        if ((!self.disable)
            && (self.mininterval <= interval)
            && (self.delay <= self.internal.timer.elapsed().as_secs_f64()))
            && (self.i % self.miniters == 0)
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

    /// restart tqdm timer from last print time.
    pub fn unpause(&mut self) {
        self.internal.elapsed_time = self
            .internal
            .timer
            .duration_since(self.internal.timer)
            .as_secs_f64()
            - self.internal.elapsed_time;
    }

    /// clear current bar display.
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

    /// force refresh the display of this bar.
    pub fn refresh(&mut self) {
        self.update(0);
    }

    /// resets to 0 iterations for repeated use.
    /// consider combining with `leave: true`.
    pub fn reset(&mut self, total: Option<u64>) {
        self.internal.started = false;
        self.i = self.initial;

        if total.is_some() {
            self.total = total.unwrap();
        }
    }

    ///  print a message via tqdm (without overlap with bars).
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

    /// set/modify description of the progress bar.
    pub fn set_description(&mut self, desc: &str, refresh: bool) {
        self.desc = String::from(desc);
        if refresh {
            self.refresh();
        }
    }

    /// set/modify postfix (additional stats) with automatic formatting based on datatype.
    pub fn set_postfix(&mut self, postfix: &str, refresh: bool) {
        self.postfix = format!(", {}", postfix);
        if refresh {
            self.refresh();
        }
    }

    /// set/modify colour of the progress bar.
    pub fn set_colour(&mut self, colour: &str) {
        if self.colour != "default" {
            self.colour = term::colour(colour);
        }
    }

    pub fn set_charset(&mut self, charset: &[&str]) {
        self.internal.charset = charset.join("");
        self.internal.charset_len = charset.len() as u64;
        self.animation = Animation::TqdmAscii;
    }

    /// set/modify animation style of the progress bar.
    pub fn set_animation(&mut self, animation: Animation) {
        self.animation = animation;

        if matches!(self.animation, Animation::TqdmAscii) || self.ascii {
            self.set_charset(&["1", "2", "3", "4", "5", "6", "7", "8", "9", "#"]);
        } else if matches!(self.animation, Animation::Tqdm) {
            self.set_charset(&[
                "\u{258F}", "\u{258E}", "\u{258D}", "\u{258C}", "\u{258B}", "\u{258A}", "\u{2589}",
                "\u{2588}",
            ])
        } else if matches!(self.animation, Animation::FillUp) {
            self.set_charset(&[
                "\u{2581}", "\u{2582}", "\u{2583}", "\u{2584}", "\u{2585}", "\u{2586}", "\u{2587}",
                "\u{2588}",
            ])
        } else if matches!(self.animation, Animation::Classic) {
            self.internal.charset = "#".to_string();
            self.fill = ".".to_string();
        } else if matches!(self.animation, Animation::Arrow) {
            self.internal.charset = "=".to_string();
        }
    }

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

// unsafe impl Sync for Bar {}
