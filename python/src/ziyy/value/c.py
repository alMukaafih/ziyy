from typing import Final

class C:
    BLACK: Final[str]   = "\x1b[30m"
    # Red Color.
    RED: Final[str]     = "\x1b[31m"
    # Green Color.
    GREEN: Final[str]   = "\x1b[32m"
    # Yellow Color.
    YELLOW: Final[str]  = "\x1b[33m"
    # Blue Color.
    BLUE: Final[str]    = "\x1b[34m"
    # Magenta Color.
    MAGENTA: Final[str] = "\x1b[35m"
    # Cyan Color.
    CYAN: Final[str]    = "\x1b[36m"
    # White Color.
    WHITE: Final[str]   = "\x1b[37m"

    # RGB Color.
    @staticmethod
    def rgb(r: int, g: int, b: int) -> str:
        return f"\x1b[38;2;{r};{g};{b}m"