#!/usr/bin/env bash

source $ZIYY_ROOT/scanner/token.sh

function __z_is_alpha {
    c=$1
    case $c in
    [abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_])
        __z_return=1 #true
        ;;
    *)
        __z_return=0 #false
        ;;
    esac
}

function __z_is_digit {
    c=$1
    case $c in
    [0123456789])
        __z_return=1 #true
        ;;
    *)
        __z_return=0 #false
        ;;
    esac
}

declare -A __z_scanner
function __z_scanner_new {
    __z_scanner[src]=$1
    __z_scanner[start]=0
    __z_scanner[current]=0
    __z_scanner[line]=1
    __z_scanner[text_line]=1
    __z_scanner[text_mode]=1 #true
    __z_scanner[escape]=0
}

function __z_scanner_is_at_end {
    local current=${__z_scanner[current]}
    local src=${__z_scanner[src]}
    local len=${#src}
    if ((current + 1 > len)); then
        __z_return=1 #true
    else
        __z_return=0 #false
    fi
}

function __z_scanner_advance {
    local current=${__z_scanner[current]}
    local src=${__z_scanner[src]}

    __z_scanner[current]=$((current + 1))
    __z_return=${src:$current:1}
}

function __z_scanner_peek {
    local current=${__z_scanner[current]}
    local src=${__z_scanner[src]}

    __z_scanner_is_at_end
    if ((__z_return == 0)); then
        __z_return=${src:$current:1}
    else
        __z_return="\0"
    fi
}

function __z_scanner_peek_next {
    local current=${__z_scanner[current]}
    local src=${__z_scanner[src]}
    ((current++))

    __z_scanner_is_at_end
    if ((__z_return == 0)); then
        __z_return=${src:$current:1}
    else
        __z_return="\0"
    fi
}

function __z_scanner_make_token {
    local kind=$1
    local start=${__z_scanner[start]}
    local current=${__z_scanner[current]}
    local src=${__z_scanner[src]}
    local len=$((current - start))

    s=${src:$start:$len}
    __z_token[kind]=$kind
    # shellcheck disable=SC2154
    __z_token[content]=$s
    # shellcheck disable=SC2154
    __z_token[err_code]=0
    # shellcheck disable=SC2154
    __z_token[line]=${__z_scanner[line]}
}

function __z_scanner_error_token {
    local code=$1
    local start=${__z_scanner[start]}
    local current=${__z_scanner[current]}
    local src=${__z_scanner[src]}
    local len=$((current - start))

    s=${src:$start:$len}
    __z_token[kind]=Error
    # shellcheck disable=SC2154
    __z_token[content]=$s
    # shellcheck disable=SC2154
    __z_token[err_code]=$code
    # shellcheck disable=SC2154
    __z_token[line]=${__z_scanner[line]}
}

function __z_scanner_text_token {
    local start=${__z_scanner[start]}
    local current=${__z_scanner[current]}
    local src=${__z_scanner[src]}
    local len=$((current - start))

    s=${src:$start:$len}
    __z_token[kind]=Text
    # shellcheck disable=SC2154
    __z_token[content]=$s
    # shellcheck disable=SC2154
    __z_token[err_code]=0
    # shellcheck disable=SC2154
    __z_token[line]=${__z_scanner[line]}
}

function __z_scanner_skip_whitespace {
    while true; do
        if [[ ${__z_scanner[text_mode]} -eq 1 ]]; then
            return
        fi
        __z_scanner_peek
        local c=$__z_return
        case $c in
        ' ' | "\r" | "\t")
            __z_scanner_advance
            continue
            ;;
        "\n")
            line=${__z_scanner[line]}
            __z_scanner[line]=$((line + 1))
            text_line=${__z_scanner[line]}
            __z_scanner[text_line]=$((text_line + 1))
            __z_scanner_advance
            continue
            ;;
        *)
            return
            ;;
        esac
    done
}

function __z_scanner_check_keyword {
    local start_1=$1
    local length=$2
    local rest=$3
    local kind=$4

    local start_2=${__z_scanner[start]}
    local start=$((start_1 + start_2))
    local current=${__z_scanner[current]}
    local src=${__z_scanner[src]}
    local len=$((current - start))

    s=${src:$start:$len}

    if ((current - start_2 == start_1 + length)) && [[ $s == "$rest" ]]; then
        __z_return=$kind
    else
        __z_return=Identifier
    fi
}

