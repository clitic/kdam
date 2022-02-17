use std::io::{self, Write};
use std::sync::mpsc;
use std::time::Instant;

use terminal_size::{terminal_size, Height, Width};

use crate::fmt_data;
use crate::iterator_bar::BarIter;
use crate::term;
use crate::Animation;

#[derive(Debug)]
pub struct Bar {
    pub desc: String,
    pub total: usize,
    pub leave: bool,
    pub file: Option<std::fs::File>,
    pub ncols: u16,
    pub mininterval: f64,
    pub ascii: bool,
    pub disable: bool,
    pub unit: String,
    pub unit_scale: bool,
    pub dynamic_ncols: bool,
    pub initial: usize,
    pub postfix: String,
    pub unit_divisor: usize,
    pub colour: String,
    pub delay: f64,
    pub fill: String,
    pub animation: Animation,
    // internal
    pub i: usize,
    pub started: bool,
    pub elapsed_time: f64,
    pub its_per: f64,
    pub bar_length: u16,
    pub user_ncols: u16,
    pub charset: String,
    pub charset_len: usize,
    pub timer: Instant,
    pub stdout: io::Stdout,
    pub nrows: u16,
    pub tx: Option<mpsc::Sender<(u16, String, bool)>>,
}

impl Default for Bar {
    fn default() -> Bar {
        let mut bar = Bar {
            desc: "".to_string(),
            total: 0,
            leave: true,
            file: None,
            ncols: 10,
            mininterval: 0.1,
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
            animation: Animation::Tqdm,
            // internal
            i: 0,
            started: false,
            elapsed_time: 0.0,
            its_per: 0.0,
            bar_length: 0,
            user_ncols: 999,
            charset: "123456789#".to_string(),
            charset_len: 0,
            timer: Instant::now(),
            stdout: io::stdout(),
            nrows: 999,
            tx: None,
        };
        bar.set_animation(Animation::Tqdm);
        bar
    }
}

impl Bar {
    pub fn new(total: usize) -> Bar {
        Bar {
            total: total,
            ..Default::default()
        }
    }

    /// fake from function
    pub fn from_iterator<I: Iterator>(iterable: I) -> BarIter<I> {
        let total = iterable.size_hint().0;
        BarIter {
            iterable: iterable,
            pb: Bar {
                total: total,
                ..Default::default()
            },
            rendered_once: false,
        }
    }

    fn render_unknown(&mut self, i: usize) -> String {
        let desc_spacing = if self.desc == "" { "" } else { ": " };
        self.elapsed_time = self.timer.elapsed().as_secs_f64();
        self.its_per = i as f64 / self.elapsed_time;
        let elapsed_time_fmt = fmt_data::format_interval(self.elapsed_time as u64);

        let count = if self.unit_scale {
            fmt_data::format_sizeof(i, self.unit_divisor)
        } else {
            format!("{}", i)
        };

        let rate_fmt = if self.unit_scale {
            fmt_data::format_sizeof(self.its_per as usize, self.unit_divisor)
        } else {
            format!("{:.2}", self.its_per).to_string()
        };

        return format!(
            "{}{}{} [{}, {}{}/s{}]",
            self.desc, desc_spacing, count, elapsed_time_fmt, rate_fmt, self.unit, self.postfix
        );
    }

    fn render_lbar(&mut self, i: usize) -> (f64, String) {
        let mut progress = (i as f64) / (self.total as f64);

        if progress >= 1.0 {
            progress = 1.0;
        }

        let desc_spacing = if self.desc == "" { "" } else { ": " };
        let percentage = (progress * 100.0) as usize;
        let mut spacing = if percentage >= 10 { " " } else { "  " };

        if progress >= 1.0 {
            spacing = ""
        }

        return (
            progress,
            format!("{}{}{}{}%", self.desc, desc_spacing, spacing, percentage),
        );
    }

