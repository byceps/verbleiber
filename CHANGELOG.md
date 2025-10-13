# Changelog


## 0.9.0 (unreleased)

- Added subcommand `identify-buttons` to show key code and name for pressed
  buttons.

- Added support for mouse buttons.

- Removed redundant key names `north`, `west`, `south`, and `east` so that the
  key code/name mapping becomes a bijective/one-to-one mapping.

  Use key names `x`, `y`, `a`, and `b`, respectively, instead.

- Changed status update behavior to trigger on button press instead of on
  button release.

- Added fallback to play default sound when no specific sounds are configured
  for a whereabouts name.

- Updated clap to v4.5.49.

- Updated log to v0.4.28.

- Updated toml to v0.9.8.

- Updated ureq to v3.1.2.


## 0.8.1 (2025-10-09)

- Fixed reference in release workflow on GitHub Actions.


## 0.8.0 (2025-10-09)

- Introduced single-user mode. Works without tag reader.

- Introduced separate, English sound names for communication errors.

- Added sound triggers for successful client sign-on and sign-off.

- Changed behavior when trying to play a missing sound file from aborting the
  program to logging a warning.


## 0.7.0 (2025-09-03)

- Introduced subcommands. To run the application as usual, use `verbleiber run`
  instead of just `verbleiber`.

- Added subcommand `register` to register a new client with the API.

- Made button mapping configurable. Added new section `buttons_to_key_codes` to
  the configuration file format.

- Changed `Bearer` authorization header value from API token to client token
  for sign-on, sign-off, tag retrieval, and status update endpoints.

- Removed now unused API token from code and configuration.

- Added handler to shut down on Ctrl-C in a more controlled way.

- Added client sign-off on application shutdown.

- Adjusted to new whereabouts API response for tags which now includes the
  sound name without file extension.

- Remove file extension from both hard-coded and configured sound names. It
  will be added by the audio player.

- Fixed outdated property name in exemplary configuration file.

- Updated clap to v4.5.47.

- Updated nanorand to v0.8.0.

- Updated rodio to v0.21.1.

- Updated toml to v0.9.5.

- Updated ureq to v3.1.0.


## 0.6.1 (2025-05-02)

- Renamed configuration property `tls_verification` to `tls_verify`.

- Fixed unintentional inversion of TLS verification configuration flag.


## 0.6.0 (2025-05-02)

- Renamed configuration property `auth_token` to `api_token` in section `api`.

- Added configuration property `client_token` in section `api`.

- Added header `x-whereabouts-client-token` to status update requests to comply
  with the client-aware whereabouts API.

- Added client sign-on on application start.

- Allowed disabling of TLS verification (useful for self-signed certificates
  during development).

- Raised minimum supported Rust version to 1.85.0.

- Switched to Rust edition 2024.

- Updated clap to v4.5.37.

- Updated evdev to v0.13.1.

- Updated flume to v0.11.1.

- Updated log to v0.4.27.

- Updated rodio to v0.20.1.

- Updated toml to v0.8.22.

- Updated ureq to v3.0.11.


## 0.5.0 (2024-08-02)

- Moved key `party_id` and tables `buttons_to_whereabouts` and
  `whereabouts_sounds` in configuration file to table `party`.

- Changed way of passing user ID and party ID on status updates from URL
  parameters to JSON payload.

- Extended details logged on failed status update attempts.

- Raised minimum supported Rust version to 1.78.0.

- Updated clap to v4.5.11.

- Updated evdev to v0.12.2.

- Updated log to v0.4.22.

- Updated rodio to v0.19.0.

- Updated simple_logger to v5.0.0.

- Updated toml to v0.8.17.

- Updated ureq to v2.10.0.


## 0.4.0 (2023-10-12)

- Replaced full URL API config property `url` with new property `base_url`
  which leaves out the specific endpoint remainder (which is now hard-coded) to
  support more endpoints without more configuration.

- Changed flow to fetch user data for a tag from the new API endpoint instead
  of keeping local mapping. This obsoletes the configuration table
  `tags_to_user_ids`.

- Obtain filename of sound to play for a user from tag details returned by the
  API. This obsoletes the configuration table `user_sounds`.

- Included party ID in configuration as it became a mandatory parameter for the
  endpoint which sets the status.

- Switched button mappings to match what some sources report as usual gamepad
  button order.

- Replaced whereabouts IDs with human-readable whereabouts names in
  configuration and API protocol.

- Added proper logging.

- Updated clap to v4.4.6.

- Updated flume to v0.11.0.

- Updated ureq to v2.8.0.

- Updated toml to v0.8.2.


## 0.3.0 (2023-05-05)

- Fixed audio playback.


## 0.2.0 (2023-05-05)

- Turned device command-line arguments into configuration properties.

- Grouped API-related configuration properties.

- Show clarifying error message and exit cleanly if input devices could not be
  opened.

- Play sound if user tag is unknown.


## 0.1.0 (2023-05-01)

- First public release
