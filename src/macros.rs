/// [tqdm](https://github.com/tqdm/tqdm) like macro for constructing `kdam::BarIterator` if iterable is given else `kdam::Bar`.
///
/// This macro use `kdam::BarBuilder` for creating `kdam::Bar`.
/// See all available [methods](https://docs.rs/kdam/latest/kdam/struct.BarBuilder.html).
///
/// # Examples
///
/// ```rust
/// use kdam::prelude::*;
///
/// tqdm!();
/// tqdm!(total = 100);
/// tqdm!(total = 100, mininterval = 0.0, colour = "green");
/// tqdm!(0..100);
/// tqdm!(0..100, desc = "0 to 99");
/// tqdm!(["a", "b", "c", "d"].iter());
/// ```
#[macro_export]
macro_rules! tqdm {
    ($($setter_method: ident = $value: expr),*) => {
        kdam::BarBuilder::default()$(.$setter_method($value))*.build()
    };

    ($iterable: expr) => {
        kdam::BarIterator::new_with_bar($iterable, kdam::Bar::default())
    };

    ($iterable: expr, $($setter_method: ident = $value: expr),*) => {
        kdam::BarIterator::new_with_bar($iterable, kdam::BarBuilder::default()$(.$setter_method($value))*.build())
    };
}
