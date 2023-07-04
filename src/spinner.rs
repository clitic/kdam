/// Generic spinner for rendering spinner animations.
///
/// See more styles at [rich repository](https://github.com/Textualize/rich/blob/master/rich/_spinners.py).
#[derive(Debug, Clone)]
pub struct Spinner {
    frames: Vec<String>,
    interval: f32,
    speed: f32,
}

impl Spinner {
    /// Create a new [Spinner](Self).
    ///
    /// # Example
    ///
    /// ```
    /// use kdam::Spinner;
    ///
    /// let spinner = Spinner::new(
    ///     &[
    ///         "▁▂▃", "▂▃▄", "▃▄▅", "▄▅▆", "▅▆▇",
    ///         "▆▇█", "▇█▇", "█▇▆", "▇▆▅", "▆▅▄",
    ///         "▅▄▃", "▄▃▂", "▃▂▁"
    ///     ],
    ///     30.0,
    ///     1.0
    /// );
    ///
    /// println!("{}", spinner.render_frame(2.89));
    /// ```
    pub fn new(frames: &[&str], interval: f32, speed: f32) -> Self {
        Self {
            frames: frames
                .iter()
                .map(|x| String::from(*x))
                .collect(),
            interval,
            speed,
        }
    }

    /// Render a single frame.
    pub fn render_frame(&self, elapsed_time: f32) -> String {
        let frame_no = (elapsed_time * self.speed) / (self.interval / 1000.0);
        self.frames
            .get(frame_no as usize % self.frames.len())
            .unwrap()
            .to_owned()
    }

    /// Render multiple frames upto `ncols` with an pulsating animation.
    pub fn render_frames(&self, elapsed_time: f32, ncols: i16) -> String {
        let pulse = self
            .frames
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
            .repeat((ncols as f32 / self.frames.len() as f32) as usize + 2);
        let pulse_len = pulse.len();
        let offset = (elapsed_time * 15_f32) as i16 % self.frames.len() as i16;
        let mut pulse_string = String::new();

        for i in offset..(offset + ncols) {
            if 0 > i {
                pulse_string += pulse[pulse_len - (-i as usize)];
            } else {
                pulse_string += pulse[i as usize];
            }
        }

        pulse_string
    }
}
