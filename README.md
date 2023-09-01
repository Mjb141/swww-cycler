# SWWW Cycler

`swww-cycler` is a small binary for Hyprland users that will change your background to a random background (from a directory provided) when you move to a new workspace.

## Installation

1) Download the latest release from [releases](https://github.com/Mjb141/swww-cycler/releases/latest)
2) Extract the binary to a location on your PATH

I may add an AUR package in the future

## Running

### Parameters

* `--backgrounds-path` (Required): Path to a directory containing background images
* `--minutes` (Optional, defaults to 5): Integer minimum number of minutes between background change

1) Add (at least) the following to your `.config/hypr/hyprland.conf`: 

```exec = swww-cycler --backgrounds-path path/to/backgrounds/dir &```

* **Notice the `&` at the end. You must include this to run the binary in the background.**
* In some cases you may need to include the full path to the binary
  * E.g. `exec = /path/to/swww-cycler --backgrounds-path path/to/backgrounds/dir`

2) You may optionally add `--minutes N`:

```exec = swww-cycler --backgrounds-path path/to/backgrounds/dir --minutes 10 &```

* This will limit the background changes to at most once every `N` minutes, no matter how many workspace change events are emitted

3) You can test the application by running `swww-cycler --backgrounds-path path/to/backgrounds/dir` (instead of adding it directly to your `hyprland.conf` file) and changing workspaces. Your background will change (change animation is controlled by your `hyprland.conf`).
