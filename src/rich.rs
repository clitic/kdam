use crate::std_bar::Bar;
use crate::term::Colorizer;
use crate::{term, term::Output};

/// Renderable columns for `kdam::RichProgress`.
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
    /// Column::Spinner(
    ///     "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"
    ///     .chars()
    ///     .map(|x| x.to_string())
    ///     .collect::<Vec<String>>(),
    ///     80.0,
    ///     1.0
    /// )
    /// ````
    Spinner(Vec<String>, f32, f32),
    /// Text column.
    /// - first argument is text to render.
    /// - second argument is colour style.
    ///
    /// # Example`
    ///
    /// ```rust
    /// Column::Text("•".to_string(), None);
    /// Column::Text("caught an error".to_string(), Some("bold red"));
    /// ```
    Text(String, Option<String>),
    /// Progress total i.e. `self.pb.total`.
    Total,
}

/// An implementation [rich.progress](https://rich.readthedocs.io/en/latest/progress.html) using `kdam::Bar`.
///
/// # Example
///
/// ```rust
/// use kdam::{tqdm, Column, RichProgress};
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
    /// Instance of `kdam::Bar` to render `kdam::RichProgress`.
    pub pb: Bar,
    /// Vector of renderable columns.
    pub columns: Vec<Column>,
}

impl RichProgress {
    /// Create a new instance of `kdam::RichProgress`.
    pub fn new(pb: Bar, columns: Vec<Column>) -> Self {
        Self { pb, columns }
    }

    /// Replace a column value at specific index.
    pub fn replace(&mut self, index: usize, col: Column) {
        let _ = std::mem::replace(&mut self.columns[index], col);
    }

    fn render(&mut self) -> String {
        let mut bar_text = vec![];
        let mut bar_length = 0;
        let mut progress_bar_index = None;
        self.pb.elapsed_time();

        for col in self.columns.clone() {
            match col {
                Column::Bar => {
                    progress_bar_index = Some(bar_text.len());
                    bar_text.push("".to_string());
                }

                Column::Count => {
                    let fmt_progress = self.pb.count_fmt();
                    bar_length += fmt_progress.chars().count();
                    bar_text.push(fmt_progress.colorize("green"));
                }

                Column::CountTotal => {
                    let fmt_progress = format!("{}/{}", self.pb.count_fmt(), self.pb.total_fmt());
                    bar_length += fmt_progress.chars().count();
                    bar_text.push(fmt_progress.colorize("green"));
                }

                Column::ElapsedTime => {
                    let elapsed_time = self.pb.elapsed_time_fmt();
                    bar_length += elapsed_time.chars().count();
                    bar_text.push(elapsed_time.colorize("cyan"));
                }

                Column::Percentage(precision) => {
                    let percentage = format!("{:.1$}%", self.pb.percentage() * 100., precision);
                    bar_length += percentage.chars().count();
                    bar_text.push(percentage.colorize("magenta"));
                }

                Column::Rate => {
                    let speed = self.pb.rate_fmt();
                    bar_length += speed.chars().count();
                    bar_text.push(speed.colorize("red"));
                }

                Column::RemainingTime => {
                    let remaining_time = self.pb.eta_fmt();
                    bar_length += remaining_time.chars().count();
                    bar_text.push(remaining_time.colorize("cyan"));
                }

                Column::Spinner(frames, interval, speed) => {
                    let frame_no = (self.pb.elapsed_time() * speed) / (interval / 1000.0);
                    let frame = frames.get(frame_no as usize % frames.len()).unwrap();
                    bar_length += frame.chars().count();
                    bar_text.push(frame.colorize("green"));
                }

                Column::Text(text, colour) => {
                    bar_length += text.chars().count();

                    if let Some(code) = colour {
                        bar_text.push(text.colorize(code.as_str()));
                    } else {
                        bar_text.push(text);
                    }
                }

                Column::Total => {
                    let fmt_progress = self.pb.total_fmt();
                    bar_length += fmt_progress.chars().count();
                    bar_text.push(fmt_progress.colorize("green"));
                }
            }
        }

        bar_length += bar_text.len() - 1;

        if progress_bar_index.is_some() {
            self.pb.set_ncols(bar_length as i16);
            let pb;

            if self.pb.total == 0 || self.pb.n == 0 {
                pb = crate::styles::rich_pulse(self.pb.ncols.clone(), self.pb.elapsed_time.clone());
            } else {
                pb = crate::styles::rich_bar(self.pb.percentage() as f32, self.pb.ncols.clone());
            }

            let _ = std::mem::replace(&mut bar_text[progress_bar_index.unwrap()], pb);
        }

        self.pb.bar_length = bar_length as i16 + self.pb.ncols;
        bar_text.join(" ")
    }

    /// Manually update the progress bar, useful for streams such as reading files.
    pub fn update(&mut self, n: usize) {
        if self.pb.trigger(n) {
            let text = self.render();
            self.pb.write_at(text);
        }
    }

    /// Set position of the progress bar.
    /// Alternative way to update bar.
    pub fn set_position(&mut self, position: usize) {
        self.pb.n = position;
        self.update(0);
    }

    /// Force refresh the display of this bar.
    pub fn refresh(&mut self) {
        if !self.pb.max_fps {
            self.pb.force_refresh = true;
            self.update(0);
            self.pb.force_refresh = false;
        } else {
            self.update(0);
        }
    }

    /// Resets to intial iterations for repeated use.
    /// Consider combining with `leave=true`.
    pub fn reset(&mut self, total: Option<usize>) {
        self.pb.reset(total);
    }

    /// Clear current bar display.
    pub fn clear(&mut self) {
        self.pb.clear();
    }

    /// Print a message via bar (without overlap with bars).
    pub fn write(&mut self, text: String) {
        self.pb.clear();

        match self.pb.output {
            Output::Stderr => term::write_to_stderr(format_args!("{}\n", text)),
            Output::Stdout => term::write_to_stdout(format_args!("{}\n", text)),
        }

        if self.pb.leave {
            self.refresh();
        }
    }

    /// Take input via bar (without overlap with bars).
    pub fn input(&mut self, text: &str) -> Result<String, std::io::Error> {
        self.pb.clear();

        match self.pb.output {
            Output::Stderr => term::write_to_stderr(format_args!("{}", text)),
            Output::Stdout => term::write_to_stdout(format_args!("{}", text)),
        }

        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string)?;

        if self.pb.leave {
            self.pb.refresh();
        }

        Ok(input_string)
    }

    /// Print a string in position of bar.
    pub fn write_at(&self, text: String) {
        self.pb.write_at(text);
    }
}
