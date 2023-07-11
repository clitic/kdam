/*
    REFERENCES:
    ----------

    1. https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797

*/

use crate::utils;
use std::sync::atomic::{AtomicBool, Ordering};

// #[cfg(feature = "crossterm")]
// use crossterm::style::Stylize;

#[cfg(target_os = "windows")]
static COLOURS_ENABLED: AtomicBool = AtomicBool::new(false);

static COLORIZE: AtomicBool = AtomicBool::new(true);

const COLOURS: [&str; 8] = [
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
];
const COLOUR_ATTRIBUTES: [&str; 8] = [
    "bold",
    "dim",
    "italic",
    "underline",
    "blink",
    "reversed",
    "hidden",
    "strikethrough",
];

/// Enable/Disable colorization property of [colorizer](crate::term::Colorizer) trait.
/// Colorization is done always by default.
///
/// **TIP**: Use [IsTerminal](https://doc.rust-lang.org/stable/std/io/trait.IsTerminal.html)
/// trait to detect real terminals and then use this function.
pub fn set_colorize(always: bool) {
    COLORIZE.store(always, Ordering::SeqCst);
}

// #FFFFFF -> Some((255, 255, 255))
fn hex_to_rgb(code: &str) -> Option<(u8, u8, u8)> {
    if code.len() == 7 {
        Some((
            u8::from_str_radix(&code[1..3], 16).ok()?,
            u8::from_str_radix(&code[3..5], 16).ok()?,
            u8::from_str_radix(&code[5..7], 16).ok()?,
        ))
    } else {
        None
    }
}

// ansi(15) -> Some(15)
fn parse_ansi(code: &str) -> Option<u8> {
    code.get(5..(code.len() - 1))?.parse::<u8>().ok()
}

// rgb(255,255,255) -> Some((255, 255, 255))
fn parse_rgb(code: &str) -> Option<(u8, u8, u8)> {
    let mut values = code
        .get(4..(code.len() - 1))?
        .split(',')
        .filter_map(|x| x.trim().parse::<u8>().ok());
    Some((values.next()?, values.next()?, values.next()?))
}

/// Create ANSI colour escape code from primary colours or hex colour code or rgb(r,g,b).
///
/// # Example
///
/// ```
/// use kdam::term::colour;
///
/// assert_eq!(colour("bold red"), "\x1b[31;1m");
/// assert_eq!(colour("blue on white"), "\x1b[34;47m");
/// ```
pub fn colour(code: &str) -> Option<String> {
    #[cfg(target_os = "windows")]
    if !COLOURS_ENABLED.load(Ordering::Acquire) {
        std::process::Command::new("cmd")
            .args(["/c", "color"])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        COLOURS_ENABLED.store(true, Ordering::SeqCst);
    }

    let mut code = code.to_lowercase();
    let mut bg = None;

    if let Some(index) = code.find("on #") {
        let end = 3 + 7;
        let (r, g, b) = hex_to_rgb(code.get((index + 3)..(index + end))?)?;
        code.replace_range(index..(index + end), "");
        bg = Some(format!("48;2;{};{};{}", r, g, b));
    } else if let Some(index) = code.find("on rgb(") {
        let end = 3 + code.get((index + 3)..)?.find(')')? + 1;
        let (r, g, b) = parse_rgb(code.get((index + 3)..(index + end))?)?;
        code.replace_range(index..(index + end), "");
        bg = Some(format!("48;2;{};{};{}", r, g, b));
    } else if let Some(index) = code.find("on ansi(") {
        let end = 4 + code.get((index + 4)..)?.find(')')? + 1;
        let number = parse_ansi(code.get((index + 3)..(index + end))?)?;
        code.replace_range(index..(index + end), "");
        bg = Some(format!("48;5;{}", number));
    } else {
        let mut number = 100_u8;

        for colour in COLOURS {
            let bright_bg_colour = "on bright ".to_owned() + colour;

            if let Some(index) = code.find(&bright_bg_colour) {
                code.replace_range(index..(index + bright_bg_colour.len()), "");
                bg = Some(number.to_string());
                break;
            }

            number += 1;
        }

        if bg.is_none() {
            number = 40;

            for colour in COLOURS {
                let bg_colour = "on ".to_owned() + colour;

                if let Some(index) = code.find(&bg_colour) {
                    code.replace_range(index..(index + bg_colour.len()), "");
                    bg = Some(number.to_string());
                    break;
                }

                number += 1;
            }
        }
    }

    let mut fg = None;

    if let Some(index) = code.find('#') {
        let end = 7;
        let (r, g, b) = hex_to_rgb(code.get(index..(index + end))?)?;
        code.replace_range(index..(index + end), "");
        fg = Some(format!("38;2;{};{};{}", r, g, b));
    } else if let Some(index) = code.find("rgb(") {
        let end = code.get(index..)?.find(')')? + 1;
        let (r, g, b) = parse_rgb(code.get(index..(index + end))?)?;
        code.replace_range(index..(index + end), "");
        fg = Some(format!("38;2;{};{};{}", r, g, b));
    } else if let Some(index) = code.find("ansi(") {
        let end = code.get(index..)?.find(')')? + 1;
        let number = parse_ansi(code.get(index..(index + end))?)?;
        code.replace_range(index..(index + end), "");
        fg = Some(format!("38;5;{}", number));
    } else {
        let mut number = 90_u8;

        for colour in COLOURS {
            let bright_fg_colour = "bright ".to_owned() + colour;

            if let Some(index) = code.find(&bright_fg_colour) {
                code.replace_range(index..(index + bright_fg_colour.len()), "");
                fg = Some(number.to_string());
                break;
            }

            number += 1;
        }

        if fg.is_none() {
            number = 30;

            for fg_colour in COLOURS {
                if let Some(index) = code.find(fg_colour) {
                    code.replace_range(index..(index + fg_colour.len()), "");
                    fg = Some(number.to_string());
                    break;
                }

                number += 1;
            }
        }
    }

    let mut attributes = String::new();
    let mut number = 1_u8;

    for attribute in COLOUR_ATTRIBUTES {
        if let Some(index) = code.find(attribute) {
            code.replace_range(index..(index + attribute.len()), "");

            if !attributes.is_empty() {
                attributes.push(';')
            }

            attributes += &number.to_string();
        }

        number += 1;
    }

    let attributes = if attributes.is_empty() {
        None
    } else {
        Some(attributes)
    };
    let escape_code = "\x1b[".to_owned();

    match (fg, bg, attributes) {
        (None, None, Some(z)) => Some(escape_code + &z + "m"),
        (None, Some(y), None) => Some(escape_code + &y + "m"),
        (None, Some(y), Some(z)) => Some(escape_code + &y + ";" + &z + "m"),
        (Some(x), None, None) => Some(escape_code + &x + "m"),
        (Some(x), None, Some(z)) => Some(escape_code + &x + ";" + &z + "m"),
        (Some(x), Some(y), None) => Some(escape_code + &x + ";" + &y + "m"),
        (Some(x), Some(y), Some(z)) => Some(escape_code + &x + ";" + &y + ";" + &z + "m"),
        _ => None,
    }
}

