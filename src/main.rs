#![cfg(windows)]

mod bindings;

use crate::bindings::{
    key_event, send_inputs, Input, VK_CONTROL, VK_F4, VK_LWIN,
};
use std::{io, thread, time::Duration};

const CLOSE_ATTEMPTS: u32 = 32;
const ATTEMPT_DELAY: Duration = Duration::from_millis(100);

fn main() -> io::Result<()> {
    thread::spawn(close_extra_desktops)
        .join()
        .map_err(|_| io::Error::other("cleanup thread panicked"))?
}

fn close_extra_desktops() -> io::Result<()> {
    for _ in 0..CLOSE_ATTEMPTS {
        send_close_desktop_shortcut()?;
        thread::sleep(ATTEMPT_DELAY);
    }
    Ok(())
}

fn send_close_desktop_shortcut() -> io::Result<()> {
    send_inputs(&close_desktop_chord()).map_err(|err| {
        io::Error::other(format!(
            "{err}; cleanup: {}",
            release_held_keys()
        ))
    })
}

fn close_desktop_chord() -> [Input; 6] {
    [
        key_event(VK_LWIN, false),
        key_event(VK_CONTROL, false),
        key_event(VK_F4, false),
        key_event(VK_F4, true),
        key_event(VK_CONTROL, true),
        key_event(VK_LWIN, true),
    ]
}

/// Best-effort release of keys that may still be held after a partial send.
fn release_held_keys() -> String {
    let releases = [
        key_event(VK_F4, true),
        key_event(VK_CONTROL, true),
        key_event(VK_LWIN, true),
    ];
    match send_inputs(&releases) {
        Ok(()) => "all key releases accepted".to_owned(),
        Err(err) => err.to_string(),
    }
}
