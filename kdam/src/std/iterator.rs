/*
    REFERENCES
    ----------

    1. https://github.com/console-rs/indicatif/blob/main/src/iter.rs
    2. https://github.com/console-rs/indicatif/blob/main/src/rayon.rs

*/

use super::{Bar, BarExt};
// use std::ops::{Deref, DerefMut};

#[cfg(feature = "rayon")]
use rayon::iter::{
    plumbing::{Consumer, Folder, Producer, ProducerCallback, UnindexedConsumer},
    IndexedParallelIterator, ParallelIterator,
};

#[cfg(feature = "rayon")]
use std::sync::{Arc, Mutex};

/// Iterable version of [Bar](crate::Bar).
#[derive(Debug)]
pub struct BarIter<T> {
    inner: T,
    #[cfg(feature = "rayon")]
    pb: Arc<Mutex<Bar>>,
    #[cfg(not(feature = "rayon"))]
    pb: Bar,
    started: bool,
}

impl<T> BarIter<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }
}

// impl<T> Deref for BarIter<T> {
//     type Target = Bar;

//     fn deref(&self) -> &Self::Target {
//         // &self.pb
//         &self.pb.lock().unwrap()
//     }
// }

// impl<T> DerefMut for BarIter<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         // &mut self.pb
//         &mut self.pb.lock().unwrap()
//     }
// }

impl<T: ExactSizeIterator> ExactSizeIterator for BarIter<T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<S, T: Iterator<Item = S>> Iterator for BarIter<T> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        if self.started {
            #[cfg(feature = "rayon")]
            self.pb.lock().unwrap().update(1).unwrap();

            #[cfg(not(feature = "rayon"))]
            self.pb.update(1).unwrap();
        } else {
            #[cfg(feature = "rayon")]
            self.pb.lock().unwrap().refresh().unwrap();

            #[cfg(not(feature = "rayon"))]
            self.pb.refresh().unwrap();

            self.started = true;
        }

        self.inner.next()
    }
}

impl<T: DoubleEndedIterator> DoubleEndedIterator for BarIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.started {
            #[cfg(feature = "rayon")]
            self.pb.lock().unwrap().update(1).unwrap();

            #[cfg(not(feature = "rayon"))]
            self.pb.update(1).unwrap();
        } else {
            #[cfg(feature = "rayon")]
            self.pb.lock().unwrap().refresh().unwrap();

            #[cfg(not(feature = "rayon"))]
            self.pb.refresh().unwrap();

            self.started = true;
        }

        self.inner.next_back()
    }
}

#[cfg(feature = "rayon")]
struct BarFolder<C> {
    inner: C,
    pb: Arc<Mutex<Bar>>,
}

#[cfg(feature = "rayon")]
impl<T, C: Folder<T>> Folder<T> for BarFolder<C> {
    type Result = C::Result;

    fn complete(self) -> Self::Result {
        self.inner.complete()
    }

    fn consume(self, item: T) -> Self {
        self.pb.lock().unwrap().update(1).unwrap();
        Self {
            inner: self.inner.consume(item),
            pb: self.pb,
        }
    }

    fn full(&self) -> bool {
        self.inner.full()
    }
}

#[cfg(feature = "rayon")]
struct BarConsumer<C> {
    inner: C,
    pb: Arc<Mutex<Bar>>,
}

#[cfg(feature = "rayon")]
impl<T, C: Consumer<T>> Consumer<T> for BarConsumer<C> {
    type Folder = BarFolder<C::Folder>;
    type Reducer = C::Reducer;
    type Result = C::Result;

    fn full(&self) -> bool {
        self.inner.full()
    }

    fn into_folder(self) -> Self::Folder {
        BarFolder {
            inner: self.inner.into_folder(),
            pb: self.pb,
        }
    }

    fn split_at(self, index: usize) -> (Self, Self, Self::Reducer) {
        let (left, right, reducer) = self.inner.split_at(index);
        (
            Self {
                inner: left,
                pb: self.pb.clone(),
            },
            Self {
                inner: right,
                pb: self.pb,
            },
            reducer,
        )
    }
}

#[cfg(feature = "rayon")]
impl<T, C: UnindexedConsumer<T>> UnindexedConsumer<T> for BarConsumer<C> {
    fn split_off_left(&self) -> Self {
        Self {
            inner: self.inner.split_off_left(),
            pb: self.pb.clone(),
        }
    }

    fn to_reducer(&self) -> Self::Reducer {
        self.inner.to_reducer()
    }
}

