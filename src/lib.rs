// export crate
pub extern crate glm;
pub extern crate log;
pub extern crate palette;
pub extern crate simple_logger;

static mut INITED: bool = false;

/// `init`只执行一次
pub fn init() {
    unsafe {
        if !INITED {
            INITED = true;
            simple_logger::init().unwrap();
        }
    }
}

pub mod object;
