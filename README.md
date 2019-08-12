# sekiro-practice-tool

Just a minimal Sekiro trainer for practicing speedruns. If you need a more complete tool, check out [Jiiks' tool](https://github.com/Jiiks/Sekiro.SpeedrunUtility).

The project is comprised of a `.dll` file, an `.exe` file and an optional configuration file (see next section). To run it, just start the game and double-click the
`.exe` file. Press `F11` to show the tool window. If the tool window is not shown, there is something wrong; please contact `johndisandonato#4484` on Discord or submit an issue here on GitHub.

## Settings

Settings are stored in a file named `jdsd_sekiro_practice_tool.toml` in the same directory as the tool.
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