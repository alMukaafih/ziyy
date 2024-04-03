let reset: string = "\x1b[0m" // reset Styles
class Color {
    firstDigit: number;
    secondDigit: string | number;
    color: string;
    rgb: string;
    sub: string;
    constructor(firstDigit: number) {
        this.firstDigit = firstDigit
    }
    _escape(secondDigit: string | number) {
        this.secondDigit = secondDigit;
        return `\x1b[${this.firstDigit}${this.secondDigit}m`;
    }
    colorValue(color: string) {
        if (color == "black") {
            this.color = this._escape(0)
        } else if (color == "red") {
            this.color = this._escape(1)
        } else if (color == "green") {
            this.color = this._escape(2)
        } else if (color == "yellow") {
            this.color = this._escape(3)
        } else if (color == "blue") {
            this.color = this._escape(4)
        } else if (color == "magenta") {
            this.color = this._escape(5)
        } else if (color == "cyan") {
            this.color = this._escape(6)
        } else if (color == "white") {
            this.color = this._escape(7)
        }
        // rgb colors
        else if (color.startsWith("rgb(") && color.endsWith(")")) {
            this.rgb = color.slice(4, -1)
            this.rgb = this.rgb.replace(/\,/g, ";")
            this.rgb = this.rgb.replace(/\s/g, "")
            this.color = this._escape(`8;2;${this.rgb}`)
        }
        else {
            console.log("Error")
            //process.exit(1)
            //raise ColorError(style(f"-c yellow -x red {color}"))
        }

        color = color.replace(/\(/, "\(")
        color = color.replace(/\)/, "\)")
        this.sub = color
    }
    substitute(text: string,tag: string): string {
        return text.replace(tag, this.color)
    
    }
}

// initialize fg and bg
let fg = new Color(3)
let bg = new Color(4)

export function style(text: string): string {
    let matched;
    let value;
    //foreground color
    matched = text.match(/\[c(\s)*\:(\w|\s|\(|\)|,)*\]/g);
    if (matched != null) {
        for (let match of matched)  {
            value = match.replace(/(\s)*\:(\s)*/, "").slice(2, -1)
            fg.colorValue(value)
            text = fg.substitute(text, match)
        }
    }
    // background colors
    matched = text.match(/\[x(\s)*\:(\w|\s|\(|\)|,)*\]/g);
    if (matched != null) {
        for (let match of matched)  {
            value = match.replace(/(\s)*\:(\s)*/, "").slice(2, -1)
            bg.colorValue(value)
            text = bg.substitute(text, match)
        }
    }
    // Bold
    if (text.includes("[b]")) {
        text = text.replace(/\[b\]/g, "\x1b[1m")
    }
    // Remove Bold
    if (text.includes("[/b]")) {
        text = text.replace(/\[\/b\]/g, "\x1b[21m")
    }
    
    // Italics
    if (text.includes("[i]")) {
        text = text.replace(/\[i\]/g, "\x1b[3m")
    }
    // Remove italics
    if (text.includes("[/i]")) {
        text = text.replace(/\[\/i\]/g, "\x1b[23m")
    }
    
    // Remove colors
    if (text.includes("[/c]")) {
        text = text.replace(/\[\/c\]/g, "\x1b[39m")

    }
    if (text.includes("[/x]")) {
        text = text.replace(/\[\/x\]/g, "\x1b[49m")

    }
        
    // Underline
    if (text.includes("[u]")) {
        text = text.replace(/\[u\]/g, "\x1b[4m")
    }
    if (text.includes("[/u]")) {
        text = text.replace(/\[\/u\]/g, "\x1b[24m")
    }
        
    // Strike through
    if (text.includes("[s]")) {
        text = text.replace(/\[s\]/g, "\x1b[9m")

    }
    if (text.includes("[/s]")) {
        text = text.replace(/\[\/s\]/g, "\x1b[29m")
    }
    if (text.includes("[/0]")) {
        text = text.replace(/\[\/0\]/g, "\x1b[0m")
    }

return `${text}${reset}`
}


export function template(save: string) {
    function styler(text: string): string {
        return style(`${save}${text}`)
    }
    return styler
}

if (require.main === module) {
    console.log(style("[b]Hello[/0] World!"))
}
