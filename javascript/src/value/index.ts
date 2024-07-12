export * from "./c"
export * from "./x"

/** Bold Value. */
export const B: string = "\x1b[1m";
/** Italics Value. */
export const I: string = "\x1b[3m";
/** Underline Value. */
export const U: string = "\x1b[4m";
/** Strike through Value. */
export const S: string = "\x1b[9m";

export const RESET: string   = "\x1b[0m";
export const RESET_B: string = "\x1b[22m";
export const RESET_I: string = "\x1b[23m";
export const RESET_U: string = "\x1b[24m";
export const RESET_S: string = "\x1b[29m";
export const RESET_C: string = "\x1b[39m";
export const RESET_X: string = "\x1b[49m";
