static mut LOCKED: bool = false;

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

pub fn unblock() {
    unsafe {
        LOCKED = false;
    }
}