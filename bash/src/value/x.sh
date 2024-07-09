#!/usr/bin/env bash

# Black Color.
__z_X_BLACK="\x1b[40m"
# Red Color.
__z_X_RED="\x1b[41m"
# Green Color.
__z_X_GREEN="\x1b[42m"
# Yellow Color.
__z_X_YELLOW="\x1b[43m"
# Blue Color.
__z_X_BLUE="\x1b[44m"
# Magenta Color.
__z_X_MAGENTA="\x1b[45m"
# Cyan Color.
__z_X_CYAN="\x1b[46m"
# White Color.
__z_X_WHITE="\x1b[47m"

# RGB Color.
__z_X_rgb() {
    __z_return="\x1b[48;2;${1};${2};${3}m"
}
