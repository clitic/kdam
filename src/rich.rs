use crate::std_bar::Bar;
use crate::term::Colorizer;
use crate::{term, term::Output};

#[derive(Debug, Clone)]
pub enum Column {
    /// progress bar
    Bar,
    /// self.n
    Count,
    /// self.n / self.total
    CountTotal,
    ElapsedTime,
    Percentage(usize),
    Rate,
    RemainingTime,
    Spinner(Vec<String>, f32, f32),
    Text(String, Option<String>),
    Total,
}

#[derive(Debug)]
pub struct RichProgress {
    pub pb: Bar,
    columns: Vec<Column>,
}

impl RichProgress {
    pub fn new(pb: Bar) -> Self {
        Self {
            pb,
            columns: vec![],
        }
    }

    pub fn add(&mut self, col: Column) {
        self.columns.push(col);
    }

    pub fn render(&mut self) -> String {
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
