use crate::term::Colorizer;

// Spinner Unicodes
// pub(crate) static CLASSICSPINNER: [&str; 4] = ["\\", "|", "/", "-"];
// pub(crate) static FIRACODESPINNER: [&str; 6] = [
//     "\u{EE06}", "\u{EE07}", "\u{EE08}", "\u{EE09}", "\u{EE0A}", "\u{EE0B}",
// ];

/// Pre configured animation styles for `kdam::Bar`.
#[derive(Debug, Clone)]
pub enum Animation {
    Arrow,
    Classic,
    Custom(&'static [&'static str]),
    FillUp,
    FiraCode,
    Tqdm,
    TqdmAscii,
}

impl Animation {
    /// Generate progress bar animation.
    ///
    /// # Arguments
    ///
    /// - progress: It can be from range (0.0 - 1.0) inclusive.
    /// - ncols: number of columns to render.
    pub fn progress(&self, progress: f32, ncols: i16) -> String {
        match self {
            Self::Arrow => {
                let block = (ncols as f32 * progress) as i16;
                format!(
                    "{}{}",
                    "=".repeat(block as usize),
                    if progress >= 1.0 {
                        "".to_owned()
                    } else {
                        ">".to_owned() + &" ".repeat((ncols - block - 1) as usize)
                    }
                )
            }

            Self::Classic => {
                let block = (ncols as f32 * progress) as i16;
                format!(
                    "{}{}",
                    "#".repeat(block as usize),
                    if progress >= 1.0 {
                        "".to_owned()
                    } else {
                        "#".to_owned() + &".".repeat((ncols - block - 1) as usize)
                    }
                )
            }

            Self::FiraCode => {
                let block = (ncols as f32 * progress) as i16;
                format!(
                    "\u{EE03}{}{}{}",
                    "\u{EE04}".repeat(block as usize),
                    "\u{EE01}".repeat((ncols - block) as usize),
                    if progress >= 1.0 {
                        "\u{EE05}"
                    } else {
                        "\u{EE02}"
                    }
                )
            }

            _ => {
                let charset: &[&str] = match self {
                    Self::TqdmAscii => &["1", "2", "3", "4", "5", "6", "7", "8", "9", "#"],
                    Self::FillUp => &[
                        "\u{2581}", "\u{2582}", "\u{2583}", "\u{2584}", "\u{2585}", "\u{2586}",
                        "\u{2587}", "\u{2588}",
                    ],
                    Self::Custom(custom_charset) => custom_charset,
                    _ => &[
                        "\u{258F}", "\u{258E}", "\u{258D}", "\u{258C}", "\u{258B}", "\u{258A}",
                        "\u{2589}", "\u{2588}",
                    ],
                };

                let nsyms = charset.len() - 1;
                let (bar_length, frac_bar_length) =
                    crate::format::divmod((progress * ncols as f32 * nsyms as f32) as usize, nsyms);
                let mut bar_animation = charset.last().unwrap().repeat(bar_length);

                if bar_length < ncols as usize {
                    bar_animation += charset[frac_bar_length + 1];
                    bar_animation += &" ".repeat((ncols - (bar_length as i16) - 1) as usize);
                }

                bar_animation
            }
        }
    }

    /// Formatted version of `self.progress` with opening and closing brackets.
    pub fn progress_fmt(&self, progress: f32, ncols: i16, colour: &str) -> String {
        let (bar_open, bar_close) = match self {
            Self::Arrow | Self::Classic => ("[", "]"),
            Self::Custom(_) | Self::FillUp | Self::Tqdm | Self::TqdmAscii => ("|", "|"),
            Self::FiraCode => (" ", ""),
        };

        if colour == "default" {
            format!(
                "{}{}{}",
                bar_open,
                self.progress(progress, ncols),
                bar_close
            )
        } else {
            format!(
                "{}{}{}",
                bar_open,
                self.progress(progress, ncols).colorize(colour),
                bar_close
            )
        }
    }
}

pub(crate) fn rich_bar(progress: f32, ncols: i16) -> String {
    if progress >= 1.0 {
        "━".repeat(ncols as usize).colorize("#729c1f")
    } else {
        let block = (ncols as f32 * progress) as i16;
        ("━".repeat(block as usize) + "╸").colorize("#F92672")
            + &"━".repeat((ncols - block - 1) as usize).colorize("#525252")
    }
}

pub(crate) fn rich_pulse(ncols: i16, current_time: f32) -> String {
    let pulse = [
        "#3a3a3a", "#3e393b", "#4c383f", "#613545", "#7b334d", "#b72c5e", "#d12a66", "#e6276c",
        "#f42670", "#f92672", "#f42670", "#e6276c", "#d12a66", "#b72c5e", "#993056", "#7b334d",
        "#613545", "#4c383f",
    ]
    .repeat((ncols as f32 / 18 as f32) as usize + 2);

    let pulse_len = pulse.len();
    let offset = (-current_time * 15 as f32) as i16 % 18;
    let mut pulse_string = String::new();

    for i in offset..(offset + ncols) {
        if 0 > i {
            pulse_string += &"━".colorize(pulse[pulse_len - (-i as usize)]);
        } else {
            pulse_string += &"━".colorize(pulse[i as usize]);
        }
    }

    pulse_string
}
