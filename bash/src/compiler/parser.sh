#!/usr/bin/env bash

source $ZIYY_ROOT/compiler/state.sh
source $ZIYY_ROOT/value/main.sh
source $ZIYY_ROOT/scanner/main.sh

declare -A __z_parser

function __z_panic {
    echo Parse Error in line "$1"
    exit 1
}

function __z_parser_new {
    __z_scanner_new "$1"
    __z_parser[out]=$2
    __z_state_new
    __z_parser_variables=$3
}

function __z_parser_parse_to_out {
    local out=${__z_parser[out]}
    # shellcheck disable=SC2154
    $out "%b" "$__z_RESET"
    while true; do
        __z_scanner_scan_token
        # shellcheck disable=SC2154
        case ${__z_token[kind]} in
        Text) $out "%b" "${__z_token[content]}" ;;
        B)
            __z_state_push "${__z_token[content]}" "$__z_B"
            $out "%b" "$__z_B"

            __z_scanner_scan_token
            if [[ ${__z_token[kind]} != CloseTag ]]; then
                __z_panic 36
            fi
            ;;
        I)
            __z_state_push "${__z_token[content]}" "$__z_I"
            $out "%b" "$__z_I"

            __z_scanner_scan_token
            if [[ ${__z_token[kind]} != CloseTag ]]; then
                __z_panic 45
            fi
            ;;
        S)
            __z_state_push "${__z_token[content]}" "$__z_S"
            $out "%b" "$__z_S"

            __z_scanner_scan_token
            if [[ ${__z_token[kind]} != CloseTag ]]; then
                __z_panic 54
            fi
            ;;
        U)
            __z_state_push "${__z_token[content]}" "$__z_U"
            $out "%b" "$__z_U"

            __z_scanner_scan_token
            if [[ ${__z_token[kind]} != CloseTag ]]; then
                __z_panic 63
            fi
            ;;
        C)
            __z_scanner_scan_token
            case ${__z_token[kind]} in
            Dot)
                case ${__z_token[kind]} in
                Black)
                    __z_state_push "c" "$__z_C_BLACK"
                    $out "%b" "$__z_C_BLACK"
                    ;;
                Blue)
                    __z_state_push "c" "$__z_C_BLUE"
                    $out "%b" "$__z_C_BLUE"
                    ;;
                Cyan)
                    __z_state_push "c" "$__z_C_CYAN"
                    $out "%b" "$__z_C_CYAN"
                    ;;
                Green)
                    __z_state_push "c" "$__z_C_GREEN"
                    $out "%b" "$__z_C_GREEN"
                    ;;
                Magenta)
                    __z_state_push "c" "$__z_C_MAGENTA"
                    $out "%b" "$__z_C_MAGENTA"
                    ;;
                Red)
                    __z_state_push "c" "$__z_C_RED"
                    $out "%b" "$__z_C_RED"
                    ;;
                White)
                    __z_state_push "c" "$__z_C_WHITE"
                    $out "%b" "$__z_C_WHITE"
                    ;;
                Yellow)
                    __z_state_push "c" "$__z_C_YELLOW"
                    $out "%b" "$__z_C_YELLOW"
                    ;;
                Rgb)
                    __z_scanner_scan_token
                    if [[ ${__z_token[kind]} != LeftParen ]]; then
                        __z_panic 106
                    fi

                    __z_scanner_scan_token
                    if [[ ${__z_token[kind]} != Number ]]; then
                        __z_panic 111
                    fi
                    local r=${__z_token[content]}

                    __z_scanner_scan_token
                    if [[ ${__z_token[kind]} != Comma ]]; then
                        __z_panic 117
                    fi

                    __z_scanner_scan_token
                    if [[ ${__z_token[kind]} != Number ]]; then
                        __z_panic 122
                    fi
                    local g=${__z_token[content]}

                    __z_scanner_scan_token
                    if [[ ${__z_token[kind]} != Comma ]]; then
                        __z_panic 128
                    fi

                    __z_scanner_scan_token
                    if [[ ${__z_token[kind]} != Number ]]; then
                        __z_panic 134
                    fi
                    local b=${__z_token[content]}

                    __z_scanner_scan_token
                    if [[ ${__z_token[kind]} != RightParen ]]; then
                        __z_panic 139
                    fi

                    __z_C_rgb "$r" "$g" "$b"
                    local rgb=$__z_return
                    __z_state_push "${__z_token[content]}" "$rgb"
                    $out "%b" "$rgb"
                    ;;
                *) __z_panic 147 ;;
                esac
                ;;
            *) __z_panic 150 ;;
            esac
            ;;
        X)
            __z_scanner_scan_token
            case ${__z_token[kind]} in
            Dot)
                case ${__z_token[kind]} in
                Black)
                    __z_state_push "x" "$__z_X_BLACK"
                    $out "%b" "$__z_X_BLACK"
                    ;;
                Blue)
                    __z_state_push "x" "$__z_X_BLUE"
                    $out "%b" "$__z_X_BLUE"
                    ;;
                Cyan)
                    __z_state_push "x" "$__z_X_CYAN"
                    $out "%b" "$__z_X_CYAN"
                    ;;
                Green)
                    __z_state_push "x" "$__z_X_GREEN"
                    $out "%b" "$__z_X_GREEN"
                    ;;
                Magenta)
                    __z_state_push "x" "$__z_X_MAGENTA"
                    $out "%b" "$__z_X_MAGENTA"
                    ;;
                Red)
                    __z_state_push "x" "$__z_X_RED"
                    $out "%b" "$__z_X_RED"
                    ;;
                White)
                    __z_state_push "x" "$__z_X_WHITE"
                    $out "%b" "$__z_X_WHITE"
                    ;;
                Yellow)
                    __z_state_push "x" "$__z_X_YELLOW"
                    $out "%b" "$__z_X_YELLOW"
                    ;;
                Rgb)
                    __z_scanner_scan_token
                    if [[ ${__z_token[kind]} != LeftParen ]]; then
                        __z_panic 193
                    fi

                    __z_scanner_scan_token
                    if [[ ${__z_token[kind]} != Number ]]; then
                        __z_panic 198
                    fi
                    local r=${__z_token[content]}

                    __z_scanner_scan_token
                    if [[ ${__z_token[kind]} != Comma ]]; then
                        __z_panic 204
                    fi

                    __z_scanner_scan_token
                    if [[ ${__z_token[kind]} != Number ]]; then
                        __z_panic 209
                    fi
                    local g=${__z_token[content]}

                    __z_scanner_scan_token
                    if [[ ${__z_token[kind]} != Comma ]]; then
                        __z_panic 216
                    fi

                    __z_scanner_scan_token
                    if [[ ${__z_token[kind]} != Number ]]; then
                        __z_panic 220
                    fi
                    local b=${__z_token[content]}

                    __z_scanner_scan_token
                    if [[ ${__z_token[kind]} != RightParen ]]; then
                        __z_panic 226
                    fi

                    __z_X_rgb "$r" "$g" "$b"
                    local rgb=$__z_return
                    __z_state_push "x" "$rgb"
                    $out "%b" "$rgb"
                    ;;
                *) __z_panic 234 ;;
                esac
                ;;
            *) __z_panic 237 ;;
            esac
            ;;
        Slash)
            __z_scanner_scan_token
            case ${__z_token[kind]} in
            B)
                if [[ ${__z_token[content]} != "b" ]]; then
                    __z_panic 245
                fi
                __z_state_pop
                $out "%b" "$__z_RESET_B"
                ;;
            I)
                if [[ ${__z_token[content]} != "i" ]]; then
                    __z_panic 252
                fi
                __z_state_pop
                $out "%b" "$__z_RESET_I"
                ;;
            S)
                if [[ ${__z_token[content]} != "s" ]]; then
                    __z_panic 259
                fi
                __z_state_pop
                $out "%b" "$__z_RESET_S"
                ;;
            U)
                if [[ ${__z_token[content]} != "u" ]]; then
                    __z_panic 266
                fi
                __z_state_pop
                $out "%b" "$__z_RESET_U"
                ;;
            C)
                if [[ ${__z_token[content]} != "c" ]]; then
                    __z_panic 273
                fi
                __z_state_pop
                __z_state_current_save
                local saved=$__z_return
                $out "%b" "$saved"
                ;;
            X)
                if [[ ${__z_token[content]} != "x" ]]; then
                    __z_panic 283
                fi
                __z_state_pop
                __z_state_current_save
                local saved=$__z_return
                $out "%b" "$saved"
                ;;
            Identifier | Black | Blue | Cyan | Green | Magenta | Red)
                __z_state_current_tag
                # shellcheck disable=SC2053
                if [[ $__z_return != ${__z_token[content]} ]]; then
                    __z_panic 293
                fi
                __z_state_pop
                __z_state_current_save
                local saved=$__z_return
                $out "%b" "$saved"
                ;;
            esac

            __z_scanner_scan_token
            if [[ ${__z_token[kind]} != CloseTag ]]; then
                __z_panic 304
            fi
            ;;
        Identifier | Black | Blue | Cyan | Green | Magenta | Red)
            local token_content=${__z_token[content]}
            local key="${__z_parser_variables}[$token_content]"
            local var=${!key}
            #if [[ -z $var ]]; then
                __z_state_push "${__z_token[content]}" "$var"
                $out "%b" "$var"
            #else
            #     __z_panic 315
            #fi
            ;;
        Eof)
            $out "%b" "$__z_RESET"
            break
            ;;
        *) continue ;;
        esac
    done
}
