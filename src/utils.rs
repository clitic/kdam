use crate::term;
use crate::term::Output;

/// Prints new line charcter n times to the given output location.
pub fn finish(n: usize, location: Output) {
    match location {
        Output::Stderr => term::write_to_stderr(format_args!("{}", "\n".repeat(n))),
        Output::Stdout => term::write_to_stderr(format_args!("{}", "\n".repeat(n))),
    }
}
