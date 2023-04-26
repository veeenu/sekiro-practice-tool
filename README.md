# Sekiro Practice Tool

[![GitHub all releases](https://img.shields.io/github/downloads/veeenu/sekiro-practice-tool/total)](https://github.com/veeenu/sekiro-practice-tool/releases/latest)
[![GitHub](https://img.shields.io/github/license/veeenu/sekiro-practice-tool)](https://github.com/veeenu/sekiro-practice-tool/blob/main/LICENSE) 
[![Discord](https://img.shields.io/discord/267623298647457802)](https://discord.gg/CVHbN7eF)
[![Twitch](https://img.shields.io/twitch/status/johndisandonato?style=social)](https://twitch.tv/johndisandonato)

A tool for practicing speedruns. It is compatible with all Sekiro patches.

Made with ❤️ by [johndisandonato](https://twitch.tv/johndisandonato).

To run the tool, extract all files from the zip archive and double-click the
`.exe` file he tool will automatically appear over the game, and it can be
toggled by pressing `0`.

You can download the **latest stable release** [here](https://github.com/veeenu/sekiro-practice-tool/releases/latest).

If you need help, **please first read** the [Known Issues](#known-issues) and [FAQ](#troubleshooting--faq) sections for
solutions, or ways to get in touch.

# Troubleshooting / FAQ

## My game crashes

- Always start with a clean zip of the latest release.
- Wait for the main menu of the game to appear before launching the tool.
- If you are running in [fullscreen](https://github.com/veeenu/eldenring-practice-tool/issues/23), try borderless or windowed mode.
- Make sure you have the latest version of your GPU drivers.
- Antivirus software and old Windows versions will interact poorly with the tool, as it
  employs some techniques that are usually typical of malware. Don't worry, the tool is
  safe! The source code is fully available and auditable in this repository.
- If all else fails, [submit an issue](#i-found-an-issue-what-do-i-do).

## "Inaccessible target process", "Could not find process"

- You have not closed your antivirus. Close it.

## I found an issue. What do I do?

- Apply the following settings to `jdsd_sekiro_practice_tool.toml`:
  - `log_level = "TRACE"` 
  - `dxgi_debug = true`
- Reproduce the steps tha cause your bug.
- Go [here](https://github.com/veeenu/sekiro-practice-tool/issues/new) and submit a new issue:
  explain the problem, compress the `jdsd_sekiro_practice_tool.log` file, and attach it.

I'll do my best to get back to you and fix the bug.

While troubleshooting bugs, I may ask you to use the [nightly release](https://github.com/veeenu/sekiro-practice-tool/releases/tag/nightly)
instead of the latest stable release. This is an automated release with the very latest changes,
intended for faster issues feedback cycles. Don't use it otherwise!

## Where are all the key bindings?

You can customize the default ones or add your own by editing
`jdsd_sekiro_practice_tool.toml` with your favorite text editor.

The bundled file contains all possible settings with predefined hotkeys and is
mostly self-explanatory.

You can find a list of supported hotkey codes [here](https://github.com/veeenu/darksoulsiii-practice-tool/blob/7aa6ac33c6f155d35d0fa99ab100c8caa13913f9/practice-tool/src/util/vk.rs#L15-L186).

## What versions of the game are supported?

All of them! When new patches come out, a new release with compatibility will be drafted as soon as possible.

## Will I get banned if I use this online?

Use at your own risk. Bans are unlikely, but in doubt, make backups of your savefiles and only use the tool offline.
By using the tool, you agree that I will not be held liable for any bans or unintended side effects resulting from the usage of the tool.

## I want to talk to you!

You can contact me on [my Discord server](https://discord.gg/jCVjxjHZ).
Please use the [Practice Tool help channel](https://discord.com/channels/267623298647457802/996101875214585867)
if you have questions about the Practice Tool.

## I want to watch your speedruns!

Sure! See you over here 👉 [https://twitch.tv/johndisandonato](https://twitch.tv/johndisandonato)!

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

