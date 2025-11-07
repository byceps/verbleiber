/*
 * Copyright 2022-2025 Jochen Kupperschmidt
 * License: MIT
 */

use std::path::PathBuf;

use anyhow::Result;

use crate::api::ApiClient;
use crate::audio::AudioPlayer;
use crate::buttons::Button;
use crate::config::{ApiConfig, PartyConfig};
use crate::events::{Event, EventReceiver};
use crate::model::{CurrentUser, UserId, UserMode};
use crate::random::Random;

struct Client {
    audio_player: AudioPlayer,
    random: Random,
    api_client: ApiClient,
    party_config: PartyConfig,
    event_receiver: EventReceiver,
}

impl Client {
    fn new(
        sounds_path: PathBuf,
        api_config: &ApiConfig,
        party_config: PartyConfig,
        event_receiver: EventReceiver,
    ) -> Result<Self> {
        Ok(Self {
            audio_player: AudioPlayer::new(sounds_path)?,
            random: Random::new(),
            api_client: ApiClient::new(api_config, party_config.party_id.clone()),
            party_config,
            event_receiver,
        })
    }

    fn sign_on(&self) -> Result<()> {
        log::info!("Signing on ...");
        match self.api_client.sign_on() {
            Ok(()) => {
                log::info!("Signed on.");
                self.play_sound("signon_successful");
            }
            Err(e) => {
                log::warn!("Signing on failed.\n{e}");
                self.play_sound("signon_failed");
            }
        }
        Ok(())
    }

    fn sign_off(&self) -> Result<()> {
        log::info!("Signing off ...");
        match self.api_client.sign_off() {
            Ok(()) => {
                log::info!("Signed off.");
                self.play_sound("signoff_successful");
            }
            Err(e) => {
                log::warn!("Signing off failed.\n{e}");
                self.play_sound("signoff_failed");
            }
        }
        Ok(())
    }

    fn handle_tag_read(&self, tag: &str) -> Result<CurrentUser> {
        log::debug!("Requesting details for tag {} ...", tag);
        match self.api_client.get_tag_details(tag) {
            Ok(details) => match details {
                Some(details) => {
                    log::debug!(
                        "User for tag {}: {} (ID: {})",
                        details.identifier,
                        details.user.screen_name.unwrap_or("<nameless>".to_string()),
                        details.user.id
                    );
                    let user_id = details.user.id;

                    if let Some(name) = details.sound_name {
                        self.play_sound(&name);
                    }

                    log::debug!("Awaiting whereabouts for user {user_id} ...");

                    Ok(CurrentUser::User(user_id))
                }
                None => {
                    log::info!("Unknown user tag: {tag}");
                    self.play_sound("unknown_user_tag");

                    Ok(CurrentUser::None)
                }
            },
            Err(e) => {
                log::warn!("Requesting tag details failed.\n{e}");
                self.play_sound("communication_failed");

                Ok(CurrentUser::None)
            }
        }
    }

    fn handle_button_press_with_identified_user(
        &self,
        user_id: &UserId,
        button: Button,
    ) -> Result<()> {
        if let Some(whereabouts_name) = &self.party_config.buttons_to_whereabouts.get(&button) {
            log::debug!("Submitting whereabouts for user {user_id} -> {whereabouts_name} ...");

            let response = self.update_status(user_id, whereabouts_name);
            match response {
                Ok(_) => {
                    log::debug!("Status successfully updated.");

                    let sound_name =
                        match &self.party_config.whereabouts_sounds.get(*whereabouts_name) {
                            Some(sound_names) => &self.random.choose_random_element(sound_names),
                            None => "status_changed",
                        };
                    self.play_sound(sound_name);
                }
                Err(e) => {
                    log::warn!("Status update failed.\n{e}");
                    self.play_sound("communication_failed");
                }
            }
        }
        Ok(())
    }

    fn shutdown(&self) -> Result<()> {
        log::info!("Shutdown requested.");
        self.sign_off()?;
        log::info!("Shutting down ...");
        Ok(())
    }

    fn update_status(&self, user_id: &UserId, whereabouts_name: &str) -> Result<()> {
        self.api_client.update_status(user_id, whereabouts_name)
    }

    fn play_sound(&self, name: &str) {
        if let Err(e) = self.audio_player.play(name) {
            log::warn!("Could not play sound: {e}");
        }
    }
}

struct SingleUserClient {
    client: Client,
    user_id: UserId,
}

impl SingleUserClient {
    fn new(client: Client, user_id: UserId) -> Result<Self> {
        Ok(Self { client, user_id })
    }

    fn run(&self) -> Result<()> {
        self.client.sign_on()?;

        self.handle_events(&self.user_id)?;

        Ok(())
    }

    fn handle_events(&self, user_id: &UserId) -> Result<()> {
        for msg in self.client.event_receiver.iter() {
            match msg {
                Event::TagRead { .. } => {
                    log::error!("Unexpected tag read event received.");
                }
                Event::ButtonPressed { button } => {
                    log::debug!("Button pressed: {:?}", button);

                    self.client
                        .handle_button_press_with_identified_user(user_id, button)?;
                }
                Event::ShutdownRequested => {
                    self.client.shutdown()?;
                    break;
                }
            }
        }

        Ok(())
    }
}

struct MultiUserClient {
    client: Client,
}

impl MultiUserClient {
    fn new(client: Client) -> Result<Self> {
        Ok(Self { client })
    }

    fn run(&self) -> Result<()> {
        self.client.sign_on()?;

        self.handle_events()?;

        Ok(())
    }

    fn handle_events(&self) -> Result<()> {
        let mut current_user = CurrentUser::None;

        for msg in self.client.event_receiver.iter() {
            match msg {
                Event::TagRead { tag } => {
                    log::debug!("Tag read: {tag}");
                    current_user = self.client.handle_tag_read(&tag)?;
                }
                Event::ButtonPressed { button } => {
                    log::debug!("Button pressed: {:?}", button);

                    // Submit if user has identified; ignore if no user has
                    // been specified.
                    if let CurrentUser::User(user_id) = current_user {
                        self.client
                            .handle_button_press_with_identified_user(&user_id, button)?;
                        current_user = CurrentUser::None; // reset
                    }
                }
                Event::ShutdownRequested => {
                    self.client.shutdown()?;
                    break;
                }
            }
        }

        Ok(())
    }
}

pub fn run_client(
    sounds_path: PathBuf,
    api_config: &ApiConfig,
    party_config: PartyConfig,
    event_receiver: EventReceiver,
    user_mode: &UserMode,
) -> Result<()> {
    let client = Client::new(sounds_path, api_config, party_config, event_receiver)?;

    match user_mode {
        UserMode::SingleUser(user_id) => SingleUserClient::new(client, user_id.clone())?.run(),
        UserMode::MultiUser => MultiUserClient::new(client)?.run(),
    }
}
