use std::sync::mpsc;

use crate::iterator_bar::BarIterator;
use crate::std_bar::Bar;
use crate::term;

/// Support for displaying multiple parllel progress bars.
///
/// # Example
///
/// ```rust
/// use kdam::{tqdm, BarMulti};
/// use std::thread;
///
/// fn main() {
///     let mut bar_handle = BarMulti::new();
///     let mut pb = tqdm!(0..100);
///     bar_handle.append(&mut pb);
///
///     let thread = thread::spawn(move || for _ in pb {});
///
///     // listen without blocking main thread
///     thread::spawn(move || {
///         bar_handle.listen();
///     });
///
///     thread.join().unwrap();
/// }
/// ```
#[derive(Debug)]
pub struct BarMulti {
    bars: Vec<String>,
    nrows: i16,
    tx: mpsc::Sender<(i16, String, bool)>,
    rx: mpsc::Receiver<(i16, String, bool)>,
}

impl BarMulti {
    /// Create a new instance of `kdam::BarMulti`.
    pub fn new() -> BarMulti {
        let (tx, rx) = mpsc::channel();
        BarMulti {
            bars: vec![],
            nrows: 0,
            tx,
            rx,
        }
    }

    /// Append instance of `kdam::Bar` to stack.
    pub fn append(&mut self, pb: &mut Bar) {
        let index = self.bars.len() as i16;
        self.bars.push(String::new());
        self.nrows += 1;

        pb.internal.nrows = index;
        pb.internal.tx = Some(self.tx.clone());
    }

    /// Append instance of `kdam::BarIterator` to stack.
    pub fn append_iter<T: Iterator>(&mut self, pb_iter: &mut BarIterator<T>) {
        let index = self.bars.len() as i16;
        self.bars.push(String::new());
        self.nrows += 1;

        pb_iter.pb.internal.nrows = index;
        pb_iter.pb.internal.tx = Some(self.tx.clone());
    }

    /// Display all progress bar whenever any bar is updated.
    pub fn listen(&mut self) {
        let mut first = true;

        while self.nrows > 0 {
            let (index, info, finished) = self.rx.recv().unwrap();
            self.bars[index as usize] = info;

            if !first {
                term::move_up(self.bars.len() as u16);
            } else {
                first = false;
            }

            let mut out = String::new();
            for bar in &self.bars {
                out.push_str(&format!("\r{}\n", bar));
            }

            print!("{}", out);

            if finished {
                self.nrows -= 1;
            }
        }
    }
}
