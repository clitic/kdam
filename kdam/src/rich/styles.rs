use std::num::{NonZeroU16, NonZeroI16};
use crate::term::Colorizer;

// Characters
const BAR_CHR: &str = "━";
const BAR_END_CHR: &str = "╸";

// Colours
const BAR_COLOUR: &str = "#F92672";
const BAR_COMPLETED_COLOUR: &str = "#729c1f";
const BAR_PULSE_COLOURS: [&str; 18] = [
    "#3a3a3a", "#3e393b", "#4c383f", "#613545", "#7b334d", "#b72c5e", "#d12a66", "#e6276c",
    "#f42670", "#f92672", "#f42670", "#e6276c", "#d12a66", "#b72c5e", "#993056", "#7b334d",
    "#613545", "#4c383f",
];
const BAR_UNCOMPLETED_COLOUR: &str = "#525252";

pub(super) fn bar(ncols: NonZeroU16, progress: f32) -> String {
    assert!((0.0..=1.0).contains(&progress));

    let ncols = ncols.get();

    if progress >= 1.0 {
        BAR_CHR.repeat(ncols as usize).colorize(BAR_COMPLETED_COLOUR)
    } else {
        let block = (ncols as f32 * progress) as u16;
        (BAR_CHR.repeat(block as usize) + BAR_END_CHR).colorize(BAR_COLOUR)
            + &BAR_CHR
                .repeat((ncols - block - 1) as usize)
                .colorize(BAR_UNCOMPLETED_COLOUR)
    }
}

pub(super) fn pulse(ncols: NonZeroI16, current_time: f32) -> String {
    assert!(current_time.is_sign_positive());

    let ncols = ncols.get();
    let pulse = BAR_PULSE_COLOURS.repeat((ncols as f32 / 18_f32) as usize + 2);

    let pulse_len = pulse.len();
    let offset = (-current_time * 15_f32) as i16 % 18;
    let mut pulse_string = String::new();

    for i in offset..(offset + ncols) {
        if 0 > i {
            pulse_string += &BAR_CHR.colorize(pulse[pulse_len - (-i as usize)]);
        } else {
            pulse_string += &BAR_CHR.colorize(pulse[i as usize]);
        }
    }

    pulse_string
}
