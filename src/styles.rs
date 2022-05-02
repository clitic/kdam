// Animation Unicodes
static FILLUPCHARSET: [&str; 8] = [
    "\u{2581}", "\u{2582}", "\u{2583}", "\u{2584}", "\u{2585}", "\u{2586}", "\u{2587}", "\u{2588}",
];
static TQDMCHARSET: [&str; 8] = [
    "\u{258F}", "\u{258E}", "\u{258D}", "\u{258C}", "\u{258B}", "\u{258A}", "\u{2589}", "\u{2588}",
];
static TQDMASCIICHARSET: [&str; 10] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "#"];

// Spinner Unicodes
// pub(crate) static CLASSICSPINNER: [&str; 4] = ["\\", "|", "/", "-"];
// pub(crate) static FIRACODESPINNER: [&str; 6] = [
//     "\u{EE06}", "\u{EE07}", "\u{EE08}", "\u{EE09}", "\u{EE0A}", "\u{EE0B}",
// ];

/// Different types of pre-configured animation styles for `kdam::Bar`.
#[derive(Debug, Clone)]
pub enum Animation {
    Arrow,
    Classic,
    Custom(Option<&'static [&'static str]>),
    FillUp,
    FiraCode,
    Tqdm,
    TqdmAscii,
}

/// Different ouput locations of `kdam::Bar`.
#[derive(Debug, Clone)]
pub enum Output {
    Stderr,
    Stdout,
}

pub(crate) fn progressive(progress: f32, ncols: i16, animation: Animation) -> String {
    let charset: &[&str];

    match animation {
        Animation::TqdmAscii => charset = &TQDMASCIICHARSET,
        Animation::FillUp => charset = &FILLUPCHARSET,
        Animation::Custom(custom_charset) => {
            charset = if custom_charset.is_some() {
                custom_charset.unwrap()
            } else {
                &TQDMCHARSET
            }
        }
        _ => charset = &TQDMCHARSET,
    }

    let nsyms = charset.len() - 1;
    let (bar_length, frac_bar_length) = crate::format::divmod(
        (progress * ncols as f32 * nsyms as f32) as usize,
        nsyms,
    );
    let mut bar_animation = charset.last().unwrap().repeat(bar_length);

    if bar_length < ncols as usize {
        bar_animation += &charset.get(frac_bar_length + 1).unwrap();
        bar_animation += &" ".repeat((ncols - (bar_length as i16) - 1) as usize);
    }

    bar_animation
}

pub(crate) fn simple(progress: f32, ncols: i16, animation: Animation) -> String {
    let charset;
    let mut fill = " ";

    match animation {
        Animation::Arrow => charset = "=",
        Animation::FiraCode => {
            charset = "\u{EE04}";
            fill = "\u{EE01}";
        }
        _ => {
            charset = "#";
            fill = ".";
        }
    }

    let block = (ncols as f32 * progress) as i16;
    let mut bar_animation = charset.repeat(block as usize);

    if !matches!(animation, Animation::Arrow) {
        bar_animation += &fill.repeat((ncols - block) as usize);
    } else {
        let x = ncols - block - 1;
        if x > 0 {
            bar_animation += ">";
            bar_animation += &" ".repeat(x as usize);
        }
    }

    if matches!(animation, Animation::FiraCode) {
        bar_animation = format!(
            "\u{EE03}{}{}",
            bar_animation,
            if progress >= 1.0 {
                "\u{EE05}"
            } else {
                "\u{EE02}"
            }
        );
    }

    bar_animation
}
