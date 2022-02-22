use crate::std_bar::Bar;

#[derive(Debug)]
pub struct BarIterStruct<T> {
    pub iterable: T,
    pub pb: Bar,
    pub rendered_once: bool,
}

impl<T> std::ops::Deref for BarIterStruct<T> {
    type Target = Bar;

    fn deref(&self) -> &Self::Target {
        &self.pb
    }
}

impl<T> std::ops::DerefMut for BarIterStruct<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pb
    }
}

impl<S, T: Iterator<Item = S>> Iterator for BarIterStruct<T> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iterable.next();
        if self.rendered_once {
            self.pb.update(1);
        } else {
            self.pb.refresh();
            self.rendered_once = true;
        }

        item
    }
}

impl<T: ExactSizeIterator> ExactSizeIterator for BarIterStruct<T> {
    fn len(&self) -> usize {
        self.iterable.len()
    }
}

impl<T: DoubleEndedIterator> DoubleEndedIterator for BarIterStruct<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = self.iterable.next_back();

        if self.rendered_once {
            self.pb.update(1);
        } else {
            self.pb.refresh();
            self.rendered_once = true;
        }

        item
    }
}

pub trait BarIter
where
    Self: Sized + Iterator,
{
    fn progress(self) -> BarIterStruct<Self>;
    fn progress_total(self, total: u64) -> BarIterStruct<Self>;
}

impl<S, T: Iterator<Item = S>> BarIter for T {
    fn progress(self) -> BarIterStruct<Self> {
        let total = self.size_hint().0;
        BarIterStruct {
            iterable: self,
            pb: Bar {
                total: total as u64,
                ..Default::default()
            },
            rendered_once: false,
        }
    }

    fn progress_total(self, total: u64) -> BarIterStruct<Self> {
        let mut pb = self.progress();
        pb.total = total;
        pb
    }
}
