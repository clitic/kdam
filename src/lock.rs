//! Thread safe sync between multiple bars.

use std::sync::atomic::{AtomicBool, Ordering};

static LOCKED: AtomicBool = AtomicBool::new(false);

/// Wait until lock is free and then acquire it.
pub fn acquire() {
    loop {
        if !LOCKED.load(Ordering::Acquire) {
            LOCKED.store(true, Ordering::SeqCst);
            break;
        }
    }
}

/// Release lock.
pub fn release() {
    LOCKED.store(false, Ordering::Release);
}
