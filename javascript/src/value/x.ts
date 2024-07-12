export class X {
    /** Black Color. */
    static BLACK: string   = "\x1b[40m"
    /** Red Color. */
    static RED: string     = "\x1b[41m"
    /** Green Color. */
    static GREEN: string   = "\x1b[42m"
    /** Yellow Color. */
    static YELLOW: string  = "\x1b[43m"
    /** Blue Color. */
    static BLUE: string    = "\x1b[44m"
    /** Magenta Color. */
    static MAGENTA: string = "\x1b[45m"
    /** Cyan Color. */
    static CYAN: string    = "\x1b[46m"
    /** White Color. */
    static WHITE: string   = "\x1b[47m"

    /** RGB Color. */
    static rgb(r: number, g: number, b: number): string {
        return `\x1b[48;2;${r};${g};${b}m`
    }
}