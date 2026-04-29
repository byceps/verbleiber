/*
 * Copyright 2022-2026 Jochen Kupperschmidt
 * License: MIT
 */

use std::thread::sleep;
use std::time::Duration;

use anyhow::{Result, anyhow, bail};
use serde::{Deserialize, Serialize};
use ureq::{Agent, Error};

use crate::http::build_agent;

pub(crate) fn register(
    base_url: &str,
    button_count: u8,
    audio_output: bool,
    disable_tls_verification: bool,
) -> Result<()> {
    let api_client = ClientRegistrationApiClient::new(base_url, disable_tls_verification);

    let registration_response = api_client.register(button_count, audio_output)?;
    let sleep_duration = Duration::from_secs(10);

    loop {
        let status_response =
            api_client.get_registration_status(&registration_response.client_id)?;

        match status_response.status {
            ClientRegistrationStatus::Pending => {
                log::info!(
                    "Sleeping {:?} before retrying to fetch client registration status.",
                    sleep_duration
                );
                sleep(sleep_duration);
                continue;
            }
            ClientRegistrationStatus::Approved => {
                log::info!(
                    "Client registration was approved! Put this client token into your configuration file: {}",
                    registration_response.token
                );
                break;
            }
            ClientRegistrationStatus::Rejected => {
                bail!("Client registration was rejected.");
            }
        }
    }

    Ok(())
}

#[derive(Debug, Serialize)]
pub(crate) struct ClientRegistrationRequest {
    pub button_count: u8,
    pub audio_output: bool,
}

#[derive(Debug, Deserialize)]
struct ClientRegistrationResponse {
    pub client_id: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ClientRegistrationStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Deserialize)]
struct ClientRegistrationStatusResponse {
    pub status: ClientRegistrationStatus,
}

struct ClientRegistrationApiClient {
    pub base_url: String,
    agent: Agent,
}

impl ClientRegistrationApiClient {
    fn new(base_url: &str, disable_tls_verification: bool) -> Self {
        Self {
            base_url: base_url.to_owned(),
            agent: build_agent(Duration::from_secs(10), disable_tls_verification),
        }
    }

    fn register(&self, button_count: u8, audio_output: bool) -> Result<ClientRegistrationResponse> {
        let url = format!("{}/client/register", self.base_url);

        match self.agent.post(&url).send_json(ClientRegistrationRequest {
            button_count,
            audio_output,
        }) {
            Ok(mut response) => response
                .body_mut()
                .read_json::<ClientRegistrationResponse>()
                .map_err(|e| anyhow!("JSON error: {}", e)),
            Err(Error::StatusCode(code)) => Err(anyhow!("API error: {}", code)),
            Err(e) => Err(anyhow!("Network error: {}", e)),
        }
    }

    fn get_registration_status(&self, client_id: &str) -> Result<ClientRegistrationStatusResponse> {
        let url = format!("{}/client/registration_status/{}", self.base_url, client_id);

        match self.agent.get(&url).call() {
            Ok(mut response) => response
                .body_mut()
                .read_json::<ClientRegistrationStatusResponse>()
                .map_err(|e| anyhow!("JSON error: {}", e)),
            Err(Error::StatusCode(code)) => Err(anyhow!("API error: {}", code)),
            Err(e) => Err(anyhow!("Network error: {}", e)),
        }
    }
}
