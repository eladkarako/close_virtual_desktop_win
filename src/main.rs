#![cfg(windows)]

use anyhow::{anyhow, bail, Result};
use std::{mem::size_of, thread, time::Duration};
use winapi::um::{
    errhandlingapi::GetLastError,
    winuser::{
        INPUT_u, SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT,
        KEYEVENTF_KEYUP, VK_CONTROL, VK_F4, VK_LWIN,
    },
};

fn main() -> Result<()> {
    thread::spawn(close_extra_desktops)
        .join()
        .map_err(|_| anyhow!("cleanup thread panicked"))??;
    Ok(())
}

fn close_extra_desktops() -> Result<()> {
    for _ in 0..32 {
        send_close_desktop_shortcut()?;
        thread::sleep(Duration::from_millis(200));
    }
    Ok(())
}

fn send_close_desktop_shortcut() -> Result<()> {
    let key_input = |key: i32, flags: u32| {
        let mut u: INPUT_u = unsafe { std::mem::zeroed() };

        // SAFETY: `ki_mut` selects the KEYBDINPUT variant, which is fully initialized here.
        unsafe {
            *u.ki_mut() = KEYBDINPUT {
                wVk: key as u16,
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            };
        }

        INPUT {
            type_: INPUT_KEYBOARD,
            u,
        }
    };

    let mut inputs = [
        key_input(VK_LWIN, 0),
        key_input(VK_CONTROL, 0),
        key_input(VK_F4, 0),
        key_input(VK_F4, KEYEVENTF_KEYUP),
        key_input(VK_CONTROL, KEYEVENTF_KEYUP),
        key_input(VK_LWIN, KEYEVENTF_KEYUP),
    ];

    // SAFETY: The initialized input array remains alive for the call.
    let sent = unsafe {
        SendInput(
            inputs.len() as u32,
            inputs.as_mut_ptr(),
            size_of::<INPUT>() as i32,
        )
    };

    if sent != inputs.len() as u32 {
        let error = unsafe { GetLastError() };
        let mut releases = [
            key_input(VK_F4, KEYEVENTF_KEYUP),
            key_input(VK_CONTROL, KEYEVENTF_KEYUP),
            key_input(VK_LWIN, KEYEVENTF_KEYUP),
        ];

        // SAFETY: The initialized release array remains alive for the call.
        let released = unsafe {
            SendInput(
                releases.len() as u32,
                releases.as_mut_ptr(),
                size_of::<INPUT>() as i32,
            )
        };

        bail!(
            "SendInput accepted {sent}/{} events (GetLastError={error}); \
             cleanup accepted {released}/{} key releases",
            inputs.len(),
            releases.len(),
        );
    }

    Ok(())
}
