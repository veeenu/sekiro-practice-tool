from ctypes import *
from ctypes.wintypes import *
from win32 import win32gui

print(win32gui.FindWindow(None, "Sekiro"))