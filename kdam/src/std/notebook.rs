use std::sync::atomic::{AtomicBool, Ordering};

static RUNNING: AtomicBool = AtomicBool::new(false);

/// Set whether `kdam` is running inside jupyter notebook or not.
pub fn set_notebook(running: bool) {
    RUNNING.store(running, Ordering::SeqCst);
}

pub(super) fn running() -> bool {
    RUNNING.load(Ordering::Acquire)
}
