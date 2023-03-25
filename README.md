# Huyer: Your personal annoying notifier

Huyer is an alarm clock application based on process `sleep`, designed for setting up adhoc reminders.

It's a simple toy app created on the spot to demonstrate development in Rust.

On nix based systems, `at` is a more robust alternative, but requires running its background deamon.
On Windows, at least Windows 10, the official app available from Microsoft Store had serious ergonomic
problems and privacy concerns, sparking my interest in a simple handmade alternative.

## Usage

Allows setting up single alarm clock either after given time period (in `123x` format, where `x` stands for letter for `s`econds, `m`inutes or `h`ours) or
at a specific time (specified as `hours:minutes` of current day). You can optionally also set a text
of the reminder.

If you want to set up multiple alarms, just run the app multiple times.

For command line arguments, run the `huyer` binary with `--help`.

The application can work both as console and graphical application, depending on chosen feature flags:

- `gui` (default): uses `egui` for notification window and possibly also to interactivaly sets the options.
  If unset, application prints into console and would try to get your attention with terminal bell.
- `pure-gui`: doesn't create console window, like full GUI apps. This distinction is important on Windows. Note that with this option command line arguments still work, but app won't guarantee any feedback, notably to `--help`.

## Trivia

Named after Hujer, famous groveler and sycophant from Czech comedy movie _Marečku, podejte mi pero_.
