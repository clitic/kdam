/// returns floor division and modulus of two values.
pub fn divmod(x: u64, y: u64) -> (u64, u64) {
    (x / y as u64, x % y)
}

/// formats a number (greater than unity) with SI order of magnitude prefixes.
pub fn format_sizeof(num: usize, divisor: usize) -> String {
    let mut value = num as f64;
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
        value = value / divisor as f64;
    }
    return format!("{:3.1}Y", value);
}

/// formats a number of seconds as a clock time, \[H:\]MM:SS
pub fn format_interval(seconds: u64) -> String {
    let (minutes, seconds) = divmod(seconds, 60);
    let (hours, minutes) = divmod(minutes, 60);

    if hours == 0 {
        return format!("{:#02}:{:#02}", minutes, seconds);
    } else {
        return format!("{:#02}:{:#02}:{:#02}", hours, minutes, seconds);
    }
}

// intelligent scientific notation (.3g).
// pub fn format_num(n: usize) -> String {
//     let f = format!("{:.3g}", n)
//         .to_string()
//         .replace("+0", "+")
//         .replace("-0", "-");
//     let n = format!("{}", n).to_string();
//     return if f.len() < n.len() { f } else { n };
// }