/// Add ANSI colour escape codes to the given text for printing coloured text in terminal.
pub trait Colorizer {
    /// Add ANSI colour escape codes to the given text.
    ///
    /// # Example
    ///
    /// ```
    /// use kdam::term::Colorizer;
    ///
    /// // ANSI [8-16; 4-bit]
    ///
    /// println!("{}", "hello world!".colorize("bold red"));
    /// println!("{}", "hello world!".colorize("bright white on blue"));
    ///
    /// // ANSI [256; 8-bit]
    /// 
    /// println!("{}", "hello world!".colorize("ansi(200)"));
    /// println!("{}", "hello world!".colorize("ansi(0) on ansi(255)"));
    ///
    /// // True Colours [(0-255, 0-255, 0-255); 24-bit]
    ///
    /// println!("{}{}",
    ///     "hello".colorize("#171717 on #00de6d"),
    ///     " world!".colorize("#ffffff on #007272")
    /// );
    /// println!("{}{}",
    ///     "hello".colorize("rgb(23,23,23) on rgb(0,255,109)"),
    ///     " world!".colorize("rgb(255,255,255) on rgb(0,144,144)")
    /// );
    /// ```
    fn colorize(&self, code: &str) -> String;

    /// Inverse of colorize method.
    /// This method trims all ANSI escape codes from given string.
    fn trim_ansi(&self) -> String;

    /// Returns terminal display length of string using graphemes.
    fn len_ansi(&self) -> usize;
}

impl Colorizer for str {
    fn colorize(&self, code: &str) -> String {
        if !COLORIZE.load(Ordering::Acquire) {
            return self.to_owned();
        }

        let escape_code = colour(code);

        if let Some(escape_code) = escape_code {
            escape_code + self + "\x1b[0m"
        } else {
            self.to_owned()
        }
    }

    fn trim_ansi(&self) -> String {
        let mut text = self.replace("\x1b[0m", "");

        while let Some(start) = text.find("\x1b[") {
            text = text.replace(
                &text[start..(start + text[start..].find('m').unwrap() + 1)],
                "",
            );
        }

        text
    }

    fn len_ansi(&self) -> usize {
        utils::len(&self.trim_ansi())
    }
}
