# Ziyy - Terminal Styler.

<p align="center">
  <img src='https://raw.githubusercontent.com/alMukaafih/ziyy/refs/heads/main/logo.svg' width='250' alt="Ziyy's Logo">
</p>

## Overview
Style your Terminal using HTML `<b>..</b>`.

## Example
```html
<b u i c="rgb(5, 165, 104)">This is a bold green underlined text in italics</b>
```

Ansi Equivalent
```
\x1b[1;4;3;38;2;5;165;104mThis is a bold green underlined text in italics\x1b[22;24;23;39m
```

### Output
<pre>
<b style="color:rgb(5, 165, 104);"><i><u>This is a bold green underlined text in italics</u></i></b>
</pre>

## Elements
All elements have [Effects](#effects), [General Colors](#general-colors) and [Inheritance](#inheritance) attributes except for `<br>`.

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
| `<a>` | Creates a hyperlink. | `href`: url of the link,  |
| `<b> \| <strong>` | Causes text to be bold. |  |
| `<br>` | Produces a line break in text (carriage-return). | `n`: no of line breaks to insert. Default is 1. |
| `<d> \| <dim>` | Causes text to be dim. |  |
| `<h> \| <hide>` | Causes text to be hidden. |  |
| `<k> \| <blink>` | Causes text to blink. |  |
| `<r> \| <invert>` | Reverse foreground and background colors of text. | |
| `<i> \| <em>` | Causes text to be italicized. |  |
| `<s> \| <del>` | Strikes through text. |  |
| `<u> \| <ins>` | Underlines text. | `double`: Use double lines to underline text. |
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
| `fixed` | ANSI 256 color | "[u8](#u8)" |
| `rgb` | Rgb colors | "[u8](#u8), [u8](#u8), [u8](#u8)" |
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
if equals "true" or is an empty attribute, it enables style. if equals "false" disables style.

### `color`
possible values are "black" \| "red" \| "green" \| "yellow" \| "blue" \| "magenta" \| "cyan \| "white" \| "fixed([u8](#u8))" \| "rgb([u8](#u8), [u8](#u8), [u8](#u8))" \| [hex](#hex). See [Special Colors](#special-colors).

### `hex`
\#XXXXXX or \#XXX. Posible values for X are 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, a, b, c, d, e, f, A, B, C, D, E and F. Examples are #001f57 and #fff.

### `mode`
if equals "light", enables light or bright version of color. If equals "dark" or is an empty attribute, enables dark version of color.

### `string`
Any UTF-8 text between single `'` and double `"` quotes.

### `u8`
A number within the range 0..=255.
