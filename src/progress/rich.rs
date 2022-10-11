use crate::progress::Bar;
use crate::term::Colorizer;

/// Renderable columns for [RichProgress](crate::RichProgress).
///
/// These columns may differ in name as of `rich.progress`.
#[derive(Debug, Clone)]
pub enum Column {
    /// Progress bar column.
    /// If progress.pb.n || progress.pb.total == 0, then an pulsating animation
    /// else rich style animation.
    Bar,
    /// Progress counter i.e. `sel.pb.n`.
    Count,
    /// Formatted counter i.e. `progress.pb.n / progress.pb.total`
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
    /// Progress total i.e. `progress.pb.total`.
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
/// let mut pb = RichProgress::new(
///     tqdm!(total = 100),
///     vec![Column::Bar, Column::Percentage(2)]
/// );
///
/// for _ in 0..100 {
///     pb.update(1);
/// }
///
/// eprint!("\n");
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
        *self.columns.get_mut(index).unwrap() = col;
        // let _ = std::mem::replace(&mut self.columns[index], col);
    }
}

crate::_impl_bar_methods!(RichProgress, render);

fn render(progress: &mut RichProgress) -> String {
    let mut bar_text = vec![];
    let mut bar_length = 0;
    let mut progress_bar_index = None;
    let et = progress.pb.elapsed_time();

    for col in progress.columns.clone() {
        match col {
            Column::Bar => {
                progress_bar_index = Some(bar_text.len());
                bar_text.push(String::new());
            }

            Column::Count => {
                let fmt_progress = progress.pb.fmt_counter();
                bar_length += fmt_progress.chars().count();
                bar_text.push(fmt_progress.colorize("green"));
            }

            Column::CountTotal => {
                let fmt_progress =
                    format!("{}/{}", progress.pb.fmt_counter(), progress.pb.fmt_total());
                bar_length += fmt_progress.chars().count();
                bar_text.push(fmt_progress.colorize("green"));
            }

            Column::ElapsedTime => {
                let elapsed_time = progress.pb.fmt_elapsed_time();
                bar_length += elapsed_time.chars().count();
                bar_text.push(elapsed_time.colorize("cyan"));
            }

            Column::Percentage(precision) => {
                let percentage = format!("{:.1$}%", progress.pb.percentage() * 100., precision);
                bar_length += percentage.chars().count();
                bar_text.push(percentage.colorize("magenta"));
            }

            Column::Rate => {
                let speed = progress.pb.fmt_rate();
                bar_length += speed.chars().count();
                bar_text.push(speed.colorize("red"));
            }

            Column::RemainingTime => {
                let remaining_time = progress.pb.fmt_remaining_time();
                bar_length += remaining_time.chars().count();
                bar_text.push(remaining_time.colorize("cyan"));
            }

            Column::Spinner(frames, interval, speed) => {
                let frame_no = (progress.pb.elapsed_time() * speed) / (interval / 1000.0);
                let frame = frames.get(frame_no as usize % frames.len()).unwrap();
                bar_length += frame.chars().count();
                bar_text.push(frame.colorize("green"));
            }

            Column::Text(text) => {
                let color = match (text.find('['), text.find(']')) {
                    (Some(start), Some(end)) => {
                        if start == 0 {
                            text.get(1..end)
                        } else {
                            None
                        }
                    }
                    _ => None,
                };

                if let Some(code) = color {
                    let text = text.replace(&format!("[{}]", code), "");
                    bar_length += text.len_ansi();
                    bar_text.push(text.colorize(code));
                } else {
                    bar_length += text.len_ansi();
                    bar_text.push(text);
                }
            }

            Column::Total => {
                let fmt_progress = progress.pb.fmt_total();
                bar_length += fmt_progress.chars().count();
                bar_text.push(fmt_progress.colorize("green"));
            }
        }
    }

    bar_length += bar_text.len() - 1;
    let mut ncols = 0;

    if let Some(progress_bar_index) = progress_bar_index {
        progress.pb.adjust_ncols(bar_length as i16);
        ncols = progress.pb.get_ncols();

        if ncols == 0 {
            let _ = bar_text.remove(progress_bar_index);
        } else {
            *bar_text.get_mut(progress_bar_index).unwrap() =
                if progress.pb.indefinite() || !progress.pb.started() {
                    crate::styles::rich::pulse(ncols, et)
                } else {
                    crate::styles::rich::bar(progress.pb.percentage() as f32, ncols)
                };
        }
    }

    progress
        .pb
        .set_bar_length(bar_length as i16 + ncols);
    bar_text.join(" ")
}
