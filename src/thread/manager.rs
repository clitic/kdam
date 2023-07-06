use crate::std::{Bar, BarExt};
use std::collections::HashSet;

/// RowManager allows to store and update many progress bars.
///
/// `nrows` is the number of progress bars to display at once.
/// All other bars are hidden and visible once any active progress bar is completed.
/// Traces of progress are left in terminal if `leave=true` else progress bar is cleared.
/// Cursor position is not restored by RowManager.
///
/// # Example
///
/// ```
/// use kdam::{tqdm, BarExt, RowManager};
///
/// let mut manager = RowManager::new(3);
/// let pb_index = manager.append(tqdm!(total = 100));
///
/// for _ in 0..100 {
///     manager.get_mut(pb_index).unwrap().update(1);
///     manager.notify(pb_index);
/// }
///
/// manager.bars.remove(pb_index);
/// ```
pub struct RowManager {
    acquired_pos: HashSet<u16>,
    avaliable_pos: HashSet<u16>,
    pub bars: Vec<Bar>,
    bars_true_disable: Vec<bool>,
    nrows: u16,
}

impl RowManager {
    /// Create a new [RowManager](crate::RowManager) instance with specific number of rows.
    ///
    /// # Example
    ///
    /// ```
    /// use kdam::RowManager;
    ///
    /// // Will only display 3 progress bars at once.
    /// let mut manager = RowManager::new(3);
    /// ```
    pub fn new(nrows: u16) -> Self {
        Self {
            acquired_pos: HashSet::new(),
            avaliable_pos: HashSet::new(),
            bars: vec![],
            bars_true_disable: vec![],
            nrows,
        }
    }

    /// Create a new [RowManager](crate::RowManager) instance from terminal window size.
    ///
    /// # Example
    ///
    /// ```
    /// use kdam::RowManager;
    ///
    /// let mut manager = RowManager::from_window_size();
    /// ```
    pub fn from_window_size() -> Self {
        Self {
            acquired_pos: HashSet::new(),
            avaliable_pos: HashSet::new(),

            bars: vec![],
            bars_true_disable: vec![],
            nrows: terminal_size::terminal_size()
                .unwrap_or((terminal_size::Width(0), terminal_size::Height(3)))
                .1
                 .0
                - 2,
        }
    }

    /// Returns the number of progress bars.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.bars.len()
    }

    /// Returns a mutable reference to progress bar.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Bar> {
        self.bars.get_mut(index)
    }

    /// Append a progress bar returning its index.
    pub fn append(&mut self, mut pb: Bar) -> usize {
        pb.position = self.acquired_pos.len() as u16;
        self.bars_true_disable.push(pb.disable);

        if self.nrows > pb.position {
            pb.refresh();
            self.acquired_pos.insert(pb.position);
        } else {
            pb.disable = true;
        }

        self.bars.push(pb);
        self.bars.len() - 1
    }

    /// Update and print the required stuff for progress bar at that index.
    pub fn notify(&mut self, index: usize) {
        let pb = self.bars.get_mut(index).unwrap();

        if pb.completed() && !self.bars_true_disable.get(index).unwrap() {
            if pb.leave {
                let text = pb.render();
                pb.writer.print(format_args!("\r{}\n", text));
            }

            pb.clear();
            pb.disable = true;

            if self.acquired_pos.remove(&pb.position) {
                self.avaliable_pos.insert(pb.position);
            }
        }

        let writer = pb.writer.clone();

        let remaining_bars = self.bars.len()
            - self
                .bars
                .iter()
                .map(|x| x.completed())
                .filter(|x| x.to_owned())
                .count();

        if self.nrows as usize > remaining_bars {
            let mut count = 0;
            for (i, bar) in self.bars.iter_mut().enumerate() {
                if bar.total > bar.get_counter() && !self.bars_true_disable.get(i).unwrap() {
                    if bar.position != count {
                        bar.clear();
                        bar.position = count;
                        bar.disable = false;
                        bar.refresh();
                    }

                    count += 1;
                }
            }
        } else {
            if self.nrows as usize == remaining_bars {
                writer.print_at(
                    (self.acquired_pos.iter().max().unwrap_or(&0) + 1) as usize,
                    format!("\r{}\r", " ".repeat(22)),
                );
            } else {
                writer.print_at(
                    (self.acquired_pos.iter().max().unwrap_or(&0) + 1) as usize,
                    " ... (more hidden) ...",
                );
            }

            for (i, bar) in self.bars.iter_mut().enumerate() {
                if bar.total > bar.get_counter() && !self.bars_true_disable.get(i).unwrap() {
                    if let Some(pos) = self.avaliable_pos.iter().min() {
                        if bar.disable && bar.position != *pos {
                            bar.position = *pos;

                            if self.nrows > bar.position {
                                bar.disable = false;
                            }

                            bar.refresh();

                            if self.avaliable_pos.remove(&bar.position) {
                                self.acquired_pos.insert(bar.position);
                            }
                        }
                    }
                }
            }
        }
    }
}
