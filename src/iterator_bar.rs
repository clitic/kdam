use crate::std_bar::Bar;

/// Struct which implements iterator trait and progress bar display updates for `kdam::Bar`.
#[derive(Debug)]
pub struct BarIterator<T> {
    /// Iterator to decorate with a progress bar.
    pub iterable: T,
    /// Instance of `kdam::Bar` to display progress updates for iterable.
    pub pb: Bar,
}

impl<T: Iterator> BarIterator<T> {
    /// Create a new instance of `kdam::BarIterator` from iterable.
    pub fn new(iterable: T) -> BarIterator<T> {
        let total = iterable.size_hint().0;
        BarIterator {
            iterable: iterable,
            pb: Bar {
                total: total as u64,
                ..Default::default()
            },
        }
    }

    /// Create a new instance of `kdam::BarIterator` from iterable and `kdam::Bar`.
    pub fn new_with_bar(iterable: T, pb: Bar) -> BarIterator<T> {
        let total = iterable.size_hint().0 as u64;

        let mut pb_iter = BarIterator {
            iterable: iterable,
            pb: pb,
        };

        if pb_iter.pb.total == 0 {
            pb_iter.pb.total = total;
        }

        pb_iter
    }
}

impl<T> std::ops::Deref for BarIterator<T> {
    type Target = Bar;

    fn deref(&self) -> &Self::Target {
        &self.pb
    }
}

impl<T> std::ops::DerefMut for BarIterator<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pb
    }
}

impl<T: ExactSizeIterator> ExactSizeIterator for BarIterator<T> {
    fn len(&self) -> usize {
        self.iterable.len()
    }
}

impl<S, T: Iterator<Item = S>> Iterator for BarIterator<T> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pb.internal.started {
            self.pb.update(1);
        } else {
            self.pb.refresh();
        }

        self.iterable.next()
    }
}

impl<T: DoubleEndedIterator> DoubleEndedIterator for BarIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.pb.internal.started {
            self.pb.update(1);
        } else {
            self.pb.refresh();
        }

        self.iterable.next_back()
    }
}

/// Rust iterators decoration with `kdam::BarIterator`.
pub trait BarProgress
where
    Self: Sized + Iterator,
{
    /// Decorate any sized iterator to `kdam::BarIterator`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kdam::BarIter;
    ///
    /// fn main() {
    ///     let chars = ["a", "b", "c", "d"];
    ///     let mut charset = String::new();
    ///
    ///     for i in chars.iter().progress() {
    ///         charset += i;
    ///     }
    ///
    ///     assert_eq!(charset, "abcd");
    /// }
    /// ```
    fn progress(self) -> BarIterator<Self>;
}

impl<S, T: Iterator<Item = S>> BarProgress for T {
    fn progress(self) -> BarIterator<Self> {
        BarIterator::new(self)
    }
}
