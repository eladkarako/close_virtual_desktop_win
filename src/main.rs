#![cfg(windows)]

mod bindings;

use anyhow::{ensure, Context, Result};
use bindings::{
    keys::{key_event, send_inputs, VK_CONTROL, VK_F4, VK_LWIN},
    registry::RegistryKey,
};
use clap::Parser;
use std::{thread, time::Duration};

const GUID_SIZE: usize = 16;
const FALLBACK_ATTEMPTS: usize = 32;
const MIN_SLEEP_TIMEOUT_MS: u64 = 10;

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
#[command(
    after_help = concat!("Repository:    ", env!("CARGO_PKG_REPOSITORY"), "\n", "Homepage:      ", env!("CARGO_PKG_HOMEPAGE"))
)]
struct Args {
    /// Delay between shortcut attempts, in milliseconds (minimum: 10).
    #[arg(long, default_value_t = 100)]
    sleep_timeout: u64,

    /// Send the shortcut this many times without checking the registry.
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..=256)
    )]
    force_number: Option<u16>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    thread::Builder::new()
        .name("virtual-desktop-cleanup".to_owned())
        .spawn(move || run(args))
        .context("could not start the cleanup thread")?
        .join()
        .map_err(|_| anyhow::anyhow!("cleanup thread panicked"))?
        .context("virtual-desktop cleanup failed")
}

fn run(args: Args) -> Result<()> {
    let effective_timeout = args
        .sleep_timeout
        .max(MIN_SLEEP_TIMEOUT_MS);

    let (desktop_count, attempts, attempt_source) = match args
        .force_number
    {
        Some(number) => (None, usize::from(number), "force-number"),
        None => match desktop_count_from_registry() {
            Ok(count) => {
                (Some(count), count.saturating_sub(1), "registry")
            }
            Err(error) => {
                eprintln!(
                    "Could not determine the desktop count from the registry: \
                     {error:#}. Falling back to {FALLBACK_ATTEMPTS} shortcut attempts."
                );
                (None, FALLBACK_ATTEMPTS, "fallback")
            }
        },
    };

    println!("Arguments: {args:?}");
    match desktop_count {
        Some(count) => println!("Desktop count: {count}"),
        None => println!("Desktop count: unavailable"),
    }
    println!("Attempt source: {attempt_source}");
    println!("Shortcut attempts: {attempts}");
    println!(
        "Sleep timeout: requested {} ms, effective {} ms",
        args.sleep_timeout, effective_timeout
    );

    let delay = Duration::from_millis(effective_timeout);

    for attempt in 1..=attempts {
        println!("Sending shortcut {attempt}/{attempts}");

        send_close_desktop_shortcut().with_context(|| {
            format!(
                "failed on shortcut attempt {attempt} of {attempts}"
            )
        })?;

        if attempt < attempts {
            thread::sleep(delay);
        }
    }

    Ok(())
}

fn desktop_count_from_registry() -> Result<usize> {
    let key = RegistryKey::open_current_user(
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\VirtualDesktops",
    )
        .context("could not open the current user's virtual-desktop registry key")?
        .ok_or_else(|| anyhow::anyhow!("the virtual-desktop registry key does not exist"))?;

    let ids = key
        .read_binary("VirtualDesktopIDs")
        .context(
            "could not read the VirtualDesktopIDs registry value",
        )?;

    ensure!(
        !ids.is_empty(),
        "the VirtualDesktopIDs registry value is missing or empty"
    );
    ensure!(
        ids.len() % GUID_SIZE == 0,
        "VirtualDesktopIDs has an invalid length of {} bytes",
        ids.len()
    );

    let count = ids.len() / GUID_SIZE;
    ensure!(count > 0, "the registry reported no virtual desktops");

    Ok(count)
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
        return match release_held_keys() {
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

    send_inputs(&releases)
        .context("could not release one or more shortcut keys")
}
