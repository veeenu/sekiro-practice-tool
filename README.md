# sekiro-practice-tool

Just a minimal Sekiro trainer for practicing speedruns. If you need a more complete tool, check out [Jiiks' tool](https://github.com/Jiiks/Sekiro.SpeedrunUtility).

It is a DLL designed to work with [Jiiks' Universal Proxy Chain](https://github.com/Jiiks/UniversalProxyChain), which all speedrunners ought to have installed.
To install it, copy it within the `uclib` directory under the game's folder.

## Settings

Settings are stored in a file named `JDSD-SekiroPracticeTool.toml` in the same directory as `Sekiro.exe`.
The file, if not present, is automatically filled with default values. Any errors coming from wrong syntax or
undefined fields may be fixed by simply removing the file.

```toml
[mappings]
  # pick values from https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
  show = "VK_F11"       # show/hide tool window
  stealth = "VK_F10"    # toggle stealth mode (enemies don't notice you)
  collision = "VK_F5"   # toggle collisions meshes
  ai = "VK_F8"          # toggle AI
  no_damage = "VK_F9"   # toggle no damage
  consume = "VK_F4"     # toggle infinite consumables
  save_pos = "VK_F7"    # save position
  load_pos = "VK_F1"    # load position
  quitout = "VK_F6"     # instant quitout
[settings]
  enabled = "true"      # set to "false" (quotation marks included) to disable the tool
  debug = "false"       # unimplemented, no effect yet
```

### Violating the speedrunning community's rules

It is likely that the mod is not allowed in speedruns. To disable it, it is enough to set `enabled = "false"` under `settings`. The mod will start, read the setting, and then immediately exit without affecting the game.

## Building

Development build (optimized but with debug symbols):

```
$ python build.py
```

Release build (optimized, debug symbols stripped):

```
$ python build.py Release
```

## Electron-based version

In the `electron-amnesiaclapp` branch you can find an earlier version of the tool, based on Electron and my [amnesiaclapp](https://github.com/veeenu/amnesiaclapp) library.
It is unmaintained and to be considered deprecated.