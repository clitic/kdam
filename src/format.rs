//! Functions for formatting values.

/// Returns floor division and modulus of two values. //try
pub fn divmod<T: Into<usize>>(x: T, y: T) -> (usize, usize) {
    let (new_x, new_y) = (x.into(), y.into());
    (new_x / new_y as usize, new_x % new_y)
}

/// Formats a number (greater than unity) with SI order of magnitude prefixes.
pub fn format_sizeof<T: Into<usize>>(num: T, divisor: T) -> String {
    let mut value = num.into() as f64;
    let new_divisor = divisor.into() as f64;

    for i in ["", "k", "M", "G", "T", "P", "E", "Z"] {
        if value.abs() < 999.5 {
            if value.abs() < 99.95 {
                if value.abs() < 9.995 {
                    return format!("{:1.2}{}", value, i);
                }
                return format!("{:2.1}{}", value, i);
            }
            return format!("{:3.0}{}", value, i);
        }
        value = value / new_divisor;
    }
    return format!("{:3.1}Y", value);
}

/// Formats a number of seconds as a clock time, \[H:\]MM:SS
pub fn format_interval<T: Into<usize>>(seconds: T) -> String {
    let (minutes, seconds) = divmod(seconds.into(), 60);
    let (hours, minutes) = divmod(minutes, 60);

    if hours == 0 {
        return format!("{:#02}:{:#02}", minutes, seconds);
    } else {
        return format!("{:#02}:{:#02}:{:#02}", hours, minutes, seconds);
    }
}

// Intelligent scientific notation (.3g).
// pub fn format_num(n: usize) -> String {
//     let f = format!("{:.3g}", n)
//         .to_string()
//         .replace("+0", "+")
//         .replace("-0", "-");
//     let n = format!("{}", n).to_string();
//     return if f.len() < n.len() { f } else { n };
// }
