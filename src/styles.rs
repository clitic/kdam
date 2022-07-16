use crate::term::Colorizer;

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
    Custom(&'static [&'static str]),
    FillUp,
    FiraCode,
    Tqdm,
    TqdmAscii,
}

pub(crate) fn progressive(progress: f32, ncols: i16, animation: Animation) -> String {
    let charset: &[&str];

    match animation {
        Animation::TqdmAscii => charset = &TQDMASCIICHARSET,
        Animation::FillUp => charset = &FILLUPCHARSET,
        Animation::Custom(custom_charset) => charset = custom_charset,
        _ => charset = &TQDMCHARSET,
    }

    let nsyms = charset.len() - 1;
    let (bar_length, frac_bar_length) =
        crate::format::divmod((progress * ncols as f32 * nsyms as f32) as usize, nsyms);
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

pub(crate) fn rich_bar(progress: f32, ncols: i16) -> String {
    let block = (ncols as f32 * progress) as i16;
    let x = ncols - block - 1;

    if x >= 0 {
        ("━".repeat(block as usize) + "╸").colorize("#F92672")
            + &"━".repeat(x as usize).colorize("#525252")
    } else {
        "━".repeat(ncols as usize).colorize("#729c1f")
    }
}

pub(crate) fn rich_pulse(ncols: i16, current_time: f32) -> String {
    let pulse: Vec<String> = [
        "#3a3a3a", "#3e393b", "#4c383f", "#613545", "#7b334d", "#b72c5e", "#d12a66", "#e6276c",
        "#f42670", "#f92672", "#f42670", "#e6276c", "#d12a66", "#b72c5e", "#993056", "#7b334d",
        "#613545", "#4c383f",
    ]
    .repeat((ncols as f32 / 18 as f32) as usize + 2)
    .iter()
    .map(|x| "━".colorize(x))
    .collect();

    let offset = (-current_time * 15 as f32) as i16 % 18;
    let mut pulse_string = String::new();

    for i in offset..(offset + ncols) {
        if i.is_negative() {
            pulse_string += pulse.get(pulse.len() - (i * -1) as usize).unwrap();
        } else {
            pulse_string += pulse.get(i as usize).unwrap();
        }
    }

    pulse_string
}
