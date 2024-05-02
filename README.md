# Sekiro Practice Tool

[![build](https://github.com/veeenu/sekiro-practice-tool/actions/workflows/build.yml/badge.svg)](https://github.com/veeenu/sekiro-practice-tool/actions)
[![GitHub all releases](https://img.shields.io/github/downloads/veeenu/sekiro-practice-tool/total)](https://github.com/veeenu/sekiro-practice-tool/releases/latest)
[![GitHub](https://img.shields.io/github/license/veeenu/sekiro-practice-tool)](https://github.com/veeenu/sekiro-practice-tool/blob/master/LICENSE) 
[![Discord](https://img.shields.io/discord/267623298647457802)](https://discord.gg/CVHbN7eF)
[![Twitch](https://img.shields.io/twitch/status/johndisandonato?style=social)](https://twitch.tv/johndisandonato)
[![Patreon](https://img.shields.io/badge/Support_me-Patreon-orange)](https://www.patreon.com/johndisandonato)

A tool for practicing speedruns. It is compatible with all Sekiro patches.

Made with ❤️ by [johndisandonato](https://twitch.tv/johndisandonato).

The tool is free, and will always be free for everyone. If you enjoy it, please consider 
[supporting me](https://www.patreon.com/johndisandonato)!

![Screenshot](lib/data/screenshot.jpg)

## Getting started

Download the **latest stable release** [here](https://github.com/veeenu/sekiro-practice-tool/releases/latest).

Prerequisites:

- Steam must be open. Offline mode is fine, but the program must be started.
- Antiviruses are disabled. This includes Windows Defender. If you don't want to do that, make sure to whitelist the contents of the practice tool in your antivirus.
- You have a legitimate copy of the game. Pirated copies will never be supported.

## Running the tool

### Standalone

- Extract all files from the zip archive. Anywhere will do.
- Start Sekiro.
- Double-click `jdsd_sekiro_practice_tool.exe`.

The tool will automatically appear over the game. Press `0` to open and close its interface.

### Installed

- Extract all files from the zip archive.
- Rename `jdsd_sekiro_practice_tool.dll` to `dinput8.dll`. Make sure your [file extensions are visible](https://www.howtogeek.com/205086/beginner-how-to-make-windows-show-file-extensions/)
  to ensure you are naming the file correctly.
- Copy `dinput8.dll` and `jdsd_sekiro_practice_tool.toml` to you Sekiro game folder.
  The files must be in the same folder as `sekiro.exe`.
- Start Sekiro normally.

The tool is now installed. To load it, start the game, press the right shift button and 
keep it pressed for a few seconds until the tool appears on screen.

If you don't do that, the tool won't load and the game will start normally.

## Running the tool on Linux

The tool fully supports Linux and should run on Steam Deck seamlessly.

### Standalone

If you want to run the tool in a standalone fashion, I recommend [protontricks](https://github.com/Matoking/protontricks):

```sh
protontricks-launch --appid 814380 jdsd_sekiro_practice_tool.exe
```

### Installed

Follow the same instructions as above. Additionally, you have to set the launch options in Steam as follows:

```sh
WINEDLLOVERRIDES="dinput8=n,b" %command%
```

## Help

If the tool doesn't work, you need help, or want to get in touch, read the [troubleshooting guide](TROUBLESHOOTING.md).

If you are looking to submit a patch, check the [contributing guide](CONTRIBUTING.md).

# Development

You will need:

- A recent [Rust nightly](https://rustup.rs/)
- The [MSVC toolchain](https://visualstudio.microsoft.com/vs/features/cplusplus/)

Most building functions are exposed by the [xtasks](https://github.com/matklad/cargo-xtask).

## Run the tool

```
cargo xtask run
```

This task will compile and run the practice tool from the repo.

## Distribution artifacts

```
cargo xtask dist
```

This task will create release artifacts in `target/dist/jdsd_sekiro_practice_tool.zip`.

## Code generation

```
cargo xtask codegen
```

This task is responsible for generating Rust code from various external sources.
Examples: params from [Paramdex](https://github.com/soulsmods/Paramdex), base pointers for
array-of-byte scans from the Sekiro executables.

## Environment

Some tasks require you to have environment variables defined that are dependent on your system.
You can put all your task-specific environment variables in a `.env` file in the top level directory
of the project. Complete example:

```
$ cat .env
ERPT_PATCHES_PATH="C:/Videogames/SekiroPatches"
```

