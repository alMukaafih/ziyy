#!/usr/bin/env bash

# Black Color.
__z_C_BLACK="\x1b[30m"
# Red Color.
__z_C_RED="\x1b[31m"
# Green Color.
__z_C_GREEN="\x1b[32m"
# Yellow Color.
__z_C_YELLOW="\x1b[33m"
# Blue Color.
__z_C_BLUE="\x1b[34m"
# Magenta Color.
__z_C_MAGENTA="\x1b[35m"
# Cyan Color.
__z_C_CYAN="\x1b[36m"
# White Color.
__z_C_WHITE="\x1b[37m"

# RGB Color.
__z_C_rgb() {
    __z_return="\x1b[38;2;${1};${2};${3}m"
}
