use super::styles;
use crate::{std::Bar, term::Colorizer, BarExt};

#[cfg(feature = "spinner")]
use crate::spinner::Spinner;

/// Renderable columns for [RichProgress](RichProgress).
#[derive(Debug, Clone)]
pub enum Column {
    /// Progress bar (animation) display.
    /// If progress total is unknown then an pulsating animation is shown else a normal animation is shown.
    Bar,
    /// Progress counter display.
    Count,
    /// Progress formatted counter display i.e. `counter/total`.
    CountTotal,
    /// Progress elapsed time display.
    ElapsedTime,
    /// Progress percentage done (with precision) display.
    Percentage(usize),
    /// Progress update rate display.
    Rate,
    /// Progress remaining time (ETA) display.
    RemainingTime,
    /// Custom spinners display.
    #[cfg(feature = "spinner")]
    #[cfg_attr(docsrs, doc(cfg(feature = "spinner")))]
    Spinner(Spinner),
    /// Custom text display.
    ///
    /// # Example
    ///
    /// ```
    /// use kdam::Column;
    ///
    /// Column::Text("•".to_owned());
    /// Column::Text("[bold red]Downloading".to_owned());
    /// ```
    Text(String),
    /// Progress total display.
    Total,
}

/// An implementation [rich.progress](https://rich.readthedocs.io/en/latest/progress.html) using [Bar](crate::Bar).
///
/// # Example
///
/// ```
/// use kdam::{tqdm, BarExt};
/// use kdam::{Column, RichProgress};
///
/// let mut pb = RichProgress::new(
///     tqdm!(total = 100),
///     vec![Column::Bar, Column::Percentage(2)]
/// );
///
/// for _ in 0..100 {
///     pb.update(1);
/// }
///
/// eprintln!();
/// ```
#[derive(BarExt, Debug)]
pub struct RichProgress {
    #[bar]
    pub pb: Bar,
    pub columns: Vec<Column>,
}

impl RichProgress {
    /// Create a new [RichProgress](Self).
    pub fn new(pb: Bar, columns: Vec<Column>) -> Self {
        Self { pb, columns }
    }

    /// Replace a column at specific index.
    ///
    /// **PANICS**: If index is out of range.
    pub fn replace(&mut self, index: usize, col: Column) {
        *self.columns.get_mut(index).unwrap() = col;
        // let _ = std::mem::replace(&mut self.columns[index], col);
    }

    /// Render progress bar.
    pub fn render(&mut self) -> String {
        let mut bar_text = vec![];
        let mut bar_length = 0;
        let mut progress_bar_index = None;

        for col in self.columns.iter() {
            match col {
                Column::Bar => {
                    progress_bar_index = Some(bar_text.len());
                    bar_text.push(String::new());
                }

                Column::Count => {
                    let fmt_progress = self.pb.fmt_counter();
                    bar_length += fmt_progress.len();
                    bar_text.push(fmt_progress.colorize("green"));
                }

                Column::CountTotal => {
                    let fmt_progress = format!("{}/{}", self.pb.fmt_counter(), self.pb.fmt_total());
                    bar_length += fmt_progress.len();
                    bar_text.push(fmt_progress.colorize("green"));
                }

                Column::ElapsedTime => {
                    let elapsed_time = self.pb.fmt_elapsed_time();
                    bar_length += elapsed_time.len();
                    bar_text.push(elapsed_time.colorize("cyan"));
                }

                Column::Percentage(precision) => {
                    let percentage = format!("{:.1$}%", self.pb.percentage() * 100., precision);
                    bar_length += percentage.len();
                    bar_text.push(percentage.colorize("magenta"));
                }

                Column::Rate => {
                    let rate = self.pb.fmt_rate();
                    bar_length += rate.len();
                    bar_text.push(rate.colorize("red"));
                }

                Column::RemainingTime => {
                    let remaining_time = self.pb.fmt_remaining_time();
                    bar_length += remaining_time.len();
                    bar_text.push(remaining_time.colorize("cyan"));
                }

                #[cfg(feature = "spinner")]
                Column::Spinner(spinner) => {
                    let frame = spinner.render_frame(self.pb.elapsed_time());
                    bar_length += frame.chars().count();
                    bar_text.push(frame.colorize("green"));
                }

                Column::Text(text) => {
                    let (color, text_start_index) = match (text.find('['), text.find(']')) {
                        (Some(start), Some(end)) => {
                            if start == 0 {
                                (text.get(1..end), end + 1)
                            } else {
                                (None, 0)
                            }
                        }
                        _ => (None, 0),
                    };

                    if let Some(code) = color {
                        let text = &text[text_start_index..];
                        bar_length += text.len_ansi();
                        bar_text.push(text.colorize(code));
                    } else {
                        bar_length += text.len_ansi();
                        bar_text.push(text.to_owned());
                    }
                }

                Column::Total => {
                    let fmt_progress = self.pb.fmt_total();
                    bar_length += fmt_progress.len();
                    bar_text.push(fmt_progress.colorize("green"));
                }
            }
        }

        bar_length += bar_text.len() - 1;
        let mut ncols = 0;

        if let Some(progress_bar_index) = progress_bar_index {
            self.pb.adjust_ncols(bar_length as i16);
            ncols = self.pb.ncols;

            if ncols == 0 {
                let _ = bar_text.remove(progress_bar_index);
            } else {
                *bar_text.get_mut(progress_bar_index).unwrap() =
                    if self.pb.indefinite() || !self.pb.started() {
                        styles::pulse(ncols, self.pb.elapsed_time())
                    } else {
                        styles::bar(ncols, self.pb.percentage() as f32)
                    };
            }
        }

        self.pb.set_bar_length(bar_length as i16 + ncols);
        bar_text.join(" ")
    }
}
