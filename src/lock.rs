//! Thread safe sync between multiple bars.

static mut LOCKED: bool = false;

/// Wait until lock is free and then acquire it.
pub fn acquire() {
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
pub fn release() {
    unsafe {
        LOCKED = false;
    }
}
