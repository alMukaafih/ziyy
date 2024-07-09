#!/usr/bin/env bash

source $ZIYY_ROOT/compiler/parser.sh

function __z_compiler_new {
    __z_parser_new "$1" "$2" "$3"
}

function __z_compiler_compile {
    __z_parser_parse_to_out
}