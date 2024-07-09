#!/usr/bin/env bash

source $ZIYY_ROOT/compiler/main.sh

__z_string_s=""
function __z_string {
    __z_string_s+="$2"
}

function __z_compile {
    declare -A vars
    __z_C_rgb 0 150 75
    # shellcheck disable=SC2154
    vars[green]=$__z_return
    __z_C_rgb 0 150 150
    # shellcheck disable=SC2034
    vars[cyan]=$__z_return

    __z_compiler_new "$1" "$2" vars
    __z_compiler_compile
}

function style {
    __z_compiler_new "$1" __z_string vars
    __z_compiler_compile
    return=$__z_string_s
}

if [[ ${0##*/} == "libziyy.sh" ]]; then
    style "[b][c: red]Hello World!"
    echo -e "$return"
fi