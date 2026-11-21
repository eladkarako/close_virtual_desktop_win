use std::{thread, time::Duration};
use winapi::um::winuser::{
    keybd_event, KEYEVENTF_KEYUP, VK_CONTROL, VK_F4, VK_LWIN,
};

fn key(
    vk: i32,
    flags: u32,
) {
    unsafe {
        keybd_event(vk as u8, 0, flags, 0);
    }
}

fn main() {
    for _ in 0..32 {
        key(VK_LWIN, 0);
        key(VK_CONTROL, 0);
        key(VK_F4, 0);
        thread::sleep(Duration::from_millis(80));

        key(VK_F4, KEYEVENTF_KEYUP);
        key(VK_CONTROL, KEYEVENTF_KEYUP);
        key(VK_LWIN, KEYEVENTF_KEYUP);
        thread::sleep(Duration::from_millis(120));
    }
}
