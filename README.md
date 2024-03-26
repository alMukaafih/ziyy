# Ziyy - Convenient Terminal Output Styler.

Style your Terminal output using easy to remember Tags. For example to make your output bold, put `[b]`  before the rest of your String to make it bold. The General Syntax is:
```
"[`a tag`]some text...[`another tag`]some more text..."
```
The Tags are in pairs and each pair behave like an on-off switch. For a Tag with name `TAG`, `[TAG]` is the on switch and `[/TAG]` is the off switch. The only exception is the `[/0]` that switches off all tags. 

## Tags
The following is a table of all supported tags,
| Tag | Description |
| --- | ----------- |
| `[c:'color']` | Color the preceding text using `color`. |
| `[/c]` | Reset color.
| `[x:'color']` | Color the background preceding text. using `color` |
| `[/x]` | Reset Background color.
| `[b]` | Embolden preceding text.
`[/b]` | Remove boldness.
`[i]` | Italicize preceding text.
`[/i]` | Remove italics.
`[u]` | Underline preceding text.
`[/u]` | Remove underline.
`[s]` | Strike through preceding text.
`[/s]` | Remove strike.
`[/0]` | Remove all styling.

## Valid Colors
These are the current valid colors for both text and background:
| Color | Description |
| ----- | ----------- |
black | Terminal's default black color.
red | Terminal's default red color.
green | Terminal's default green color.
yellow | Terminal's default yellow color.
blue | Terminal's default blue color.
magenta | Terminal's default magenta color.
cyan | Terminal's default cyan color.
white | Terminal's default white color.
rgb(#, #, #) | RGB color. # represents a Number within the Range 0 - 255

> A new color will overwrite the previous color