#[cfg(feature = "rayon")]
impl<S: Send, T: ParallelIterator<Item = S>> ParallelIterator for BarIter<T> {
    type Item = S;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.inner.drive_unindexed(BarConsumer {
            inner: consumer,
            pb: self.pb,
        })
    }
}

#[cfg(feature = "rayon")]
struct BarProducer<T> {
    inner: T,
    pb: Arc<Mutex<Bar>>,
}

#[cfg(feature = "rayon")]
impl<T, P: Producer<Item = T>> Producer for BarProducer<P> {
    type IntoIter = BarIter<P::IntoIter>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        BarIter {
            inner: self.inner.into_iter(),
            pb: self.pb,
            started: false,
        }
    }

    fn max_len(&self) -> usize {
        self.inner.max_len()
    }

    fn min_len(&self) -> usize {
        self.inner.min_len()
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let (left, right) = self.inner.split_at(index);
        (
            Self {
                inner: left,
                pb: self.pb.clone(),
            },
            Self {
                inner: right,
                pb: self.pb,
            },
        )
    }
}

#[cfg(feature = "rayon")]
impl<S: Send, T: IndexedParallelIterator<Item = S>> IndexedParallelIterator for BarIter<T> {
    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        let consumer = BarConsumer {
            inner: consumer,
            pb: self.pb,
        };
        self.inner.drive(consumer)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn with_producer<CB: ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
        return self.inner.with_producer(Callback {
            inner: callback,
            pb: self.pb,
        });

        struct Callback<CB> {
            inner: CB,
            pb: Arc<Mutex<Bar>>,
        }

        impl<T, CB: ProducerCallback<T>> ProducerCallback<T> for Callback<CB> {
            type Output = CB::Output;

            fn callback<P>(self, inner: P) -> CB::Output
            where
                P: Producer<Item = T>,
            {
                self.inner.callback(BarProducer { inner, pb: self.pb })
            }
        }
    }
}

/// Iterators decoration with [BarIter](crate::BarIter).
pub trait TqdmIterator
where
    Self: Iterator + Sized,
{
    /// Decorate any sized iterator to [BarIter](crate::BarIter).
    ///
    /// # Example
    ///
    /// ```
    /// use kdam::TqdmIterator;
    ///
    /// let chars = ["a", "b", "c", "d"];
    /// let mut charset = String::new();
    ///
    /// for i in chars.iter().tqdm() {
    ///     charset += i;
    /// }
    ///
    /// eprint!("\n");
    /// assert_eq!(charset, "abcd");
    /// ```
    fn tqdm(self) -> BarIter<Self>;

    /// Decorate any sized iterator to [BarIter](crate::BarIter) with existing [Bar](crate::Bar).
    fn tqdm_with_bar(self, pb: Bar) -> BarIter<Self>;
}

impl<S, T: Iterator<Item = S>> TqdmIterator for T {
    fn tqdm(self) -> BarIter<Self> {
        Self::tqdm_with_bar(self, Bar::default())
    }

    fn tqdm_with_bar(self, mut pb: Bar) -> BarIter<Self> {
        if pb.indefinite() {
            pb.total = self.size_hint().0;
        }

        BarIter {
            inner: self,
            #[cfg(feature = "rayon")]
            pb: Arc::new(Mutex::new(pb)),
            #[cfg(not(feature = "rayon"))]
            pb,
            started: false,
        }
    }
}

/// Parallel iterators decoration with [BarIter](crate::BarIter).
#[cfg(feature = "rayon")]
pub trait TqdmParallelIterator
where
    Self: ParallelIterator + Sized,
{
    /// Decorate any sized parallel iterator to [BarIter](crate::BarIter).
    fn tqdm(self) -> BarIter<Self>
    where
        Self: IndexedParallelIterator;

    /// Decorate any sized parallel iterator to [BarIter](crate::BarIter) with existing [Bar](crate::Bar).
    fn tqdm_with_bar(self, pb: Bar) -> BarIter<Self>
    where
        Self: IndexedParallelIterator;
}

#[cfg(feature = "rayon")]
impl<S, T: ParallelIterator<Item = S>> TqdmParallelIterator for T {
    fn tqdm(self) -> BarIter<Self>
    where
        Self: IndexedParallelIterator,
    {
        Self::tqdm_with_bar(self, Bar::default())
    }

    fn tqdm_with_bar(self, mut pb: Bar) -> BarIter<Self>
    where
        Self: IndexedParallelIterator,
    {
        if pb.indefinite() {
            pb.total = self.len();
        }

        BarIter {
            inner: self,
            #[cfg(feature = "rayon")]
            pb: Arc::new(Mutex::new(pb)),
            #[cfg(not(feature = "rayon"))]
            pb,
            started: false,
        }
    }
}
