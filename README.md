# Ziyy - Terminal Styles.

<p align="center">
  <img src='logo.svg' width='250' alt='iyy Logo'>
</p>

## Overview
Style your Terminal using HTML `<b>..</b>`.

## Example
```html
<b u i c="rgb(5, 165, 104)">This is a Bold Green Underlined Text in Italics</b>
```
### Output
<pre>
<b style="color:rgb(5, 165, 104);"><i><u>This is a Bold Green Underlined Text in Italics</u></i></b>
</pre>

## Tags
All tags have [Effects](#effects), [General Colors](#general-colors) and [Inheritance](#inheritance) Properties except for `<br>`.

### Text Effects
| Tag | Effect | Properties |
| --------| ----------- | --- |
| `<b>` | Bold |  |
| `<d>` | Dim |  |
| `<h>` | Hide |  |
| `<k>` | Blink |  |
| `<r>` | Reverse foreground and background | |
| `<i>` | Italics |  |
| `<s>` | Strike-Through |  |
| `<u>` | Underline | `double`: Double Underline |

### Text Color
| Tag | Color | Properties |
| --- | ------ | -- |
| `<c>` | Foreground Color | [Special Colors](#special-colors) properties. |
| `<x>` | Background Color | [Special Colors](#special-colors) properties. |

### Others
| Tag | Description | Properties |
| --- | ------ | -- |
| `<a>` | Insert a link. | `href`: url of the link,  |
| `<p>` | Insert new Paragraph | `tab`: indent the paragraph with *n* spaces,  |
| `<br>` | Insert a line break. | `n`: no of line breaks to insert. default is 1. |
| `<let>` | Declares new custom tag.  | `name`: Name of tag. Supports only ASCII character set.  |
| `<ziyy>` | The root of other tags. |  |

## Properties
### Effects
| Property | Description | Type |
| --- | ------ | --- |
| `b \| bold` | Bold | `"true" \| "false" or unassigned (true` |
| `d \| dim` | Dim |  |
| `h \| hidden \| hide \| invisible` | Hide |  |
| `k \| blink` | Blink |  |
| `r \| invert \| reverse \| negative` | Reverse |  |
| `i \| italics` | Italics |  |
| `s \| strike-through` | Strike-Through |  |
| `u \| underline` | Underline |  |
| `uu \| double-underline` | Double Underline |  |

### General Colors
| Property | Description | Type |
| --- | ------ | --- |
| `c \| fg`   | Foreground color | `"black" \| "red" \| "green" \| "yellow" \| "blue" \| "magenta" \| "cyan" \| "white" \| "fixed(#)" \| "rgb(#, #, #)"` |
| `x \| bg`   | Background color |  | ANSI 4 bit colors (`<c>` and `<x>` tags only) | `unassinged` |

## Special Colors
| Property | Description | Type |
| --- | ------ | --- |
| `fixed` | ANSI 256 color | `"#"` |
| `rgb` | Rgb colors | `"#, #, #"` |
> `#` is a number within `0..255`

### Inheritance
| Property | Description | Type |
| --- | ------ | --- |
| `src` | Inherit properties from a tag. | `string` |

