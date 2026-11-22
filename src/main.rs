#![cfg(windows)]

mod bindings;

use crate::bindings::{
    keys::{
        key_event, send_inputs, Input, VK_CONTROL, VK_F4, VK_LWIN,
    },
    registry::RegistryKey,
};

use std::{io, thread, time::Duration};

const GUID_SIZE: usize = 16;
const ATTEMPT_DELAY: Duration = Duration::from_millis(100);

fn main() -> io::Result<()> {
    thread::spawn(close_extra_desktops)
        .join()
        .map_err(|_| io::Error::other("cleanup thread panicked"))?
}

fn close_extra_desktops() -> io::Result<()> {
    let attempts = desktop_count()?.saturating_sub(1);
    for _ in 0..attempts {
        send_close_desktop_shortcut()?;
        thread::sleep(ATTEMPT_DELAY);
    }
    Ok(())
}

fn desktop_count() -> io::Result<usize> {
    let Some(key) = RegistryKey::open_current_user(
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\VirtualDesktops",
    )?
    else {
        return Ok(1);
    };
    let ids = key.read_binary("VirtualDesktopIDs")?;
    Ok((ids.len() / GUID_SIZE).max(1))
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
