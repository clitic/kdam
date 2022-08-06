use crate::prelude::*;
use crate::Bar;

/// Renderable columns for [RichProgress](crate::RichProgress).
///
/// These columns may differ in name as of `rich.progress`.
#[derive(Debug, Clone)]
pub enum Column {
    /// Progress bar column.
    /// If self.pb.n || self.pb.total == 0, then an pulsating animation
    /// else rich style animation.
    Bar,
    /// Progress counter i.e. `sel.pb.n`.
    Count,
    /// Formatted counter i.e. `self.pb.n / self.pb.total`
    CountTotal,
    /// Progress elapsed time
    ElapsedTime,
    /// Progress percentage done, with precision.
    Percentage(usize),
    /// Progress update rate.
    Rate,
    /// Progress remaining time / ETA.
    RemainingTime,
    /// Spinner for progress. See more styles at [rich repository](https://github.com/Textualize/rich/blob/master/rich/_spinners.py).
    /// - first argument is Vec<String> of frames.
    /// - second argument is interval of frames.
    /// - third argument is speed of frames.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kdam::Column;
    ///
    /// Column::Spinner(
    ///     "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"
    ///     .chars()
    ///     .map(|x| x.to_string())
    ///     .collect::<Vec<String>>(),
    ///     80.0,
    ///     1.0
    /// );
    /// ````
    Spinner(Vec<String>, f32, f32),
    /// Text column.
    ///
    /// # Example`
    ///
    /// ```rust
    /// use kdam::Column;
    ///
    /// Column::Text("•".to_owned());
    /// Column::Text("[bold red]Downloading".to_owned());
    /// ```
    Text(String),
    /// Progress total i.e. `self.pb.total`.
    Total,
}

impl Column {
    /// Text column.
    ///
    /// # Example`
    ///
    /// ```rust
    /// use kdam::Column;
    ///
    /// Column::text("•");
    /// Column::Text("•".to_owned());
    ///
    /// Column::text("[bold red]Downloading");
    /// Column::Text("[bold red]Downloading".to_owned());
    /// ```
    pub fn text(text: &str) -> Self {
        Self::Text(text.to_owned())
    }
}

/// An implementation [rich.progress](https://rich.readthedocs.io/en/latest/progress.html) using [Bar](crate::Bar).
///
/// # Example
///
/// ```rust
/// use kdam::prelude::*;
/// use kdam::{Column, RichProgress};
///
/// fn main() {
///     let mut pb = RichProgress::new(
///         tqdm!(total = 100),
///         vec![Column::Bar, Column::Percentage(2)]
///     );
///
///     for _ in 0..100 {
///         pb.update(1);
///     }
///
///     eprint!("\n");
/// }
/// ```
#[derive(Debug)]
pub struct RichProgress {
    /// Instance of [Bar](crate::Bar) to render [RichProgress](crate::RichProgress).
    pub pb: Bar,
    /// Vector of renderable columns.
    pub columns: Vec<Column>,
}

impl RichProgress {
    /// Create a new instance of [RichProgress](crate::RichProgress).
    pub fn new(pb: Bar, columns: Vec<Column>) -> Self {
        Self { pb, columns }
    }

    /// Replace a column value at specific index.
    pub fn replace(&mut self, index: usize, col: Column) {
        let _ = std::mem::replace(&mut self.columns[index], col);
    }
}

impl BarMethods for RichProgress {
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
        let mut bar_text = vec![];
        let mut bar_length = 0;
        let mut progress_bar_index = None;
        self.pb.bar_elapsed_time();

        for col in self.columns.clone() {
            match col {
                Column::Bar => {
                    progress_bar_index = Some(bar_text.len());
                    bar_text.push("".to_owned());
                }

                Column::Count => {
                    let fmt_progress = self.pb.bar_fmt_count();
                    bar_length += fmt_progress.chars().count();
                    bar_text.push(fmt_progress.colorize("green"));
                }

                Column::CountTotal => {
                    let fmt_progress =
                        format!("{}/{}", self.pb.bar_fmt_count(), self.pb.bar_fmt_total());
                    bar_length += fmt_progress.chars().count();
                    bar_text.push(fmt_progress.colorize("green"));
                }

                Column::ElapsedTime => {
                    let elapsed_time = self.pb.bar_fmt_elapsed_time();
                    bar_length += elapsed_time.chars().count();
                    bar_text.push(elapsed_time.colorize("cyan"));
                }

                Column::Percentage(precision) => {
                    let percentage = format!("{:.1$}%", self.pb.bar_percentage() * 100., precision);
                    bar_length += percentage.chars().count();
                    bar_text.push(percentage.colorize("magenta"));
                }

                Column::Rate => {
                    let speed = self.pb.bar_fmt_rate();
                    bar_length += speed.chars().count();
                    bar_text.push(speed.colorize("red"));
                }

                Column::RemainingTime => {
                    let remaining_time = self.pb.bar_fmt_remaining_time();
                    bar_length += remaining_time.chars().count();
                    bar_text.push(remaining_time.colorize("cyan"));
                }

                Column::Spinner(frames, interval, speed) => {
                    let frame_no = (self.pb.bar_elapsed_time() * speed) / (interval / 1000.0);
                    let frame = frames.get(frame_no as usize % frames.len()).unwrap();
                    bar_length += frame.chars().count();
                    bar_text.push(frame.colorize("green"));
                }

                Column::Text(text) => {
                    let color = match (text.find("["), text.find("]")) {
                        (Some(start), Some(end)) => {
                            if start == 0 {
                                Some(&text[(start + 1)..(end)])
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };

                    if let Some(code) = color {
                        let text = text.replace(&format!("[{}]", code), "");
                        bar_length += text.chars().count();
                        bar_text.push(text.colorize(code));
                    } else {
                        bar_length += text.chars().count();
                        bar_text.push(text);
                    }
                }

                Column::Total => {
                    let fmt_progress = self.pb.bar_fmt_total();
                    bar_length += fmt_progress.chars().count();
                    bar_text.push(fmt_progress.colorize("green"));
                }
            }
        }

        bar_length += bar_text.len() - 1;

        if progress_bar_index.is_some() {
            self.pb.set_ncols(bar_length as i16);
            let pb;

            if self.pb.total == 0 || self.pb.counter() == 0 {
                pb = crate::styles::rich_pulse(self.pb.ncols.clone(), self.pb.elapsed_time);
            } else {
                pb =
                    crate::styles::rich_bar(self.pb.bar_percentage() as f32, self.pb.ncols.clone());
            }

            let _ = std::mem::replace(&mut bar_text[progress_bar_index.unwrap()], pb);
        }

        self.pb.bar_length = bar_length as i16 + self.pb.ncols;
        bar_text.join(" ")
    }

    fn reset(&mut self, total: Option<usize>) {
        self.pb.reset(total);
    }

    fn update(&mut self, n: usize) {
        self.pb.init();

        if self.pb.trigger(n) {
            let text = self.render();
            let length = crate::term::string_display_length(text.clone()) as i16;

            if length != self.pb.bar_length {
                self.pb.clear();
            }

            self.pb.bar_length = length;
            self.pb.write_at(text);
        }
    }

    fn update_to(&mut self, update_to_n: usize) {
        self.pb.n = update_to_n;
        self.update(0);
    }

    fn write<T: Into<String>>(&mut self, text: T) {
        self.pb.clear();
        self.pb.writer.print(format_args!("\r{}\n", text.into()));

        if self.pb.leave {
            self.refresh();
        }
    }
}
