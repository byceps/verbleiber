/*
 * Copyright 2022-2025 Jochen Kupperschmidt
 * License: MIT
 */

use std::time::Duration;

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use ureq::{Agent, Error};

use crate::config::ApiConfig;
use crate::http::build_agent;
use crate::model::{PartyId, Tag, UserId};

pub(crate) struct ApiClient {
    pub base_url: String,
    pub client_token: String,
    pub party_id: PartyId,
    agent: Agent,
}

#[derive(Debug, Serialize)]
pub(crate) struct StatusUpdate {
    pub user_id: UserId,
    pub party_id: String,
    pub whereabouts_name: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TagDetails {
    pub identifier: String,
    pub user: TagUser,
    pub sound_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TagUser {
    pub id: UserId,
    pub screen_name: Option<String>,
}

impl ApiClient {
    pub(crate) fn new(config: &ApiConfig, party_id: PartyId) -> Self {
        Self {
            base_url: config.base_url.to_owned(),
            client_token: config.client_token.to_owned(),
            party_id,
            agent: build_agent(
                Duration::from_secs(config.timeout_in_seconds),
                !config.tls_verify,
            ),
        }
    }

    pub(crate) fn sign_on(&self) -> Result<()> {
        let url = format!("{}/client/sign_on", self.base_url);

        match self
            .agent
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.client_token))
            .send_empty()
        {
            Ok(_) => Ok(()),
            Err(Error::StatusCode(code)) => Err(anyhow!("API error: {}", code)),
            Err(e) => Err(anyhow!("Network error: {}", e)),
        }
    }

    pub(crate) fn sign_off(&self) -> Result<()> {
        let url = format!("{}/client/sign_off", self.base_url);

        match self
            .agent
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.client_token))
            .send_empty()
        {
            Ok(_) => Ok(()),
            Err(Error::StatusCode(code)) => Err(anyhow!("API error: {}", code)),
            Err(e) => Err(anyhow!("Network error: {}", e)),
        }
    }

    pub(crate) fn get_tag_details(&self, tag: &Tag) -> Result<Option<TagDetails>> {
        let url = format!("{}/tags/{}", &self.base_url, tag.value);

        match self
            .agent
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.client_token))
            .call()
        {
            Ok(mut response) => response
                .body_mut()
                .read_json::<TagDetails>()
                .map_err(|e| anyhow!("JSON error: {}", e))
                .map(Some),
            Err(Error::StatusCode(404)) => Ok(None),
            Err(e) => Err(anyhow!("Network error: {}", e)),
        }
    }

    pub(crate) fn update_status(&self, user_id: &UserId, whereabouts_name: &str) -> Result<()> {
        let url = format!("{}/statuses", self.base_url);

        match self
            .agent
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.client_token))
            .send_json(StatusUpdate {
                user_id: user_id.to_string(),
                party_id: self.party_id.to_string(),
                whereabouts_name: whereabouts_name.to_string(),
            }) {
            Ok(_) => Ok(()),
            Err(Error::StatusCode(code)) => Err(anyhow!("API error: {}", code)),
            Err(e) => Err(anyhow!("Network error: {}", e)),
        }
    }
}
