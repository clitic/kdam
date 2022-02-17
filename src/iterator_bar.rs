use crate::std_bar::Bar;

#[derive(Debug)]
pub struct BarIter<I: Iterator> {
    pub iterable: I,
    pub pb: Bar,
    pub rendered_once: bool,
}

impl<I: Iterator> std::ops::Deref for BarIter<I> {
    type Target = Bar;

    fn deref(&self) -> &Self::Target {
        &self.pb
    }
}

impl<I: Iterator> std::ops::DerefMut for BarIter<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pb
    }
}

impl<I: Iterator> Iterator for BarIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iterable.next();
        if self.rendered_once {
            self.pb.update(1);
        } else {
            self.pb.refresh();
            self.rendered_once = true;
        }

        if next.is_some() {
            next
        } else {
            None
        }
    }
}
