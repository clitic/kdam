use std::num::NonZeroU16;
use crate::{term::Colorizer, utils::divmod};

#[cfg(feature = "gradient")]
use crate::utils;

#[cfg(feature = "gradient")]
use colorgrad::{CustomGradient, Gradient};

#[cfg(feature = "unicode")]
use unicode_segmentation::UnicodeSegmentation;

const BAR_FILLUP: [&str; 8] = [
    "\u{2581}", "\u{2582}", "\u{2583}", "\u{2584}", "\u{2585}", "\u{2586}", "\u{2587}", "\u{2588}",
];
const BAR_TQDM: [&str; 8] = [
    "\u{258F}", "\u{258E}", "\u{258D}", "\u{258C}", "\u{258B}", "\u{258A}", "\u{2589}", "\u{2588}",
];
const BAR_TQDM_ASCII: [&str; 10] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "#"];

/// Animation styles for [Bar](crate::Bar).
#[derive(Clone, Debug)]
pub enum Animation {
    Arrow,
    Classic,
    Custom(Vec<String>, Option<String>),
    FillUp,
    FiraCode,
    Tqdm,
    TqdmAscii,
}

impl Animation {
    /// Create a new [Animation::Custom](Self::Custom) enum variant.
    ///
    /// # Example
    ///
    /// ```
    /// use kdam::Animation;
    ///
    /// let custom = Animation::custom(&["\\", "|", "/", "-"], None);
    /// let custom_with_fill = Animation::custom(&["\\", "|", "/", "-"], Some("."));
    /// ```
    pub fn custom(charset: &[&str], fill: Option<&str>) -> Self {
        Self::Custom(
            charset.iter().map(|x| String::from(*x)).collect(),
            fill.map(|x| x.to_owned()),
        )
    }

    /// Render progress bar animation.
    ///
    /// # Arguments
    ///
    /// - *ncols*: Number of columns to render.
    /// - *progress*: Percentage done, it should be in range (0.0 - 1.0) inclusive.
    pub fn render(&self, ncols: NonZeroU16, progress: f32) -> String {
        assert!((0.0..=1.0).contains(&progress));

        let ncols = ncols.get();

        match self {
            Self::Arrow | Self::Classic => {
                let block = (ncols as f32 * progress) as u16;

                let (bar_completed, bar_head, bar_uncompleted) = match self {
                    Self::Arrow => ("=", ">", " "),
                    Self::Classic => ("#", "#", "."),
                    _ => unreachable!(),
                };

                bar_completed.repeat(block as usize)
                    + &if progress >= 1.0 {
                        String::new()
                    } else {
                        bar_head.to_owned() + &bar_uncompleted.repeat((ncols - block - 1) as usize)
                    }
            }

            Self::FiraCode => {
                let block = (ncols as f32 * progress) as u16;
                "\u{EE03}".to_owned()
                    + &"\u{EE04}".repeat(block as usize)
                    + &"\u{EE01}".repeat((ncols - block) as usize)
                    + if progress >= 1.0 {
                        "\u{EE05}"
                    } else {
                        "\u{EE02}"
                    }
            }

            _ => {
                let mut bar_uncompleted = None;

                let charset = match self {
                    Self::Custom(custom_charset, fill) => {
                        bar_uncompleted = fill.as_ref().map(|x| x.as_str());
                        custom_charset.iter().map(|x| x.as_str()).collect()
                    }
                    Self::FillUp => Vec::from(BAR_FILLUP),
                    Self::TqdmAscii => Vec::from(BAR_TQDM_ASCII),
                    _ => Vec::from(BAR_TQDM),
                };

                let nsyms = charset.len() - 1;
                let (bar_length, frac_bar_length) =
                    divmod((progress * ncols as f32 * nsyms as f32) as usize, nsyms);
                let mut bar_animation = charset.last().unwrap().repeat(bar_length);

                if ncols > bar_length as u16 {
                    bar_animation += charset[frac_bar_length + 1];
                    let bar_uncompleted = bar_uncompleted.unwrap_or(" ");
                    bar_animation +=
                        &bar_uncompleted.repeat((ncols - bar_length as u16 - 1) as usize);
                }

                bar_animation
            }
        }
    }

    /// Render progress bar animation with opening and closing brackets.
    pub fn fmt_render(&self, ncols: NonZeroU16, progress: f32, colour: &Option<Colour>) -> String {
        let (bar_open, bar_close) = match self {
            Self::Arrow | Self::Classic => ("[", "]"),
            Self::FiraCode => (" ", ""),
            _ => ("|", "|"),
        };

        let render = self.render(ncols, progress);

        if let Some(colour) = colour {
            bar_open.to_owned() + &colour.apply(&render) + bar_close
        } else {
            bar_open.to_owned() + &render + bar_close
        }
    }

    /// Returns extra spaces consumed by [fmt_render](Self::fmt_render).
    pub fn spaces(&self) -> u8 {
        match self {
            Self::FiraCode => 3,
            _ => 2,
        }
    }
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

/// Colour applicable to text.
#[derive(Debug)]
pub enum Colour {
    Solid(String),
    #[cfg(feature = "gradient")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gradient")))]
    Gradient(Gradient),
}

impl Colour {
    /// Create a new [Color::Gradient](Self::Gradient) enum variant with custom colors.
    #[cfg(feature = "gradient")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gradient")))]
    pub fn gradient(colors: &[&str]) -> Self {
        Self::Gradient(
            CustomGradient::new()
                .html_colors(colors)
                .build()
                .unwrap_or(colorgrad::rainbow()),
        )
    }

    /// Create a new [Color::Gradient](Self::Gradient) enum variant with rainbow colors.
    #[cfg(feature = "gradient")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gradient")))]
    pub fn rainbow() -> Self {
        Self::Gradient(colorgrad::rainbow())
    }

    /// Create a new [Color::Solid](Self::Solid) enum variant.
    pub fn solid(color: &str) -> Self {
        Self::Solid(color.to_owned())
    }

    /// Apply colour to given text.
    pub fn apply(&self, text: &str) -> String {
        match self {
            #[cfg(feature = "gradient")]
            Colour::Gradient(gradient) => {
                let mut colors = gradient
                    .colors(utils::len(text))
                    .into_iter()
                    .map(|x| x.to_hex_string());
                let mut gradient_text = String::new();

                #[cfg(feature = "unicode")]
                let characters = text.graphemes(true);
                
                #[cfg(not(feature = "unicode"))]
                let characters = text.chars();

                for character in characters {
                    #[cfg(not(feature = "unicode"))]
                    let character = character.to_string();
                    #[cfg(not(feature = "unicode"))]
                    let character = character.as_str();
                    
                    if let Some(color) = colors.next() {
                        gradient_text += &character.colorize(&color);
                    } else {
                        gradient_text += character;
                    }
                }

                gradient_text
            }
            Colour::Solid(color) => text.colorize(color),
        }
    }
}

impl From<&str> for Colour {
    fn from(value: &str) -> Self {
        let value = value.to_lowercase();

        #[cfg(feature = "gradient")]
        if value.starts_with("gradient(") {
            return Self::gradient(
                &value
                    .trim_start_matches("gradient(")
                    .trim_end_matches(')')
                    .split(',')
                    .map(|x| {
                        x.trim()
                            .trim_start_matches('\"')
                            .trim_start_matches('\'')
                            .trim_end_matches('\"')
                            .trim_end_matches('\'')
                    })
                    .collect::<Vec<&str>>(),
            );
        }

        Self::Solid(value)
    }
}
