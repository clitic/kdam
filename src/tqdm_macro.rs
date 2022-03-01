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

            if pb.total == 0 {
                pb.total = $iterable.size_hint().0 as u64;
            }

            let pb_iter = kdam::BarIterStruct {
                iterable: $iterable,
                pb: pb
            };

            pb_iter
        }
    };

    ($iterable: expr, $($struct_field: ident = $value: expr),*) => {
        {
            let mut pb = kdam::Bar::default();
            $(
                pb.$struct_field = $value;
            )*
            pb.set_defaults();

            if pb.total == 0 {
                pb.total = $iterable.size_hint().0 as u64;
            }
            
            let pb_iter = kdam::BarIterStruct {
                iterable: $iterable,
                pb: pb
            };

            pb_iter
        }
    };
}
