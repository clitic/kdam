/// [tqdm](https://github.com/tqdm/tqdm) like macro using `kdam::Bar` and `kdam::BarIterator`.
///
/// # Examples
///
/// ```rust
/// use kdam::tqdm;
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
            let mut pb = kdam::Bar::default();
            kdam::BarIterator::new_with_bar($iterable, pb)
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

#[macro_export]
macro_rules! write_at {
    ($position:literal, $($arg:tt)*) => {
        {
            use std::io::Write;

            let mut stdout = std::io::stdout();
            kdam::lock::block();

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
            kdam::lock::unblock();
        }
    }
}
