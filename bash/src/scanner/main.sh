. token.sh

function __z_is_alpha {
    c=$1
    case $c in
        [abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_])
            __z_return=1 # true
        ;;
        *)
            __z_return=0 # false
        ;;
    esac
}

function __z_is_digit {
    c=$1
    case $c in
        [0123456789])
            __z_return=1 # true
        ;;
        *)
            __z_return=0 # false
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
    __z_scanner[text_mode]=1 # true
    __z_scanner[escape]=0
}

function __z_scanner_is_at_end {
    local current=${__z_scanner[current]}
    local src=${__z_scanner[src]}
    local len=${#src}
    if (( current + 1 > len )); then
        __z_return=1 # true
    else
        __z_return=0 # false
    fi
}

function __z_scanner_advance {
    local current=${__z_scanner[current]}
    local src=${__z_scanner[src]}

    __z_scanner[current]=$(( current + 1 ))
    __z_return=${src:$current:1}
}

function __z_scanner_peek {
    local current=${__z_scanner[current]}
    local src=${__z_scanner[src]}

    __z_scanner_is_at_end
    if (( __z_return == 0 )); then
        __z_return=${src:$current:1}
    else
        __z_return="\0"
    fi
}

function __z_scanner_peek_next {
    local current=${__z_scanner[current]}
    local src=${__z_scanner[src]}
    (( current ++ ))

    __z_scanner_is_at_end
    if (( __z_return == 0 )); then
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
    local len=$(( current - start ))

    s=${src:$start:$len}
    __z_token[kind]=$kind
    __z_token[content]=$s
    __z_token[err_code]=0
    __z_token[line]=${__z_scanner[line]}
}