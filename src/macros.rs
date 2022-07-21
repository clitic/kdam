/// [tqdm](https://github.com/tqdm/tqdm) like macro for constructing `kdam::Bar` if iterable is not given else `kdam::BarIterator`.
///
/// See available all [fields](https://docs.rs/kdam/latest/kdam/struct.Bar.html#fields).
/// 
/// # Examples
///
/// ```rust
/// use kdam::prelude::*;
///
/// tqdm!();
/// tqdm!(total = 100);
/// tqdm!(total = 100, mininterval = 0.0, colour = "green".to_string());
/// tqdm!(0..100);
/// tqdm!(0..100, desc = "0 to 99".to_string());
/// tqdm!(["a", "b", "c", "d"].iter());
/// ```
#[macro_export]
macro_rules! tqdm {
    ($($struct_field: ident = $value: expr),*) => {
        {
            let mut pb = kdam::Bar::default();
            $(
                pb.$struct_field = $value;
            )*
            pb
        }
    };

    ($iterable: expr) => {
        {
            kdam::BarIterator::new_with_bar($iterable, kdam::Bar::default())
        }
    };

    ($iterable: expr, $($struct_field: ident = $value: expr),*) => {
        {
            let mut pb = kdam::Bar::default();
            $(
                pb.$struct_field = $value;
            )*
            kdam::BarIterator::new_with_bar($iterable, pb)
        }
    };
}

/// Prints to the standard error at specified position.
///
/// Also cursor position is restored to original position after print.
///
/// # Example
///
/// ```rust
/// use kdam::prelude::*;
/// 
/// eprint_at!(1, "1 + 1 = {}", 2);
/// ```
#[macro_export]
macro_rules! eprint_at {
    ($position:tt, $($arg:tt)*) => {
        {
            use std::io::Write;

            let mut stdout = std::io::stderr();
            kdam::lock::acquire();

            if $position > 0 {
                stdout.write_fmt(format_args!(
                    "{}{}{}",
                    "\n".repeat($position as usize),
                    format!($($arg)*),
                    format!("\x1b[{}A", $position)
                )).unwrap();
            } else {
                stdout.write_fmt(format_args!($($arg)*)).unwrap();
            }

            stdout.flush().unwrap();
            kdam::lock::release();
        }
    }
}
