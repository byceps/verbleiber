# Verbleiber


## The System

The Verbleiber (German; roughly "Whereabouter") is a system for presence
tracking.

It consists of:
- hardware clients at different locations to sent presence information
- a central backend to accept, persist, and provide presence information
- a frontend to give an overview of the presences

Users can authenticate themselves via barcode or RFID transponder to a hardware
client configured for and placed at a specific location and set their new
status:

- When they are arriving, they "check in" to that location.
- When they are leaving, they move to the "travelling" status.
- Just before they go to sleep, they can set that as their new status.

Multiple locations would have such devices set up, so when a person arrives at
another location, they can "check in" there, changing their status from
"travelling" to being at the new location.


## This Application

This application is a software implementation for such a hardware client in
[Rust](https://www.rust-lang.org/).

To save time, the first set of clients was assembled from USB devices (cheap
RFID reader, cheap gamepad with arcade buttons soldered to it) which are then
connected to small form factor computers.

For future device generations, tiny computers (like Raspberry Pis) or even
custom-built hardware would be a nice.

For an implementation of a backend and overview frontend, which are not covered
here, check out the [Whereabouts
extension](https://github.com/lanresort/byceps-whereabouts) for the
[BYCEPS](https://byceps.nwsnet.de/) LAN party platform. Meanwhile, the
extension has been merged into BYCEPS and is available out-of-the-box.


## Usage

Create a configuration file based on the provided example and adjust as
necessary.

`reader_input_device` should reference a barcode/RFID reader,
`button_input_device` a push button input device (e.g. a gamepad). It might be
helpful to address the devices by their ID (if your system provides such
symlinks in `/dev/input/by-id`) so they are independent of the varying event
device number they get assigned depending on the order they are connected to
the host and other factors.

To register a new client, send a registration request to the API:

```sh
$ verbleiber register --base-url 'https://api.byceps.example/v1/whereabouts' --audio-output --button-count 3 --no-tls-verify
```

Then check BYCEPS' admin frontend for a new client candidate and approve it.

After a few seconds, Verbleiber should then print the client token. Add that to
your configuration file.

Now start the application using the `run` subcommand and specifying a
configuration file (via option `-c`/`--config`):

```sh
$ verbleiber run -c config.toml
```


## Sound Formats

Ogg Vorbis is supported out of the box. However, the employed audio playback
library ([rodio](https://github.com/RustAudio/rodio)) also supports MP3, FLAC,
WAV, MP4 and AAC, but those have to be enabled as features in `Cargo.toml` and
require recompilation of the program.


## History

Verbleiber has been originally built for and introduced on
[LANresort](https://www.lanresort.de/) 2022. It has since been used on
subsequent events as well as on [NorthCon](https://www.northcon.de/).


## License

Verbleiber is licensed under the MIT license.


## Author

Verbleiber was created by Jochen Kupperschmidt.
