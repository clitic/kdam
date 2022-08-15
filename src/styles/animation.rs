use crate::term::Colorizer;

/// Bar animation styles for [Bar](crate::Bar).
#[derive(Debug, Clone)]
pub enum Animation {
    Arrow,
    Classic,
    Custom(Vec<String>),
    CustomWithFill(Vec<String>, String),
    FillUp,
    FiraCode,
    Tqdm,
    TqdmAscii,
}

impl From<&str> for Animation {
    fn from(animation: &str) -> Self {
        match animation.to_lowercase().as_str() {
            "arrow" => Self::Arrow,
            "classic" => Self::Classic,
            "fillup" => Self::FillUp,
            "firacode" => Self::FiraCode,
            "ascii" => Self::TqdmAscii,
            _ => Self::Tqdm,
        }
    }
}

impl Animation {
    /// Construct [Animation::Custom](crate::Animation) enum variant.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kdam::Animation;
    ///
    /// let anim = Animation::custom(&["\\", "|", "/", "-"]);
    /// ```
    pub fn custom(charset: &[&str]) -> Self {
        Self::Custom(
            charset
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        )
    }

    /// Construct [Animation::CustomWithFill](crate::Animation) enum variant.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kdam::Animation;
    ///
    /// let anim = Animation::custom_with_fill(&["\\", "|", "/", "-"], ".");
    /// ```
    pub fn custom_with_fill(charset: &[&str], fill: &str) -> Self {
        Self::CustomWithFill(
            charset
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            fill.to_owned(),
        )
    }

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
                let mut fill = None;

                let charset = match self {
                    Self::TqdmAscii => vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "#"],
                    Self::FillUp => vec![
                        "\u{2581}", "\u{2582}", "\u{2583}", "\u{2584}", "\u{2585}", "\u{2586}",
                        "\u{2587}", "\u{2588}",
                    ],
                    Self::Custom(custom_charset) => {
                        custom_charset.iter().map(|x| x.as_str()).collect::<_>()
                    }
                    Self::CustomWithFill(custom_charset, filling) => {
                        fill = Some(filling.to_owned());
                        custom_charset.iter().map(|x| x.as_str()).collect::<_>()
                    }
                    _ => vec![
                        "\u{258F}", "\u{258E}", "\u{258D}", "\u{258C}", "\u{258B}", "\u{258A}",
                        "\u{2589}", "\u{2588}",
                    ],
                };

                let nsyms = charset.len() - 1;
                let (bar_length, frac_bar_length) = crate::styles::format::divmod(
                    (progress * ncols as f32 * nsyms as f32) as usize,
                    nsyms,
                );
                let mut bar_animation = charset.last().unwrap().repeat(bar_length);

                if bar_length < ncols as usize {
                    bar_animation += charset[frac_bar_length + 1];

                    if let Some(filling) = fill {
                        bar_animation += &filling
                            .to_string()
                            .repeat((ncols - (bar_length as i16) - 1) as usize);
                    } else {
                        bar_animation += &" ".repeat((ncols - (bar_length as i16) - 1) as usize);
                    }
                }

                bar_animation
            }
        }
    }

    /// Formatted version of `self.progress` with opening and closing brackets.
    pub fn fmt_progress(&self, progress: f32, ncols: i16, colour: &str) -> String {
        let (bar_open, bar_close) = match self {
            Self::Arrow | Self::Classic => ("[", "]"),
            Self::Custom(_)
            | Self::CustomWithFill(_, _)
            | Self::FillUp
            | Self::Tqdm
            | Self::TqdmAscii => ("|", "|"),
            Self::FiraCode => (" ", ""),
        };

        if colour.to_uppercase().starts_with("GRADIENT(") {
            if !cfg!(feature = "gradient") {
                panic!("Enable cargo feature `gradient` to use gradient colours.");
            }

            #[cfg(feature = "gradient")]
            return format!(
                "{}{}{}",
                bar_open,
                self.progress(progress, ncols).gradient(
                    &colour
                        .to_uppercase()
                        .trim_start_matches("GRADIENT(")
                        .trim_end_matches(')')
                        .split(",")
                        .collect::<Vec<&str>>(),
                    ncols as usize,
                ),
                bar_close
            );
        }

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

    /// Returns extra spaces consumed by `self.fmt_progress`.
    pub fn spaces(&self) -> u8 {
        match self {
            Self::FiraCode => 3,
            _ => 2,
        }
    }
}
