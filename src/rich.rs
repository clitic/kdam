use crate::std_bar::Bar;
use crate::term::Colorizer;

#[derive(Debug, Clone)]
pub enum Column {
    Bar,
    Download,
    FileSize,
    Spinner(Vec<String>, f32, f32),
    TaskProgress(usize),
    Text(String, Option<String>),
    TimeElapsed,
    TimeRemaining,
    TotalFileSize,
    TransferSpeed,
}

#[derive(Debug)]
pub struct RichProgress {
    pb: Bar,
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

    pub fn render(&mut self) -> (String, usize) {
        let mut bar_text = vec![];
        let mut bar_length = 0;
        let mut progress_bar_index = None;

        for col in self.columns.clone() {
            match col {
                Column::Bar => {
                    progress_bar_index = Some(bar_text.len());
                    bar_text.push("".to_string());
                }

                Column::Download => {
                    let fmt_progress = self.pb.progress_fmt();

                    bar_length += fmt_progress.chars().count();
                    bar_text.push(fmt_progress.colorize("green"));
                }

                Column::FileSize => {}

                Column::Spinner(frames, interval, speed) => {
                    let frame_no = (self.pb.elapsed_time() * speed) / (interval / 1000.0);
                    let frame = frames.get(frame_no as usize % frames.len()).unwrap();

                    bar_length += frame.chars().count();
                    bar_text.push(frame.colorize("green"));
                }

                Column::TaskProgress(precision) => {
                    let percentage = format!("{:.1$}%", self.pb.percentage() * 100., precision);

                    bar_length += percentage.chars().count();
                    bar_text.push(percentage.colorize("magenta"));
                }

                Column::Text(text, colour) => {
                    bar_length += text.chars().count();

                    if let Some(code) = colour {
                        bar_text.push(text.colorize(code.as_str()));
                    } else {
                        bar_text.push(text);
                    }
                }

                Column::TimeElapsed => {
                    let elapsed_time =
                        crate::format::format_interval(self.pb.elapsed_time() as usize);

                    bar_length += elapsed_time.chars().count();
                    bar_text.push(elapsed_time.colorize("cyan"));
                }

                Column::TimeRemaining => {
                    self.pb.elapsed_time();
                    let remaining_time =
                        crate::format::format_interval(self.pb.eta() as usize);

                    bar_length += remaining_time.chars().count();
                    bar_text.push(remaining_time.colorize("cyan"));
                }

                Column::TotalFileSize => {}

                Column::TransferSpeed => {
                    let speed = self.pb.rate_fmt();

                    bar_length += speed.chars().count();
                    bar_text.push(speed.colorize("red"));
                }
            }
        }

        bar_length += bar_text.len() - 1;

        if progress_bar_index.is_some() {
            self.pb.set_ncols(bar_length as i16);
            let pb;

            if self.pb.total == 0 || self.pb.n == 0 {
                pb = crate::styles::rich_pulse(self.pb.ncols.clone(), self.pb.elapsed_time());
            } else {
                pb = crate::styles::rich_bar(self.pb.percentage() as f32, self.pb.ncols.clone());
            }

            let _ = std::mem::replace(&mut bar_text[progress_bar_index.unwrap()], pb);
        }

        (bar_text.join(" "), bar_length)
    }

    pub fn update(&mut self, n: usize) {
        if self.pb.trigger(n) {
            let (text, length) = self.render();
            self.pb.bar_length = length as i16 + self.pb.ncols;

            if !self.pb.wrap {
                self.pb.write_at(text);
            } else {
                let columns = crate::term::get_columns() as usize;
                if self.pb.bar_length as usize > columns {
                    self.pb.write_at(text[..columns].to_string());
                } else {
                    self.pb.write_at(text);
                }
            }
        }
    }
}
