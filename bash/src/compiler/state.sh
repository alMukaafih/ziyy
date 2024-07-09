#!/usr/bin/env bash

source $ZIYY_ROOT/value/main.sh

declare -a __z_state_tags
declare -a __z_state_saves

function __z_state_new {
    __z_state_tags=("")
    # shellcheck disable=SC2206
    __z_state_saves=("$__z_RESET")
}

function __z_state_push {
    local tag=$1
    local string=$2
    # shellcheck disable=SC2154
    local src=${__z_scanner[src]}
    local len=${#src}
    local l=$(( len - 1 ))
    local s=${__z_state_saves[$l]}
    s+=$string
    __z_state_saves+=("$s")
    __z_state_tags+=("$tag")
}

function __z_state_pop {
    local a=${__z_state_tags[-1]}
    local b=${__z_state_saves[-1]}
    unset -v '__z_state_tags[-1]'
    unset -v '__z_state_saves[-1]'
    __z_return_arr=("$a" "$b")
}

function __z_state_current_tag {
    local l=${__z_state_tags[-1]}
    __z_return=$l
}

function __z_state_current_save {
    local l=${__z_state_saves[-1]}
    __z_return=$l
}