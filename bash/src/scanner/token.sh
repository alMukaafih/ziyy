#!/usr/bin/env bash

# __z_T_LeftParen=__z_T_LeftParen
# __z_T_RightParen=__z_T_RightParen
# __z_T_Comma=__z_T_Comma
# __z_T_OpenTag=__z_T_OpenTag
# __z_T_CloseTag=__z_T_CloseTag
# __z_T_Dot=__z_T_Dot
# __z_T_Slash=__z_T_Slash
# __z_T_BackSlash=__z_T_BackSlash
# Literals.
# __z_T_Identifier=__z_T_Identifier
# __z_T_Number=__z_T_Number
# __z_T_Text=__z_T_Text
# Builtin Variables.
# __z_T_Black=__z_T_Black
# __z_T_Red=__z_T_Red
# __z_T_Green=__z_T_Green
# __z_T_Yellow=__z_T_Yellow
# __z_T_Blue=__z_T_Blue
# __z_T_Magenta=__z_T_Magenta
# __z_T_Cyan=__z_T_Cyan
# __z_T_White=__z_T_White
# __z_T_Rgb=__z_T_Rgb
# __z_T_B=__z_T_B
# __z_T_C=__z_T_C
# __z_T_I=__z_T_I
# __z_T_S=__z_T_S
# __z_T_U=__z_T_U
# __z_T_X=__z_T_X
# Keywords.
# __z_T_Eof=__z_T_Eof
# __z_T_Error=__z_T_Error

# Token
declare -A __z_token
function __z_token_new {
    kind=$1
    content=$2
    err_code=$3
    line=$4

    __z_token[kind]=$kind
    __z_token[content]=$content
    __z_token[err_code]=$err_code
    __z_token[line]=$line
}