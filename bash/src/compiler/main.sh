#!/usr/bin/env bash

source $ZIYY_ROOT/compiler/parser.sh

function compiler_new {
    __z_parser_new "$1" "$2" "$3"
}

function compiler_compile {
    __z_parser_parse_to_out
}

function compiler_compile_source {
    # shellcheck disable=SC2154
    __z_scanner[src]=$1
    compiler_compile
}