    fn render_rbar(&mut self, i: usize) -> String {
        let count = if self.unit_scale {
            fmt_data::format_sizeof(i, self.unit_divisor)
        } else {
            format!("{}", i)
        };

        let total = if self.unit_scale {
            fmt_data::format_sizeof(self.total, self.unit_divisor)
        } else {
            format!("{}", self.total)
        };

        self.elapsed_time = self.timer.elapsed().as_secs_f64();
        self.its_per = i as f64 / self.elapsed_time;

        let remaning_time = (self.total - i) as f64 / self.its_per;

        let elapsed_time_fmt = fmt_data::format_interval(self.elapsed_time as u64);
        let mut remaning_time_fmt = fmt_data::format_interval(remaning_time as u64);
        let mut rate_fmt = if self.unit_scale {
            fmt_data::format_sizeof(self.its_per as usize, self.unit_divisor)
        } else {
            format!("{:.2}", self.its_per).to_string()
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

    fn set_ncols(&mut self, lbar_rbar_len: u16) {
        if self.dynamic_ncols
            || ((lbar_rbar_len + self.ncols + 2) as i16 - self.bar_length as i16) > 0
        {
            if self.user_ncols != 999 {
                self.ncols = self.user_ncols;
            } else {
                let columns = terminal_size().unwrap_or((Width(0), Height(0))).0 .0;

                if columns != 0 {
                    let new_ncols = columns as i16 - lbar_rbar_len as i16 - 3;
                    if new_ncols >= 0 {
                        self.ncols = new_ncols as u16;
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
            let nsyms = self.charset_len - 1;
            let (bar_length, frac_bar_length) = fmt_data::divmod(
                (progress * self.ncols as f64 * nsyms as f64) as u64,
                nsyms as u64,
            );
            bar_animation = self
                .charset
                .chars()
                .nth_back(0)
                .unwrap()
                .to_string()
                .repeat(bar_length as usize);

            if bar_length < self.ncols as u64 {
                bar_animation += &self
                    .charset
                    .chars()
                    .nth((frac_bar_length as usize) + 1)
                    .unwrap()
                    .to_string();
                bar_animation += &self
                    .fill
                    .repeat((self.ncols - (bar_length as u16) - 1) as usize);
            }
        } else {
            let block = (self.ncols as f64 * progress) as u16;
            bar_animation = self.charset.repeat(block as usize);
            if matches!(self.animation, Animation::Classic) {
                bar_animation += &self.fill.repeat((self.ncols - block) as usize);
            } else if matches!(self.animation, Animation::Arrow) {
                let x = (self.ncols as i16) - (block as i16) - 1;
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
    fn render(&mut self, mut i: usize) -> (String, String, String) {
        let (progress, lbar) = self.render_lbar(i);

        if progress == 1.0 {
            i = self.total;

            if !self.leave {
                return (
                    " ".repeat(self.bar_length as usize).to_string(),
                    "".to_string(),
                    "\r".to_string(),
                );
            }
        }

        let rbar = self.render_rbar(i);

        self.set_ncols(format!("\r{}{}", lbar, rbar).len() as u16);

        if self.ncols == 0 || self.ncols < 0 as u16 {
            return (lbar, "".to_string(), rbar);
        }

        let mbar = self.render_mbar(progress);

        return (lbar, mbar, rbar);
    }

    /// manually update the progress bar, useful for streams such as reading files.
    pub fn update(&mut self, i: usize) {
        if !self.started {
            term::init();
            self.timer = Instant::now();
            self.started = true;
        }

        let mut force_refresh = false;
        let interval = self.timer.elapsed().as_secs_f64() - self.elapsed_time as f64;
        self.i += i;

        if self.i == self.total || i == 0 {
            force_refresh = true;
        }

        if ((!self.disable)
            && (self.mininterval <= interval)
            && (self.delay <= self.timer.elapsed().as_secs_f64()))
            || force_refresh
        {
            let text: String;

            if self.total != 0 {
                let (lbar, mbar, rbar) = self.render(self.i);
                text = format!("{}{}{}", lbar, mbar, rbar);
                self.bar_length = format!("\r{}{}", lbar, rbar).len() as u16 + self.ncols + 2;
            } else {
                text = self.render_unknown(self.i);
                self.bar_length = text.len() as u16;
            }

            if self.file.is_none() {
                if self.nrows == 999 {
                    self.stdout
                        .write_fmt(format_args!("\r{}", text.as_str()))
                        .unwrap();
                    self.stdout.flush().unwrap();
                } else {
                    if self.tx.is_some() {
                        self.tx
                            .as_ref()
                            .unwrap()
                            .send((self.nrows, text, self.i == self.total))
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
        self.elapsed_time = self.timer.duration_since(self.timer).as_secs_f64() - self.elapsed_time;
    }

    /// clear current bar display.
    pub fn clear(&mut self) {
        self.stdout
            .write_fmt(format_args!("\r{}\r", " ".repeat(self.bar_length as usize)))
            .unwrap();
        self.stdout.flush().unwrap();
    }

    /// force refresh the display of this bar.
    pub fn refresh(&mut self) {
        self.update(0);
    }

    /// resets to 0 iterations for repeated use.
    /// consider combining with `leave: true`.
    pub fn reset(&mut self, total: Option<usize>) {
        self.started = false;
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
        self.colour = term::colour(colour);
    }

    pub fn set_charset(&mut self, charset: &[&str]) {
        self.charset = charset.join("");
        self.charset_len = charset.len();
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
            self.charset = "#".to_string();
            self.fill = ".".to_string();
        } else if matches!(self.animation, Animation::Arrow) {
            self.charset = "=".to_string();
        }
    }
}

// pub trait Progress1
// where
//     Self: Sized + Iterator,
// {
//     fn pbfy(self) -> BarIter<Self>
//     where
//         Self: Iterator,
//     {
//         let total = self.size_hint().0;
//         BarIter {
//             iterable: self,
//             pb: Bar {
//                 total: total,
//                 ..Default::default()
//             },
//             rendered_once: false,
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use crate::std_bar::Progress1;

//     #[test]
//     fn sas() {
//         let a = vec![1,2,3];

//         Progress1::pbfy(a.iter());
//         for o in a.iter().pbfy() {
//         }
//         unimplemented!();
//     }
// }
