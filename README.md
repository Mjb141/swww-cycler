# SWWW Cycler

`swww-cycler` is a small binary for Hyprland users that will change your background to a random background (from a directory provided) when you move to a new workspace.

## Installation

1) Download the latest release from [releases](https://github.com/Mjb141/swww-cycler/releases/latest)
2) Extract the binary to a location on your PATH

I may add an AUR package in the future

## Running

1) Add the following to your `.config/hypr/hyprland.conf`: 

```exec = /usr/bin/swww-cycler --backgrounds-path <path/to/backgrounds/dir> &```

**Notice the `&` at the end. You must include this to run the binary in the background.**


2) You can test the application by running `swww-cycler --backgrounds-path <path/to/backgrounds/dir>` and changing workspaces. Your background will change (change animation is controlled by your `hyprland.conf`). `swww-cycler` will log basic info about the background image change.

## Configuration

`swww-cycler` provides a single configuration environment variable:

* `SWWW_CYCLER_LOG_LEVEL` which should be set to one of `debug`, or `error`
  * Defaults to `info`, which will prevent any log output.

