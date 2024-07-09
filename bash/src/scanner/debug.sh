#!/usr/bin/env bash

source $ZIYY_ROOT/scanner/main.sh

function debug {
    __z_scanner_new "$1"
    local out=$2
    local line=-1
    while true; do
        __z_scanner_scan_token
        # shellcheck disable=SC2154
        if [[ ${__z_token[err_code]} -eq 0 ]]; then
            local content=${__z_token[content]}
        else
            local content="Unexpected character."
        fi
        if [[ ${__z_token[line]} -ne $line ]]; then
            $out "%4d " "${__z_token[line]}"
            line=${__z_token[line]}
        else
            $out "   | "
        fi
        $out "%s '%s'\n" "${__z_token[kind]}" "$content"
        if [[ ${__z_token[kind]} == "Eof" ]]; then
            break
        fi
    done
}

debug "Convenient Terminal Output Styler.

<green><b><u>Usage:</u></b> <cyan><b>ziyy</b> <i>[OPTION] [TEXT]</i></cyan>

<b><u>Options:</u></b></green>
  <cyan><b>-V</b></cyan>, <cyan><b>--version</b></cyan>
          Print version info and exit
  <cyan><b>-f</b></cyan>, <cyan><b>--file</b> \<FILENAME\></cyan>
          Read input from file.
  <cyan><b>-n</b></cyan>, <cyan><b>--no-newline</b></cyan>
          Do not print newline after text.
  <cyan><b>-h</b></cyan>, <cyan><b>--help</b></cyan>
          Print help
" "builtin printf"