/*
 * Copyright 2022-2025 Jochen Kupperschmidt
 * License: MIT
 */

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Command-line arguments
#[derive(Parser, Debug)]
#[clap(about, author, version)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Detect button presses and display corresponding key code names
    IdentifyButtons {
        /// Specify button input device (e.g.
        /// `/dev/input/by-id/usb-RANDOM-GAMEPAD-event-joystick`)
        #[clap(short = 'd', long = "device")]
        device: String,
    },

    /// Register a Verbleiber client
    Register {
        /// Specify API hots
        #[clap(long = "base-url")]
        base_url: String,

        /// Supply number of buttons
        #[clap(long = "button-count")]
        button_count: u8,

        /// Specify if device has audio output
        #[clap(long = "audio-output")]
        audio_output: bool,

        /// Disable TLS verification
        #[clap(long = "no-tls-verify")]
        disable_tls_verification: bool,
    },

    /// Run the Verbleiber client
    Run {
        /// Specify configuration filename (e.g. `config.toml`)
        #[clap(short = 'c', long = "config")]
        config_filename: PathBuf,
    },
}

pub(crate) fn parse_cli() -> Cli {
    Cli::parse()
}
