#!/usr/bin/env bash

source $ZIYY_ROOT/libziyy.sh
out="builtin printf"

function usage {
    __z_compile "Convenient Terminal Output Styler.

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
" "$out"
}

function main {
    # shellcheck disable=SC2206
    args=(${@:1})
    if [[ ${#args} -lt 1 ]]; then
        usage
        exit 0
    fi
    first=$1
    if [[ $first == "-n" ]] || [[ $first == "--no-newline" ]]; then
        __z_compile "$2" "$out"
    elif [[ $first == "-f" ]] || [[ $first == "--filename" ]]; then
        if [[ ${#args} -eq 1 ]]; then
            usage
            exit 1
        fi
        if ! [[ -f "$2" ]]; then
            usage
            exit 1
        fi
        file=$(<"$2")
        __z_compile "$file" "$out"
    elif [[ $first == "-V" ]] || [[ $first == "--version" ]]; then
        $out "ziyy 2.0.0-beta.0\n"
    elif [[ $first == "-h" ]] || [[ $first == "--help" ]]; then
        usage
        exit 0
    else
        __z_compile "$1" "$out"
    fi
}
main "$@"
