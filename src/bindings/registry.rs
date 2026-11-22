#![cfg(windows)]

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

/// Owns an open registry key and closes it when dropped.
pub struct RegistryKey(Hkey);

impl RegistryKey {
    /// Opens a key under HKEY_CURRENT_USER. Returns `None` if the key does not exist.
    pub fn open_current_user(
        sub_key: &str
    ) -> io::Result<Option<Self>> {
        let sub_key = wide(sub_key);
        let mut handle: Hkey = ptr::null_mut();
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
            ERROR_SUCCESS => Ok(Some(Self(handle))),
            ERROR_FILE_NOT_FOUND => Ok(None),
            _ => Err(io::Error::from_raw_os_error(status)),
        }
    }

    /// Reads a REG_BINARY value. Returns an empty vector if the value does not exist.
    pub fn read_binary(
        &self,
        name: &str,
    ) -> io::Result<Vec<u8>> {
        let name = wide(name);
        let mut buffer = vec![0_u8; 1024];
        loop {
            let mut size = u32::try_from(buffer.len())
                .map_err(io::Error::other)?;
            let mut kind = 0_u32;
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
                    if kind != REG_BINARY {
                        return Err(io::Error::other(
                            "registry value is not REG_BINARY",
                        ));
                    }
                    buffer.truncate(size as usize);
                    return Ok(buffer);
                }
                ERROR_MORE_DATA => buffer.resize(size as usize, 0),
                ERROR_FILE_NOT_FOUND => return Ok(Vec::new()),
                _ => {
                    return Err(io::Error::from_raw_os_error(status));
                }
            }
        }
    }
}

impl Drop for RegistryKey {
    fn drop(&mut self) {
        unsafe { reg_close_key(self.0) };
    }
}
