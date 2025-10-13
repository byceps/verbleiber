/*
 * Copyright 2022-2025 Jochen Kupperschmidt
 * License: MIT
 */

use anyhow::{Result, anyhow};
use evdev::Device;

pub(crate) type DeviceName = String;

pub(crate) fn open_input_device(device_name: DeviceName, label: String) -> Result<Device> {
    Device::open(device_name)
        .map_err(|e| anyhow!("Could not open {}: {}", label, e))
        .and_then(|mut device| {
            log::info!(
                "Opened {} \"{}\".",
                label,
                device.name().unwrap_or("unnamed device")
            );

            grab_input_device(&mut device, label)?;

            Ok(device)
        })
}

fn grab_input_device(device: &mut Device, label: String) -> Result<()> {
    device
        .grab()
        .map_err(|e| anyhow!("Could not get exclusive access to {}: {}", label, e))
        .map(|()| {
            log::info!("Successfully obtained exclusive access to {}.", label);
        })
}
