//! Sync bars between multiple threads.

static mut LOCKED: bool = false;

/// Wait until lock is free and then acquire it.
pub fn block() {
    loop {
        unsafe {
            if !LOCKED {
                LOCKED = true;
                break;
            }
        }
    }
}

/// Release lock.
pub fn unblock() {
    unsafe {
        LOCKED = false;
    }
}