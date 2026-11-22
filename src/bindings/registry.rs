#![cfg(windows)]

use anyhow::{ensure, Context, Result};
use std::{ffi::c_void, io, ptr};

pub type Hkey = *mut c_void;

const ERROR_SUCCESS: i32 = 0;
const ERROR_FILE_NOT_FOUND: i32 = 2;
const ERROR_MORE_DATA: i32 = 234;

const KEY_READ: u32 = 0x0002_0019;
const REG_BINARY: u32 = 3;

#[link(name = "advapi32")]
unsafe extern "system" {
    #[link_name = "RegOpenKeyExW"]
    fn reg_open_key_ex_w(
        h_key: Hkey,
        lp_sub_key: *const u16,
        ul_options: u32,
        sam_desired: u32,
        phk_result: *mut Hkey,
    ) -> i32;

    #[link_name = "RegQueryValueExW"]
    fn reg_query_value_ex_w(
        h_key: Hkey,
        lp_value_name: *const u16,
        lp_reserved: *mut u32,
        lp_type: *mut u32,
        lp_data: *mut u8,
        lp_cb_data: *mut u32,
    ) -> i32;

    #[link_name = "RegCloseKey"]
    fn reg_close_key(h_key: Hkey) -> i32;
}

fn hkey_current_user() -> Hkey {
    0x8000_0001_u32 as i32 as isize as Hkey
}

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16()
        .chain(std::iter::once(0))
        .collect()
}

pub struct RegistryKey(Hkey);

impl RegistryKey {
    /// Opens a key under HKEY_CURRENT_USER. Returns `None` if the key does not exist.
    pub fn open_current_user(sub_key: &str) -> Result<Option<Self>> {
        let sub_key = wide(sub_key);
        let mut handle: Hkey = ptr::null_mut();

        // SAFETY: `sub_key` is null-terminated and `handle` is a valid output
        // pointer. The returned handle is wrapped and closed by `Drop`.
        let status = unsafe {
            reg_open_key_ex_w(
                hkey_current_user(),
                sub_key.as_ptr(),
                0,
                KEY_READ,
                &mut handle,
            )
        };

        match status {
            ERROR_SUCCESS => {
                ensure!(
                    !handle.is_null(),
                    "registry returned a null key handle"
                );
                Ok(Some(Self(handle)))
            }
            ERROR_FILE_NOT_FOUND => Ok(None),
            code => Err(io::Error::from_raw_os_error(code))
                .context("RegOpenKeyExW failed"),
        }
    }

    /// Reads a REG_BINARY value. Returns an empty vector if the value does not exist.
    pub fn read_binary(
        &self,
        name: &str,
    ) -> Result<Vec<u8>> {
        let name = wide(name);
        let mut buffer = vec![0_u8; 1024];

        loop {
            let mut size = u32::try_from(buffer.len()).context(
                "registry buffer size exceeds the Windows API limit",
            )?;
            let mut kind = 0_u32;

            // SAFETY: `name` is null-terminated; `buffer` is writable for the
            // supplied size; and `self.0` is a live registry handle.
            let status = unsafe {
                reg_query_value_ex_w(
                    self.0,
                    name.as_ptr(),
                    ptr::null_mut(),
                    &mut kind,
                    buffer.as_mut_ptr(),
                    &mut size,
                )
            };

            match status {
                ERROR_SUCCESS => {
                    ensure!(
                        kind == REG_BINARY,
                        "registry value is not REG_BINARY (type {kind})"
                    );
                    ensure!(
                        size as usize <= buffer.len(),
                        "registry returned a size larger than the supplied buffer"
                    );

                    buffer.truncate(size as usize);
                    return Ok(buffer);
                }
                ERROR_MORE_DATA => {
                    ensure!(
                        size as usize > buffer.len(),
                        "registry requested a non-growing buffer after ERROR_MORE_DATA"
                    );
                    buffer.resize(size as usize, 0);
                }
                ERROR_FILE_NOT_FOUND => {
                    anyhow::bail!("registry value does not exist");
                }
                code => {
                    return Err(io::Error::from_raw_os_error(code))
                        .context("RegQueryValueExW failed");
                }
            }
        }
    }
}

impl Drop for RegistryKey {
    fn drop(&mut self) {
        if !self.0.is_null() {
            // SAFETY: This handle was successfully returned by RegOpenKeyExW
            // and is closed exactly once here.
            let _ = unsafe { reg_close_key(self.0) };
        }
    }
}
