export class C {
    /** Black Color. */
    static BLACK: string   = "\x1b[30m"
    /** Red Color. */
    static RED: string     = "\x1b[31m"
    /** Green Color. */
    static GREEN: string   = "\x1b[32m"
    /** Yellow Color. */
    static YELLOW: string  = "\x1b[33m"
    /** Blue Color. */
    static BLUE: string    = "\x1b[34m"
    /** Magenta Color. */
    static MAGENTA: string = "\x1b[35m"
    /** Cyan Color. */
    static CYAN: string    = "\x1b[36m"
    /** White Color. */
    static WHITE: string   = "\x1b[37m"

    /** RGB Color. */
    static rgb(r: number, g: number, b: number): string {
        return `\x1b[38;2;${r};${g};${b}m`
    }
}