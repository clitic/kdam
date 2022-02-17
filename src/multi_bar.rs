use std::sync::mpsc;

use crate::std_bar::Bar;
use crate::term;

#[derive(Debug)]
pub struct MultiBar {
    bars: Vec<String>,
    nrows: u16,
    tx: mpsc::Sender<(u16, String, bool)>,
    rx: mpsc::Receiver<(u16, String, bool)>,
}

impl MultiBar {
    pub fn new() -> MultiBar {
        let (tx, rx) = mpsc::channel();
        MultiBar {
            bars: vec![],
            nrows: 0,
            tx,
            rx,
        }
    }

    pub fn append(&mut self, pb: &mut Bar) {
        let index = self.bars.len() as u16;
        self.bars.push(String::new());
        self.nrows += 1;

        pb.nrows = index;
        pb.tx = Some(self.tx.clone());
    }

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
