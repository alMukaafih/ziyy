from .c import C
from .x import X

# Bold Value.
B: str = "\x1b[1m"
# Italics Value.
I: str = "\x1b[3m"
# Underline Value.
U: str = "\x1b[4m"
# Strike through Value.
S: str = "\x1b[9m"

RESET: str   = "\x1b[0m"
RESET_B: str = "\x1b[22m"
RESET_I: str = "\x1b[23m"
RESET_U: str = "\x1b[24m"
RESET_S: str = "\x1b[29m"
RESET_C: str = "\x1b[39m"
RESET_X: str = "\x1b[49m"