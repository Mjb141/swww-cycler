# SWWW Cycler

`swww-cycler` is a small binary for Hyprland users that will change your background to a random background (from a directory provided) when you move to a new workspace.

## Installation

1) Download the latest release from [releases](https://github.com/Mjb141/swww-cycler/releases/latest)
2) Extract the binary to a location on your PATH

I intend to add AUR installation in the future.

## Running

### Testing the application

You can test the application by running `swww-cycler --backgrounds-path <path/to/backgrounds/dir>` and changing workspaces. Your background will change (change animation is controlled by your `hyprland.conf`). `swww-cycler` will log basic info about the background image change.

### Running on Hyprland

You should place `exec-once = swww-cycler --backgrounds-path <path/to/backgrounds/dir>` **after** `exec-once = swww init` in your `.config/hypr/hyprland.conf`.

E.g.
```
exec-once = swww init
exec-once = swww-cycler --backgrounds-path <path/to/backgrounds/dir>
```

## Configuration

`swww-cycler` provides a single configuration environment variable:

* `SWWW_CYCLER_LOG_LEVEL` which should be set to one of `info`, `debug`, or `error`
  * Defaults to `info`, which will only output a single log line per workspace/background change

