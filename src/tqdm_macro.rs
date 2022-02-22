#[macro_export]
macro_rules! tqdm {
    ($($struct_field: ident = $value: expr),*) => {
        {
            let mut pb = kdam::Bar::default();
            $(
                pb.$struct_field = $value;
            )*
            
            pb.i = pb.initial;

            if !matches!(pb.animation, kdam::Animation::TqdmAscii) {
                pb.set_animation(pb.animation.clone());
            }

            pb.set_colour(&pb.colour.clone());

            pb
        }
    };

    ($iterable: expr) => {
        {
            let mut pb = kdam::Bar::default();
            pb.i = pb.initial;

            if pb.total == 0 {
                pb.total = $iterable.size_hint().0 as u64;
            }

            let pb_iter = kdam::BarIterStruct {
                iterable: $iterable,
                pb: pb,
                rendered_once: false
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

            pb.i = pb.initial;

            if !matches!(pb.animation, kdam::Animation::TqdmAscii) {
                pb.set_animation(pb.animation.clone());
            }

            pb.set_colour(&pb.colour.clone());

            if pb.total == 0 {
                pb.total = $iterable.size_hint().0 as u64;
            }
            
            let pb_iter = kdam::BarIterStruct {
                iterable: $iterable,
                pb: pb,
                rendered_once: false
            };

            pb_iter
        }
    };
}
