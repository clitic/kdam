use crate::term::Colorizer;

/// Panics ->  capacity overflow, if ncols == 0
pub(crate) fn bar(progress: f32, ncols: i16) -> String {
    if progress >= 1.0 {
        "━".repeat(ncols as usize).colorize("#729c1f")
    } else {
        let block = (ncols as f32 * progress) as i16;
        ("━".repeat(block as usize) + "╸").colorize("#F92672")
            + &"━".repeat((ncols - block - 1) as usize).colorize("#525252")
    }
}

pub(crate) fn pulse(ncols: i16, current_time: f32) -> String {
    let pulse = [
        "#3a3a3a", "#3e393b", "#4c383f", "#613545", "#7b334d", "#b72c5e", "#d12a66", "#e6276c",
        "#f42670", "#f92672", "#f42670", "#e6276c", "#d12a66", "#b72c5e", "#993056", "#7b334d",
        "#613545", "#4c383f",
    ]
    .repeat((ncols as f32 / 18_f32) as usize + 2);

    let pulse_len = pulse.len();
    let offset = (-current_time * 15_f32) as i16 % 18;
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
