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

## How can I change the key bindings?

You can customize the default ones or add your own by editing
`jdsd_sekiro_practice_tool.toml` with your favorite text editor.

The bundled file contains all possible settings with predefined hotkeys and is mostly
self-explanatory.

You can find a list of supported hotkey codes [here](https://github.com/veeenu/practice-tool-core/blob/2960d851005ca0edaf030472cdddd3c992f077f9/src/key.rs#L7-L151).

Valid combinations are:
- Individual keys: `"tab"`, `"left"`
- Keys with up to 3 modifiers, separated by `+`: `"ctrl+x"`, `"alt+1"`, `"ctrl+rshift+alt+q"`.

  Valid modifiers are:
  - `ctrl`, `shift`, `alt`, `super` (bilateral)
  - `lctrl`, `lshift`, `lalt`, `lsuper` (left variant)
  - `rctrl`, `rshift`, `ralt`, `rsuper` (right variant)

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

