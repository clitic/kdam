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
            pb.init();
            pb
        }
    };

    ($iterable: expr) => {
        {
            let mut pb = kdam::Bar::default();
            pb.init();
            kdam::BarIterator::new_with_bar($iterable, pb)
        }
    };

    ($iterable: expr, $($struct_field: ident = $value: expr),*) => {
        {
            let mut pb = kdam::Bar::default();
            $(
                pb.$struct_field = $value;
            )*
            pb.init();
            kdam::BarIterator::new_with_bar($iterable, pb)
        }
    };
}
