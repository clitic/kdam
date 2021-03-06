use crate::prelude::*;
use crate::Bar;

/// Iterable version of `kdam::Bar`.
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
        let mut pb = Bar::default();
        pb.total = iterable.size_hint().0;

        BarIterator {
            iterable: iterable,
            pb: pb,
        }
    }

    /// Create a new instance of `kdam::BarIterator` from iterable and `kdam::Bar`.
    pub fn new_with_bar(iterable: T, pb: Bar) -> BarIterator<T> {
        let total = iterable.size_hint().0;

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
        if self.pb.started() {
            self.pb.update(1);
        } else {
            self.pb.refresh();
        }

        self.iterable.next()
    }
}

impl<T: DoubleEndedIterator> DoubleEndedIterator for BarIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.pb.started() {
            self.pb.update(1);
        } else {
            self.pb.refresh();
        }

        self.iterable.next_back()
    }
}

/// Rust iterators decoration with `kdam::BarIterator`.
pub trait TqdmIterator
where
    Self: Sized + Iterator,
{
    /// Decorate any sized iterator to `kdam::BarIterator`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kdam::prelude::*;
    ///
    /// fn main() {
    ///     let chars = ["a", "b", "c", "d"];
    ///     let mut charset = String::new();
    ///
    ///     for i in chars.iter().tqdm() {
    ///         charset += i;
    ///     }
    ///
    ///     eprint!("\n");
    ///     assert_eq!(charset, "abcd");
    /// }
    /// ```
    fn tqdm(self) -> BarIterator<Self>;

    /// Decorate any sized iterator to `kdam::BarIterator` with existing `kdam::Bar`.
    fn tqdm_with_bar(self, pb: Bar) -> BarIterator<Self>;
}

impl<S, T: Iterator<Item = S>> TqdmIterator for T {
    fn tqdm(self) -> BarIterator<Self> {
        BarIterator::new(self)
    }

    fn tqdm_with_bar(self, pb: Bar) -> BarIterator<Self> {
        BarIterator::new_with_bar(self, pb)
    }
}
