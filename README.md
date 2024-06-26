# Ziyy - Convenient Terminal Output Styler.

Style your Terminal output using html like Tags. For example to make your output bold, enclose it using `<b>` and `</b>` to make it bold.

> You can omit the closing tag.

## Tags
The following is a table of all supported tags,
| Tag | Description |
| --- | ----------- |
| `<c.'color'>...</c>` | Color text using `color`.
| `<x.'color'>...</x>` | Color the background of text using `color`.
| `<b>...</b>` | Embolden text.
`<i>...</i>` | Italicize text.
`<u>...</u>` | Underline text.
`<s>...</s>` | Strike through text.

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

