#[macro_export]
macro_rules! tqdm {
    ($($struct_field: ident = $value: expr),*) => {
        {
            let mut pb = kdam::Bar::default();
            $(
                pb.$struct_field = $value;
            )*
            pb.set_defaults();
            pb
        }
    };

    ($iterable: expr) => {
        {
            let mut pb = kdam::Bar::default();
            pb.set_defaults();
            kdam::BarIterator::new_with_bar($iterable, pb)
        }
    };

    ($iterable: expr, $($struct_field: ident = $value: expr),*) => {
        {
            let mut pb = kdam::Bar::default();
            $(
                pb.$struct_field = $value;
            )*
            pb.set_defaults();
            kdam::BarIterator::new_with_bar($iterable, pb)
        }
    };
}
