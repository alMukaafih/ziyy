from typing import Final

class X:
    BLACK: Final[str]   = "\x1b[40m"
    # Red Color.
    RED: Final[str]     = "\x1b[41m"
    # Green Color.
    GREEN: Final[str]   = "\x1b[42m"
    # Yellow Color.
    YELLOW: Final[str]  = "\x1b[43m"
    # Blue Color.
    BLUE: Final[str]    = "\x1b[44m"
    # Magenta Color.
    MAGENTA: Final[str] = "\x1b[45m"
    # Cyan Color.
    CYAN: Final[str]    = "\x1b[46m"
    # White Color.
    WHITE: Final[str]   = "\x1b[47m"

    # RGB Color.
    @staticmethod
    def rgb(r: int, g: int, b: int) -> str:
        return f"\x1b[48;2;{r};{g};{b}m"