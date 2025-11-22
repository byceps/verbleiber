/*
 * Copyright 2022-2025 Jochen Kupperschmidt
 * License: MIT
 */

use std::path::PathBuf;

use anyhow::{Result, bail};
use simple_logger::SimpleLogger;

mod api;
mod audio;
mod buttons;
mod cli;
mod client;
mod config;
mod devices;
mod events;
mod http;
mod keycodenames;
mod model;
mod random;
mod registration;
mod tagreader;

use crate::client::Client;
use crate::events::{EventReceiver, EventSender};
use crate::model::UserMode;

fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .with_module_level("verbleiber", log::LevelFilter::Debug)
        .init()?;

    let cli = cli::parse_cli();

    match cli.command {
        cli::Command::IdentifyButtons { device } => buttons::identify_buttons(device)?,
        cli::Command::Register {
            base_url,
            button_count,
            audio_output,
            disable_tls_verification,
        } => registration::register(
            &base_url,
            button_count,
            audio_output,
            disable_tls_verification,
        )?,
        cli::Command::Run { config_filename } => run(config_filename)?,
    }

    Ok(())
}

fn run(config_filename: PathBuf) -> Result<()> {
    let config = config::load_config(&config_filename)?;

    let admin_tags = config.get_admin_tags();

    let user_mode = config.get_user_mode();
    match user_mode {
        UserMode::SingleUser(ref id) => {
            log::info!("Running in single-user mode for user ID '{id}'.")
        }
        UserMode::MultiUser => log::info!("Running in multi-user mode."),
    }

    let sounds_path = config.sounds_path.clone();

    let (tx1, rx): (EventSender, EventReceiver) = events::create_event_channel();
    let tx2 = tx1.clone();
    let tx3 = tx1.clone();
    let tx4 = tx1.clone();

    ctrlc::set_handler(move || handle_ctrl_c(&tx1)).expect("Could not set Ctrl-C handler");

    if let UserMode::MultiUser = user_mode {
        match config.reader_input_device {
            Some(device_name) => tagreader::handle_tag_reads(device_name, tx2)?,
            None => bail!("No reader device configured, but one is required in multi-user mode."),
        }
    }

    buttons::handle_button_presses(
        config.button_input_device,
        config.buttons_to_key_code_names,
        tx3,
    )?;

    let client = Client::new(
        sounds_path,
        user_mode,
        admin_tags,
        &config.api,
        config.party,
        rx,
        tx4,
    )?;
    client.run()
}

fn handle_ctrl_c(event_sender: &EventSender) {
    event_sender
        .send_shutdown_requested()
        .expect("Could not send shutdown signal")
}
