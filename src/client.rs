/*
 * Copyright 2022-2025 Jochen Kupperschmidt
 * License: MIT
 */

use std::path::PathBuf;

use anyhow::Result;

use crate::api::ApiClient;
use crate::audio::{AudioPlayer, Sound};
use crate::buttons::Button;
use crate::config::{ApiConfig, PartyConfig};
use crate::events::{Event, EventReceiver};
use crate::model::{CurrentUser, Tag, UserId, UserMode};
use crate::random::Random;

enum EventHandlingResult {
    KeepCurrentUser,
    SetCurrentUser(CurrentUser),
    ResetCurrentUser,
    Abort,
}

pub(crate) struct Client {
    audio_player: AudioPlayer,
    random: Random,
    user_mode: UserMode,
    api_client: ApiClient,
    party_config: PartyConfig,
    event_receiver: EventReceiver,
}

impl Client {
    pub(crate) fn new(
        sounds_path: PathBuf,
        user_mode: UserMode,
        api_config: &ApiConfig,
        party_config: PartyConfig,
        event_receiver: EventReceiver,
    ) -> Result<Self> {
        Ok(Self {
            audio_player: AudioPlayer::new(sounds_path)?,
            random: Random::new(),
            user_mode,
            api_client: ApiClient::new(api_config, party_config.party_id.clone()),
            party_config,
            event_receiver,
        })
    }

    pub(crate) fn run(&self) -> Result<()> {
        self.sign_on()?;

        self.handle_events()?;

        Ok(())
    }

    fn handle_events(&self) -> Result<()> {
        let default_current_user = match self.user_mode {
            UserMode::SingleUser(ref user_id) => CurrentUser::User(user_id.clone()),
            UserMode::MultiUser => CurrentUser::None,
        };

        let mut current_user = default_current_user.clone();

        for event in self.event_receiver.iter() {
            let result = match self.user_mode {
                UserMode::SingleUser(ref user_id) => {
                    self.handle_single_user_event(event, user_id.clone())?
                }
                UserMode::MultiUser => self.handle_multi_user_event(event, &current_user)?,
            };
            current_user = match result {
                EventHandlingResult::KeepCurrentUser => current_user.clone(),
                EventHandlingResult::SetCurrentUser(new_current_user) => new_current_user.clone(),
                EventHandlingResult::ResetCurrentUser => default_current_user.clone(),
                EventHandlingResult::Abort => {
                    break;
                }
            }
        }

        Ok(())
    }

    fn handle_single_user_event(
        &self,
        event: Event,
        single_user_id: UserId,
    ) -> Result<EventHandlingResult> {
        Ok(match event {
            Event::TagRead { .. } => {
                log::error!("Unexpected tag read event received.");
                EventHandlingResult::ResetCurrentUser
            }
            Event::ButtonPressed { button } => {
                log::debug!("Button pressed: {:?}", button);
                self.handle_button_press_with_identified_user(&single_user_id, button)?;
                EventHandlingResult::ResetCurrentUser
            }
            Event::ShutdownRequested => {
                self.shutdown()?;
                EventHandlingResult::Abort
            }
        })
    }

    fn handle_multi_user_event(
        &self,
        event: Event,
        current_user: &CurrentUser,
    ) -> Result<EventHandlingResult> {
        Ok(match event {
            Event::TagRead { tag } => {
                log::debug!("Tag read: {}", tag.value);
                self.handle_tag_read(&tag)?
            }
            Event::ButtonPressed { button } => {
                log::debug!("Button pressed: {:?}", button);

                // Submit if user has identified; ignore if no user has
                // been specified.
                match current_user {
                    CurrentUser::User(user_id) => {
                        self.handle_button_press_with_identified_user(user_id, button)?;
                        EventHandlingResult::ResetCurrentUser
                    }
                    CurrentUser::None => EventHandlingResult::ResetCurrentUser,
                }
            }
            Event::ShutdownRequested => {
                self.shutdown()?;
                EventHandlingResult::Abort
            }
        })
    }

    fn sign_on(&self) -> Result<()> {
        log::info!("Signing on ...");
        match self.api_client.sign_on() {
            Ok(()) => {
                log::info!("Signed on.");
                self.play_sound(Sound::SignOnSuccessful);
            }
            Err(e) => {
                log::warn!("Signing on failed.\n{e}");
                self.play_sound(Sound::SignOnFailed);
            }
        }
        Ok(())
    }

    fn sign_off(&self) -> Result<()> {
        log::info!("Signing off ...");
        match self.api_client.sign_off() {
            Ok(()) => {
                log::info!("Signed off.");
                self.play_sound(Sound::SignOffSuccessful);
            }
            Err(e) => {
                log::warn!("Signing off failed.\n{e}");
                self.play_sound(Sound::SignOffFailed);
            }
        }
        Ok(())
    }

    fn handle_tag_read(&self, tag: &Tag) -> Result<EventHandlingResult> {
        log::debug!("Requesting details for tag {} ...", tag.value);
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
                        self.play_sound(Sound::UserTagCustomGreeting(name));
                    }

                    log::debug!("Awaiting whereabouts for user {user_id} ...");

                    Ok(EventHandlingResult::SetCurrentUser(CurrentUser::User(
                        user_id,
                    )))
                }
                None => {
                    log::info!("Unknown user tag: {}", tag.value);
                    self.play_sound(Sound::UserTagUnknown);

                    Ok(EventHandlingResult::ResetCurrentUser)
                }
            },
            Err(e) => {
                log::warn!("Requesting tag details failed.\n{e}");
                self.play_sound(Sound::CommunicationFailed);

                Ok(EventHandlingResult::ResetCurrentUser)
            }
        }
    }

    fn handle_button_press_with_identified_user(
        &self,
        user_id: &UserId,
        button: Button,
    ) -> Result<()> {
        if let Some(whereabouts_name) = &self.party_config.buttons_to_whereabouts.get(&button) {
            log::debug!("Updating whereabouts status for user {user_id} -> {whereabouts_name} ...");

            let response = self.update_status(user_id, whereabouts_name);
            match response {
                Ok(_) => {
                    log::debug!("Whereabouts status successfully updated.");

                    let sound = self
                        .party_config
                        .whereabouts_sounds
                        .get(*whereabouts_name)
                        .map(|sound_names| {
                            self.random.choose_random_element(sound_names).to_owned()
                        })
                        .map(Sound::WhereaboutsStatusUpdatedCustom)
                        .unwrap_or(Sound::WhereaboutsStatusUpdated);
                    self.play_sound(sound);
                }
                Err(e) => {
                    log::warn!("Whereabouts status update failed.\n{e}");
                    self.play_sound(Sound::CommunicationFailed);
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

    fn play_sound(&self, sound: Sound) {
        let name = sound.get_name();
        if let Err(e) = self.audio_player.play(&name) {
            log::warn!("Could not play sound: {e}");
        }
    }
}
