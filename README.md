# Ziyy 2.0 - Terminal Styler.

<p align="center">
  <img src='https://raw.githubusercontent.com/alMukaafih/ziyy/main/logo.svg' width='250' alt="Ziyy's Logo">
</p>

## Overview
Style your Terminal using HTML-like tags `<b>..</b>`, making it easy to apply styles such as bold, italics, and colors directly in your terminal output. For example, `<b u c="red">Hello</b>` (where `c` stands for color) will render "Hello" in bold and red.

## Example
```html
<b u i c="rgb(5, 165, 104)">This is a bold green underlined text in italics</b>
```

### [Ansi Escape Code](https://en.wikipedia.org/wiki/ANSI_escape_code) Equivalent
```
\x1b[1;4;3;38;2;5;165;104mThis is a bold green underlined text in italics\x1b[22;24;23;39m
```

### Output
<pre>
<b style="color:rgb(5, 165, 104);"><i><u>This is a bold green underlined text in italics</u></i></b>
</pre>

## Elements
All elements have [Effects](#effects), [General Colors](#general-colors) and [Inheritance](#inheritance) attributes except for `<br>`, which is an exception because it only inserts line breaks and does not modify text appearance.

### Main root
| Element | Description | Attributes |
| --------| ----------- | --- |
| `<ziyy>` | The root element. |  |

### Text content
| Element | Description | Attributes |
| --------| ----------- | --- |
| `<div>` | Used to group related content |  |
| `<pre>` | Represents preformatted text which is to be presented exactly as written. Whitespaces are preserved. |  |
| `<p>` | Represents a paragraph. | `indent`: indent the paragraph with *n* spaces,  |

### Inline text semantics
| Element | Description | Attributes |
| --------| ----------- | --- |
| `<a>` | Creates a hyperlink. For example: `<a href="https://example.com">Example</a>`. | `href`: url of the link,  |
| `<b> \| <strong>` | Causes text to be bold. |  |
| `<br>` | Produces a line break in text (carriage-return). It does not support any styling attributes. | `n`: number of line breaks to insert. Default is 1. |
| `<d> \| <dim>` | Causes text to be dim. |  |
| `<h> \| <hide>` | Causes text to be hidden. |  |
| `<k> \| <blink>` | Causes text to blink. |  |
| `<r> \| <invert>` | Reverse foreground and background colors of text. | |
| `<i> \| <em>` | Causes text to be italicized. |  |
| `<s> \| <del>` | Strikes through text. |  |
| `<u> \| <ins>` | Underlines text. | `double`: Use double lines to underline text. Unlike `<uu>`, which is a standalone element for double underlining, this attribute applies double underlining to the `<u>` element. |
| `<uu>` | Double Underlines text. |  |

### Text Color
| Element | Description | Attributes |
| --- | ------ | -- |
| `<c>` | Sets foreground color. | [Special Colors](#special-colors) attributes. |
| `<x>` | Sets background color. | [Special Colors](#special-colors) attributes. |

### Declaration
| Element | Description | Attributes |
| --- | ------ | -- |
| `<let>` | Declares new custom element.  | `id`: Name of element.  |

## Attributes
### Effects
| Property | Description | Type |
| --- | ------ | --- |
| `b \| bold` | Causes text to be bold. | [bool](#bool) |
| `d \| dim` | Causes text to be dim. | [bool](#bool) |
| `h \| hidden \| hide \| invisible` | Causes text to be hidden. | [bool](#bool) |
| `k \| blink` | Causes text to blink. | [bool](#bool) |
| `r \| invert \| reverse \| negative` | Reverse foreground and background colors of text. | [bool](#bool) |
| `i \| italics` | Causes text to be italicized. | [bool](#bool) |
| `s \| strike \| strike-through` | Strike through text. | [bool](#bool) |
| `u \| under \|  underline` | Underlines text. | [bool](#bool) |
| `uu \| double-under \| double-underline` | Underlines text using double lines. | [bool](#bool) |

### General Colors
| Property | Description | Type |
| --- | ------ | --- |
| `c \| fg`   | Sets foreground color. | [color](#color)  |
| `x \| bg`   | Sets background color. |[color](#color) |

## Special Colors
| Property | Description | Type |
| --- | ------ | --- |
| [fixed](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit) | ANSI 256 color | "[u8](#u8)" |
| [rgb](https://www.w3schools.com/colors/colors_rgb.asp) | Rgb colors | "[u8](#u8), [u8](#u8), [u8](#u8)" |
| `black` | Black color | [mode](#mode) |
| `red` | Red color | [mode](#mode) |
| `green` | Green color | [mode](#mode) |
| `yellow` | Yellow color | [mode](#mode) |
| `blue` | Blue color | [mode](#mode) |
| `magenta` | Magenta color | [mode](#mode) |
| `cyan` | Cyan color | [mode](#mode) |
| `white` | White color | [mode](#mode) |

### Inheritance
| Property | Description | Type |
| --- | ------ | --- |
| `class` | A space-separated list of elements to inherit styles from. Can either be a builtin element or a element declared with `<let>`. | [string](#string) |


## Types

### `bool`
If equals "true" (case-sensitive) or is an empty attribute, it enables the style. If equals "false" (case-sensitive), it disables the style.

### `color`
possible values are "black" \| "red" \| "green" \| "yellow" \| "blue" \| "magenta" \| "cyan \| "white" \| "fixed([u8](#u8))" \| "[rgb(red, green, blue)](https://www.w3schools.com/colors/colors_rgb.asp)" \| [#rrggbb or #rgb](https://www.w3schools.com/colors/colors_hexadecimal.asp). See [Special Colors](#special-colors).

### `mode`
if equals "light", enables brighter or pastel shades of the color. If equals "dark" or is an empty attribute, enables darker or muted shades of the color.

### `string`
Any [UTF-8](https://en.wikipedia.org/wiki/UTF-8) text between single `'` and double `"` quotes. For example: `'example'` or `"example"`.

### `u8`
A number within the range 0 to 255. For example, 128 is a valid `u8` value.

