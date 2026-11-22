#![cfg(windows)]

use anyhow::{ensure, Context, Result};
use std::mem::size_of;

pub const INPUT_KEYBOARD: u32 = 1;

pub const KEYEVENTF_KEYUP: u32 = 0x0002;

pub const VK_CONTROL: u16 = 0x11;

pub const VK_F4: u16 = 0x73;

pub const VK_LWIN: u16 = 0x5B;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MouseInput {
    pub dx: i32,
    pub dy: i32,
    pub mouse_data: u32,
    pub dw_flags: u32,
    pub time: u32,
    pub dw_extra_info: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KeybdInput {
    pub w_vk: u16,
    pub w_scan: u16,
    pub dw_flags: u32,
    pub time: u32,
    pub dw_extra_info: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HardwareInput {
    pub u_msg: u32,
    pub w_param_l: u16,
    pub w_param_h: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union InputUnion {
    pub mi: MouseInput,
    pub ki: KeybdInput,
    pub hi: HardwareInput,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Input {
    pub r#type: u32,
    pub data: InputUnion,
}

// INPUT is 40 bytes on 64-bit Windows and 28 bytes on 32-bit Windows.
const _: () = assert!(
    size_of::<Input>()
        == if cfg!(target_pointer_width = "64") {
        40
    } else {
        28
    }
);

#[link(name = "user32")]
unsafe extern "system" {
    #[link_name = "SendInput"]
    fn send_input(
        c_inputs: u32,
        p_inputs: *const Input,
        cb_size: i32,
    ) -> u32;
}

/// Builds a keyboard event for a virtual key, either press or release.
pub fn key_event(
    vk: u16,
    released: bool,
) -> Input {
    let dw_flags = if released { KEYEVENTF_KEYUP } else { 0 };

    Input {
        r#type: INPUT_KEYBOARD,
        data: InputUnion {
            ki: KeybdInput {
                w_vk: vk,
                w_scan: 0,
                dw_flags,
                time: 0,
                dw_extra_info: 0,
            },
        },
    }
}

/// Sends all events in one `SendInput` call.
/// Returns an error if the OS accepted fewer events than were supplied.
pub fn send_inputs(inputs: &[Input]) -> Result<()> {
    ensure!(
        !inputs.is_empty(),
        "refusing to send an empty input batch"
    );

    let count = u32::try_from(inputs.len())
        .context("input batch is too large for SendInput")?;
    let input_size = i32::try_from(size_of::<Input>()).context(
        "INPUT structure size does not fit in SendInput's parameter",
    )?;

    // SAFETY: `inputs` points to `count` initialized INPUT-compatible values,
    // and `input_size` is the size of the Rust representation above.
    let sent =
        unsafe { send_input(count, inputs.as_ptr(), input_size) };

    ensure!(
        sent == count,
        "SendInput accepted {sent} of {count} events: {}",
        std::io::Error::last_os_error()
    );

    Ok(())
}
