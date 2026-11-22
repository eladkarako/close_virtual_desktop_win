#![cfg(windows)]

mod bindings;

use anyhow::{ensure, Context, Result};
use bindings::{
    keys::{
        key_event, send_inputs, VK_CONTROL, VK_F4, VK_LWIN,
    },
    registry::RegistryKey,
};
use clap::Parser;
use std::{
    thread,
    time::Duration,
};

const GUID_SIZE: usize = 16;
const FALLBACK_ATTEMPTS: usize = 32;

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Close virtual desktops using Win+Ctrl+F4."
)]
struct Args {
    /// Delay between shortcut attempts, in milliseconds (10 minimum).
    #[arg(long, default_value_t = 100, value_parser = clap::value_parser!(u64).range(10..)
    )]
    sleep_timeout: u64,

    /// Send the keys this many times without checking the registry for how much actual desktops are used (range 1-256, no default).
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..=256)
    )]
    force_number: Option<u16>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    run(args).context("virtual-desktop cleanup failed")
}

fn run(args: Args) -> Result<()> {
    let attempts = match args.force_number {
        Some(number) => usize::from(number),
        None => match attempts_from_registry() {
            Ok(attempts) => attempts,
            Err(error) => {
                eprintln!(
                    "Could not determine the desktop count from the registry: \
                     {error:#}. Falling back to {FALLBACK_ATTEMPTS} shortcut attempts."
                );
                FALLBACK_ATTEMPTS
            }
        },
    };

    let delay = Duration::from_millis(args.sleep_timeout);

    for attempt in 1..=attempts {
        send_close_desktop_shortcut()
            .with_context(|| format!("failed on shortcut attempt {attempt} of {attempts}"))?;

        if attempt < attempts && !delay.is_zero() {
            thread::sleep(delay);
        }
    }

    Ok(())
}

/// Returns the number of shortcut attempts needed to leave one desktop.
///
/// The registry value contains desktop IDs, so the number of attempts is the
/// number of IDs minus one. An absent key, absent value, or malformed value is
/// an error; the caller handles it by using the fallback count.
fn attempts_from_registry() -> Result<usize> {
    let key = RegistryKey::open_current_user(
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\VirtualDesktops",
    )
        .context("could not open the current user's virtual-desktop registry key")?
        .ok_or_else(|| anyhow::anyhow!("the virtual-desktop registry key does not exist"))?;

    let ids = key
        .read_binary("VirtualDesktopIDs")
        .context("could not read the VirtualDesktopIDs registry value")?;

    ensure!(
        !ids.is_empty(),
        "the VirtualDesktopIDs registry value is missing or empty"
    );
    ensure!(
        ids.len() % GUID_SIZE == 0,
        "VirtualDesktopIDs has an invalid length of {} bytes",
        ids.len()
    );

    let desktop_count = ids.len() / GUID_SIZE;
    ensure!(desktop_count > 0, "the registry reported no virtual desktops");

    Ok(desktop_count.saturating_sub(1))
}

fn send_close_desktop_shortcut() -> Result<()> {
    let chord = [
        key_event(VK_LWIN, false),
        key_event(VK_CONTROL, false),
        key_event(VK_F4, false),
        key_event(VK_F4, true),
        key_event(VK_CONTROL, true),
        key_event(VK_LWIN, true),
    ];

    if let Err(send_error) = send_inputs(&chord) {
        let release_result = release_held_keys();

        return match release_result {
            Ok(()) => Err(send_error)
                .context("SendInput failed; attempted to release all shortcut keys"),
            Err(release_error) => Err(send_error).context(format!(
                "SendInput failed, and releasing shortcut keys also failed: \
                 {release_error:#}"
            )),
        };
    }

    Ok(())
}

fn release_held_keys() -> Result<()> {
    let releases = [
        key_event(VK_F4, true),
        key_event(VK_CONTROL, true),
        key_event(VK_LWIN, true),
    ];

    send_inputs(&releases).context("could not release one or more shortcut keys")
}
