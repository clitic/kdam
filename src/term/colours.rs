use unicode_segmentation::UnicodeSegmentation;

#[cfg(target_os = "windows")]
static COLOURS_ENABLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Create ANSI colour escape code from primary colours or hex colour code or rgb(r,g,b).
///
/// # Example
///
/// ```rust
/// use kdam::term::colour;
///
/// assert_eq!(colour("bold red"), "\x1b[31;1m");
/// assert_eq!(colour("blue on white"), "\x1b[34;47m");
/// ```
pub fn colour(colour_code: &str) -> String {
    #[cfg(target_os = "windows")]
    if !COLOURS_ENABLED.load(std::sync::atomic::Ordering::Acquire) {
        std::process::Command::new("cmd")
            .args(["/c", "color"])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        COLOURS_ENABLED.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    let mut color = colour_code.to_uppercase();
    let mut code = "\x1b[".to_string();

    let bg = if let Some(hex_index) = color.find("ON #") {
        let ansi_256 = format!(
            ";48;2;{};{};{}",
            i16::from_str_radix(&colour_code[(hex_index + 4)..(hex_index + 6)], 16).unwrap(),
            i16::from_str_radix(&colour_code[(hex_index + 6)..(hex_index + 8)], 16).unwrap(),
            i16::from_str_radix(&colour_code[(hex_index + 8)..(hex_index + 10)], 16).unwrap()
        );
        color.replace_range(hex_index..(hex_index + 10), "");
        ansi_256
    } else if let Some(rgb_index) = color.find("ON RGB(") {
        let rgb = &color[(rgb_index + 7)..(rgb_index + color[rgb_index..].find(')').unwrap())]
            .split(',')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        color.replace_range(
            rgb_index..(rgb_index + color[rgb_index..].find(')').unwrap() + 1),
            "",
        );
        format!(";48;2;{};{};{}", rgb[0], rgb[1], rgb[2])
    } else if color.contains("ON BLACK") {
        color = color.replace("ON BLACK", "");
        ";40".to_owned()
    } else if color.contains("ON RED") {
        color = color.replace("ON RED", "");
        ";41".to_owned()
    } else if color.contains("ON GREEN") {
        color = color.replace("ON GREEN", "");
        ";42".to_owned()
    } else if color.contains("ON YELLOW") {
        color = color.replace("ON YELLOW", "");
        ";43".to_owned()
    } else if color.contains("ON BLUE") {
        color = color.replace("ON BLUE", "");
        ";44".to_owned()
    } else if color.contains("ON MAGENTA") {
        color = color.replace("ON MAGENTA", "");
        ";45".to_owned()
    } else if color.contains("ON CYAN") {
        color = color.replace("ON CYAN", "");
        ";46".to_owned()
    } else if color.contains("ON WHITE") {
        color = color.replace("ON WHITE", "");
        ";47".to_owned()
    } else {
        "".to_owned()
    };

    if let Some(hex_index) = color.find('#') {
        code += &format!(
            "38;2;{};{};{}",
            i16::from_str_radix(&color[(hex_index + 1)..(hex_index + 3)], 16).unwrap(),
            i16::from_str_radix(&color[(hex_index + 3)..(hex_index + 5)], 16).unwrap(),
            i16::from_str_radix(&color[(hex_index + 5)..(hex_index + 7)], 16).unwrap()
        );
    } else if let Some(rgb_index) = color.find("RGB(") {
        let rgb = &color[(rgb_index + 4)..(rgb_index + color[rgb_index..].find(')').unwrap())]
            .split(',')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        code += &format!("38;2;{};{};{}", rgb[0], rgb[1], rgb[2]);
    } else if color.contains("BRIGHT") {
        if color.contains("BLACK") {
            code += "90";
        } else if color.contains("RED") {
            code += "91";
        } else if color.contains("GREEN") {
            code += "92";
        } else if color.contains("YELLOW") {
            code += "93";
        } else if color.contains("BLUE") {
            code += "94";
        } else if color.contains("MAGENTA") {
            code += "95";
        } else if color.contains("CYAN") {
            code += "96";
        } else if color.contains("WHITE") {
            code += "97";
        } else {
            return "".to_owned();
        }
    } else if color.contains("BLACK") {
        code += "30";
    } else if color.contains("RED") {
        code += "31";
    } else if color.contains("GREEN") {
        code += "32";
    } else if color.contains("YELLOW") {
        code += "33";
    } else if color.contains("BLUE") {
        code += "34";
    } else if color.contains("MAGENTA") {
        code += "35";
    } else if color.contains("CYAN") {
        code += "36";
    } else if color.contains("WHITE") {
        code += "37";
    } else {
        return "".to_owned();
    }

    code += &bg;

    if color.contains("BOLD") {
        code += ";1";
    }

    if color.contains("DIM") {
        code += ";2";
    }

    if color.contains("ITALIC") {
        code += ";3";
    }

    if color.contains("UNDERLINE") {
        code += ";4";
    }

    if color.contains("BLINK") {
        code += ";5";
    }

    if color.contains("REVERSED") {
        code += ";7";
    }

    if color.contains("HIDDEN") {
        code += ";8";
    }

    if color.contains("STRIKETHROUGH") {
        code += ";9";
    }

    code += "m";
    code
}

/// Add ANSI colour escape codes to the given text for printing coloured text in terminal.
/// This trait is only implemented for `&str`.
pub trait Colorizer {
    /// Add ANSI colour escape codes to the given text.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kdam::prelude::*;
    ///
    /// // ANSI [8-16; 4bit]
    ///
    /// println!("{}", "hello world!".colorize("bold red"));
    /// println!("{}", "hello world!".colorize("bright white on blue"));
    ///
    /// // ANSI [256; 8-bit]
    /// // NOT IMPLEMENTED
    ///
    /// // True Colors [(16, 777, 216); 24bit]
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
    fn colorize<'a>(&'a self, code: &str) -> String;

    /// Apply linear gradient ansi escape codes from html colours to the given text with specific length.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kdam::prelude::*;
    ///
    /// println!("{}", "text".gradient(&["#5A56E0", "#EE6FF8"], 4));
    /// ```
    #[cfg(feature = "gradient")]
	#[cfg_attr(docsrs, doc(cfg(feature = "gradient")))]
    fn gradient<'a>(&'a self, codes: &[&str], len: usize) -> String;

    /// Apply linear gradient ansi escape codes from html colours to the given text.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kdam::prelude::*;
    ///
    /// println!("{}", "text".gradient_text(&["#5A56E0", "#EE6FF8"]));
    /// ```
    #[cfg(feature = "gradient")]
	#[cfg_attr(docsrs, doc(cfg(feature = "gradient")))]
    fn gradient_text<'a>(&'a self, codes: &[&str]) -> String;

    /// Inverse of colorize method.
    /// This method trims all ANSI escape codes from given string.
    fn trim_ansi<'a>(&'a self) -> String;

    /// Returns terminal display length of string using graphemes.
    fn len_ansi<'a>(&'a self) -> usize;
}

impl Colorizer for str {
    fn colorize(&self, code: &str) -> String {
        let esc_code = colour(code);

        if esc_code.is_empty() {
            self.to_owned()
        } else {
            esc_code + self + "\x1b[0m"
        }
    }

    #[cfg(feature = "gradient")]
    fn gradient(&self, codes: &[&str], len: usize) -> String {
        let gradient = colorgrad::CustomGradient::new()
            .html_colors(codes)
            .build()
            .unwrap()
            .colors(len);

        let mut gradient_text = String::new();
        let mut gradient = gradient.iter().map(|x| x.to_hex_string());

        for character in self.graphemes(true) {
            if let Some(colour) = gradient.next() {
                gradient_text += &character.colorize(&colour);
            } else {
                gradient_text += character;
            }
        }

        gradient_text
    }

    #[cfg(feature = "gradient")]
    fn gradient_text(&self, codes: &[&str]) -> String {
        self.gradient(codes, self.graphemes(true).count())
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
        self.trim_ansi().graphemes(true).count()
    }
}
