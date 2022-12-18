# AutoSDF

procedurally generated signed distance fields

## controls

- move mouse to look around
- W,A,S,D/arrow keys to move forward/backwards/left/right + Q,E/PageUp,PageDown to move up/down
- R to create a new SDF - use this a lot! most SDFs are boring!
- Space to "unpause time" (start animating). On some SDFs this might not do much.
  Press space again to pause time, and shift+space to rewind time.
- 0 to reset location + time
- =/- (equals/minus) to change the level set
- Ctrl+C to copy SDF to clipboard, Ctrl+V to paste SDF from clipboard. On Linux
  if you close the application the clipboard contents will be lost (thanks a lot X11).
- F to go fullscreen

## saving SDFs

if you get an SDF which looks cool, you can copy it with Ctrl+C, and save it somewhere/share it with
your friends.

also a list of SDFs is stored in `scenes.txt`.

## info

AutoSDF can be configured by editing `settings.txt`.

If AutoSDF is running slow, you should make `sdf-length` and `color-length` smaller.
If it's running fast, and you want more interesting shapes, you should increase them.