function __z_scanner_identifier_kind {
    local start=${__z_scanner[start]}
    local start_1=$((start + 1))
    local start_2=$((start + 2))
    local current=${__z_scanner[current]}
    local src=${__z_scanner[src]}

    if ((current - start == 1)); then
        case ${src:$start:1} in
        b) __z_return=B ;;
        c) __z_return=C ;;
        i) __z_return=I ;;
        s) __z_return=S ;;
        u) __z_return=U ;;
        x) __z_return=X ;;
        *) __z_return=Identifier ;;
        esac
    else
        case ${src:$start:1} in
        b)
            case ${src:$start_1:1} in
            l)
                case ${src:$start_2:1} in
                a) __z_scanner_check_keyword 3 2 ck Black ;;
                u) __z_scanner_check_keyword 3 1 e Blue ;;
                *) __z_return=Identifier ;;
                esac
                ;;
            *) __z_return=Identifier ;;
            esac
            ;;
        c) __z_scanner_check_keyword 1 3 yan Cyan ;;
        g) __z_scanner_check_keyword 1 4 reen Cyan ;;
        m) __z_scanner_check_keyword 1 6 agenta Magenta ;;
        r)
            case ${src:$start_1:1} in
            e) __z_scanner_check_keyword 2 1 d Red ;;
            g) __z_scanner_check_keyword 2 1 b Rbg ;;
            esac
            ;;
        w) __z_scanner_check_keyword 1 4 hite White ;;
        y) __z_scanner_check_keyword 1 5 ellow Yellow ;;
        *) __z_return=Identifier ;;
        esac
    fi
}

function __z_scanner_identifier {
    __z_scanner_peek
    self_peek=$__z_return
    __z_is_alpha "$self_peek"
    self_is_alpha=$__z_return
    __z_is_digit "$self_peek"
    self_is_digit=$__z_return

    while [[ $self_is_alpha -eq 1 ]] || [[ $self_is_digit -eq 1 ]]; do
        __z_scanner_advance

        __z_scanner_peek
        self_peek=$__z_return
        __z_is_alpha "$self_peek"
        self_is_alpha=$__z_return
        __z_is_digit "$self_peek"
        self_is_digit=$__z_return
    done

    __z_scanner_identifier_kind
    kind=$__z_return
    __z_scanner_make_token "$kind"
}

function __z_scanner_number {
    __z_scanner_peek
    self_peek=$__z_return
    __z_is_digit "$self_peek"
    self_is_digit=$__z_return

    while [[ $self_is_digit -eq 1 ]]; do
        __z_scanner_advance

        __z_scanner_peek
        self_peek=$__z_return
        __z_is_digit "$self_peek"
        self_is_digit=$__z_return
    done

    __z_scanner_make_token Number
}


function __z_scanner_scan_token {
    local escape=${__z_scanner[escape]}
    if (( escape == 0  )); then
        __z_scanner_skip_whitespace
    fi
    __z_scanner[start]=${__z_scanner[current]}
    __z_scanner_peek
    local self_peek=$__z_return
    if (( escape == 2 )) && [[ $self_peek == "\\" ]]; then
        __z_scanner[escape]=1
        __z_scanner_advance
        __z_scanner_make_token BackSlash
        return
    fi
    if (( escape == 1 )); then
        __z_scanner[escape]=0
        __z_scanner_advance
        __z_scanner_text_token
        return
    fi
    __z_scanner_is_at_end
    local self_is_at_end=$__z_return
    if [[ $self_is_at_end -eq 1 ]]; then
        __z_scanner_make_token Eof
        return
    fi

    __z_scanner_advance
    local c=$__z_return
    if [[ $c == '<' ]]; then
        __z_scanner[text_mode]=0 #false
        __z_scanner_make_token OpenTag
        return
    elif [[ $c == '>' ]]; then
        __z_scanner[text_mode]=1 #true
        __z_scanner_make_token CloseTag
        return
    fi

    if [[ ${__z_scanner[text_mode]} -eq 1 ]]; then
        __z_scanner_is_at_end
        local is_at_end=$__z_return
        while [[ $is_at_end -ne 1 ]]; do
            __z_scanner_peek
            self_peek=$__z_return
            if [[ $self_peek == "\n" ]]; then
                local line=${__z_scanner[line]}
                local text_line=${__z_scanner[text_line]}
                __z_scanner[line]=$(( line + 1 ))
                __z_scanner[text_line]=$(( text_line + 1 ))
            fi
            if [[ $self_peek == "\\" ]]; then
                __z_scanner[escape]=2
                __z_scanner_text_token
                return
            fi
            if [[ $self_peek != "<" ]]; then
                __z_scanner_advance
            else
                break
            fi
            __z_scanner_is_at_end
            local is_at_end=$__z_return
        done
        __z_scanner_text_token
        return
    fi

    __z_is_alpha "$c"
    local is_alpha=$__z_return
    if [[ is_alpha -eq 1 ]]; then
        __z_scanner_identifier
        return
    fi
    __z_is_digit "$c"
    local is_digit=$__z_return
    if [[ is_digit -eq 1 ]]; then
        __z_scanner_number
        return
    fi

    case $c in
    "(") __z_scanner_make_token LeftParen ;;
    ")") __z_scanner_make_token RightParen ;;
    ",") __z_scanner_make_token Comma ;;
    ".") __z_scanner_make_token Dot ;;
    "/") __z_scanner_make_token Slash ;;
    *) __z_scanner_error_token 1 ;;
    esac
}